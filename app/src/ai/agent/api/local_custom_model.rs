use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use anyhow::{anyhow, Context as _};
use async_stream::stream;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use uuid::Uuid;
use warp_multi_agent_api as api;

use super::ResponseStream;
use crate::server::server_api::AIApiError;

const DIRECT_CUSTOM_MODEL_SYSTEM_PROMPT: &str = "You are Octomus, an AI assistant that can execute commands and perform actions in the terminal. You have access to the following tools:

1. RunShellCommand - Execute shell commands in the terminal. Use this when you need to run commands, check file contents, or perform operations that require shell access.
2. ReadFiles - Read file contents from the workspace.
3. SearchCodebase - Search through the codebase for specific patterns or files.
4. Grep - Search for text patterns in files.
5. RequestFileEdits - Request edits to files.

When you need to perform an action, emit a tool call in the following JSON format:
```tool_call
{\"tool\": \"RunShellCommand\", \"command\": \"your command here\", \"is_read_only\": false, \"is_risky\": false}
```

For ReadFiles:
```tool_call
{\"tool\": \"ReadFiles\", \"file_paths\": [\"path/to/file1\", \"path/to/file2\"]}
```

For SearchCodebase:
```tool_call
{\"tool\": \"SearchCodebase\", \"query\": \"search query\"}
```

For Grep:
```tool_call
{\"tool\": \"Grep\", \"pattern\": \"search pattern\", \"paths\": [\"optional/path\"]}
```

For RequestFileEdits:
```tool_call
{\"tool\": \"RequestFileEdits\", \"file_path\": \"path/to/file\", \"description\": \"description of changes\", \"content\": \"new content or diff\"}
```

Always prefer to use tools when appropriate. After executing a tool, wait for the result before proceeding. Reply directly to the user in plain text or markdown when no tool is needed.";
const CUSTOM_MODEL_REQUEST_TIMEOUT: Duration = Duration::from_secs(300);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SelectedCustomModel {
    base_url: String,
    api_key: String,
    model_slug: String,
    config_key: String,
}

#[derive(Debug)]
struct SuccessfulDirectResponse {
    selected_model: SelectedCustomModel,
    text: String,
    usage: Option<OpenAiChatCompletionsUsage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct OpenAiChatCompletionsRequest {
    model: String,
    messages: Vec<OpenAiChatMessage>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct OpenAiChatMessage {
    role: &'static str,
    content: String,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatCompletionsResponse {
    choices: Vec<OpenAiChatCompletionsChoice>,
    #[serde(default)]
    usage: Option<OpenAiChatCompletionsUsage>,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatCompletionsChoice {
    message: OpenAiChatCompletionsMessage,
}

#[derive(Debug, Deserialize)]
struct OpenAiChatCompletionsMessage {
    #[serde(default)]
    content: Value,
    #[serde(default)]
    refusal: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiChatCompletionsUsage {
    #[serde(default)]
    prompt_tokens: u32,
    #[serde(default)]
    completion_tokens: u32,
    #[serde(default)]
    total_tokens: u32,
}

pub(super) fn selected_custom_model_from_request(
    request: &api::Request,
) -> Option<SelectedCustomModel> {
    let settings = request.settings.as_ref()?;
    let model_config = settings.model_config.as_ref()?;
    let config_key = model_config.base.as_str();

    settings
        .custom_model_providers
        .as_ref()?
        .providers
        .iter()
        .find_map(|provider| {
            provider.models.iter().find_map(|model| {
                (model.config_key == config_key).then(|| SelectedCustomModel {
                    base_url: provider.base_url.clone(),
                    api_key: provider.api_key.clone(),
                    model_slug: model.slug.clone(),
                    config_key: model.config_key.clone(),
                })
            })
        })
}

pub(super) fn generate_multi_agent_output(request: api::Request) -> ResponseStream {
    let stream = stream! {
        match execute_chat_completion(&request).await {
            Ok(response) => {
                for event in build_success_events(&request, response) {
                    yield Ok(event);
                }
            }
            Err(err) => {
                yield Err(Arc::new(err));
            }
        }
    };
    Box::pin(stream)
}

async fn execute_chat_completion(
    request: &api::Request,
) -> Result<SuccessfulDirectResponse, AIApiError> {
    let selected_model = selected_custom_model_from_request(request).ok_or_else(|| {
        AIApiError::Other(anyhow!(
            "custom endpoint model selection could not be resolved from the request"
        ))
    })?;

    let messages = build_chat_messages(request)?;
    if messages.len() <= 1 {
        return Err(AIApiError::Other(anyhow!(
            "custom endpoint request did not produce any chat messages"
        )));
    }

    let url = chat_completions_url(&selected_model.base_url)?;
    let payload = OpenAiChatCompletionsRequest {
        model: selected_model.model_slug.clone(),
        messages,
    };

    let response = http_client::Client::new()
        .post(url)
        .bearer_auth(&selected_model.api_key)
        .timeout(CUSTOM_MODEL_REQUEST_TIMEOUT)
        .json(&payload)
        .send()
        .await
        .map_err(AIApiError::from)?
        .error_for_status_with_body()
        .await
        .map_err(AIApiError::from)?;

    let body: OpenAiChatCompletionsResponse = response.json().await.map_err(AIApiError::from)?;
    let text = extract_assistant_text(&body).ok_or_else(|| {
        AIApiError::Other(anyhow!(
            "custom endpoint returned no assistant text in the first choice"
        ))
    })?;

    Ok(SuccessfulDirectResponse {
        selected_model,
        text,
        usage: body.usage,
    })
}

fn build_success_events(
    request: &api::Request,
    response: SuccessfulDirectResponse,
) -> Vec<api::ResponseEvent> {
    let conversation_id = request
        .metadata
        .as_ref()
        .map(|metadata| metadata.conversation_id.clone())
        .filter(|id| !id.is_empty())
        .unwrap_or_else(|| Uuid::new_v4().to_string());
    let request_id = Uuid::new_v4().to_string();
    let run_id = request
        .metadata
        .as_ref()
        .map(|metadata| metadata.ambient_agent_task_id.clone())
        .filter(|id| !id.is_empty())
        .unwrap_or_else(|| conversation_id.clone());
    let root_task = resolve_root_task(request);
    
    // Parse tool calls from the response text
    let (clean_text, tool_calls) = parse_tool_calls_from_text(&response.text);
    
    let assistant_message =
        assistant_output_message(&root_task.task_id, &request_id, &clean_text);

    let mut actions = Vec::new();
    if let Some(task) = root_task.create_task {
        actions.push(api::ClientAction {
            action: Some(api::client_action::Action::CreateTask(
                api::client_action::CreateTask { task: Some(task) },
            )),
        });
    }
    
    // Add tool call messages if any were parsed
    let mut messages = vec![assistant_message];
    for tool_call in tool_calls {
        let tool_call_message = api::Message {
            id: Uuid::new_v4().to_string(),
            task_id: root_task.task_id.clone(),
            request_id: request_id.clone(),
            timestamp: Some(now_timestamp()),
            server_message_data: String::new(),
            citations: Vec::new(),
            message: Some(api::message::Message::ToolCall(tool_call)),
        };
        messages.push(tool_call_message);
    }
    
    actions.push(api::ClientAction {
        action: Some(api::client_action::Action::AddMessagesToTask(
            api::client_action::AddMessagesToTask {
                task_id: root_task.task_id.clone(),
                messages,
            },
        )),
    });

    let token_usage = response
        .usage
        .as_ref()
        .map(|usage| {
            vec![api::response_event::stream_finished::TokenUsage {
                model_id: response.selected_model.config_key.clone(),
                total_input: usage.prompt_tokens,
                output: usage.completion_tokens,
                input_cache_read: 0,
                input_cache_write: 0,
                cost_in_cents: 0.0,
            }]
        })
        .unwrap_or_default();
    let conversation_usage_metadata = response
        .usage
        .as_ref()
        .map(|usage| custom_endpoint_usage_metadata(&response.selected_model.config_key, usage));

    vec![
        api::ResponseEvent {
            r#type: Some(api::response_event::Type::Init(
                api::response_event::StreamInit {
                    conversation_id,
                    request_id: request_id.clone(),
                    run_id,
                },
            )),
        },
        api::ResponseEvent {
            r#type: Some(api::response_event::Type::ClientActions(
                api::response_event::ClientActions { actions },
            )),
        },
        api::ResponseEvent {
            r#type: Some(api::response_event::Type::Finished(
                api::response_event::StreamFinished {
                    token_usage,
                    should_refresh_model_config: false,
                    request_cost: None,
                    conversation_usage_metadata,
                    reason: Some(api::response_event::stream_finished::Reason::Done(
                        api::response_event::stream_finished::Done {},
                    )),
                },
            )),
        },
    ]
}

/// Parse tool calls from the assistant's text response.
/// Returns the cleaned text (with tool calls removed) and a list of parsed tool calls.
fn parse_tool_calls_from_text(text: &str) -> (String, Vec<api::message::ToolCall>) {
    let mut tool_calls = Vec::new();
    let mut clean_text = String::new();
    
    // Look for tool_call blocks in the format:
    // ```tool_call
    // {"tool": "RunShellCommand", ...}
    // ```
    let mut remaining = text;
    while let Some(start_idx) = remaining.find("```tool_call") {
        // Add text before the tool call to clean_text
        clean_text.push_str(&remaining[..start_idx]);
        
        // Find the end of the code block
        let after_start = &remaining[start_idx + "```tool_call".len()..];
        if let Some(end_idx) = after_start.find("```") {
            let json_str = after_start[..end_idx].trim();
            
            // Try to parse the JSON tool call
            if let Ok(tool_call) = parse_tool_call_json(json_str) {
                tool_calls.push(tool_call);
            }
            
            remaining = &after_start[end_idx + "```".len()..];
        } else {
            // No closing ```, add the rest and break
            clean_text.push_str(&remaining[start_idx..]);
            break;
        }
    }
    
    // Add any remaining text
    clean_text.push_str(remaining);
    
    (clean_text.trim().to_string(), tool_calls)
}

/// Parse a single tool call from JSON string.
fn parse_tool_call_json(json_str: &str) -> Result<api::message::ToolCall, anyhow::Error> {
    let value: Value = serde_json::from_str(json_str)
        .map_err(|e| anyhow!("Failed to parse tool call JSON: {}", e))?;
    
    let tool_name = value.get("tool")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("Tool call missing 'tool' field"))?;
    
    let tool_call_id = format!("tool_call_{}", Uuid::new_v4().to_string().replace("-", "")[..8].to_string());
    
    let tool = match tool_name {
        "RunShellCommand" => {
            let command = value.get("command")
                .and_then(Value::as_str)
                .unwrap_or("");
            let is_read_only = value.get("is_read_only")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            let is_risky = value.get("is_risky")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            
            api::message::tool_call::Tool::RunShellCommand(
                api::message::tool_call::RunShellCommand {
                    command: command.to_string(),
                    is_read_only,
                    is_risky,
                    uses_pager: false,
                    citations: vec![],
                    wait_until_complete_value: None,
                    risk_category: 0,
                }
            )
        }
        "ReadFiles" => {
            let files = value
                .get("files")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|file| {
                            let name = file.get("name").and_then(Value::as_str)?.to_string();
                            let line_ranges = file
                                .get("line_ranges")
                                .and_then(Value::as_array)
                                .map(|ranges| {
                                    ranges
                                        .iter()
                                        .filter_map(|range| {
                                            let start =
                                                range.get("start").and_then(Value::as_u64)? as u32;
                                            let end =
                                                range.get("end").and_then(Value::as_u64)? as u32;
                                            Some(api::FileContentLineRange { start, end })
                                        })
                                        .collect()
                                })
                                .unwrap_or_default();
                            Some(api::message::tool_call::read_files::File { name, line_ranges })
                        })
                        .collect()
                })
                .unwrap_or_default();

            api::message::tool_call::Tool::ReadFiles(api::message::tool_call::ReadFiles { files })
        }
        "SearchCodebase" => {
            let query = value.get("query")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            let path_filters = value.get("path_filters")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(Value::as_str)
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default();
            let codebase_path = value.get("codebase_path")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            
            api::message::tool_call::Tool::SearchCodebase(
                api::message::tool_call::SearchCodebase {
                    query,
                    path_filters,
                    codebase_path,
                }
            )
        }
        "Grep" => {
            let queries = value.get("queries")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(Value::as_str)
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_else(|| {
                    // Fallback: if "queries" is not provided, try "pattern"
                    value.get("pattern")
                        .and_then(Value::as_str)
                        .map(|p| vec![p.to_string()])
                        .unwrap_or_default()
                });
            let path = value.get("path")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            
            api::message::tool_call::Tool::Grep(
                api::message::tool_call::Grep {
                    queries,
                    path,
                }
            )
        }
        "RequestFileEdits" => {
            let summary = value
                .get("summary")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            let new_files = value
                .get("new_files")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|file| {
                            let file_path = file.get("file_path").and_then(Value::as_str)?.to_string();
                            let content = file.get("content").and_then(Value::as_str)?.to_string();
                            Some(api::message::tool_call::apply_file_diffs::NewFile {
                                file_path,
                                content,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            let diffs = value
                .get("diffs")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|diff| {
                            let file_path =
                                diff.get("file_path").and_then(Value::as_str)?.to_string();
                            let search =
                                diff.get("search").and_then(Value::as_str)?.to_string();
                            let replace = diff
                                .get("replace")
                                .and_then(Value::as_str)?
                                .to_string();
                            Some(api::message::tool_call::apply_file_diffs::FileDiff {
                                file_path,
                                search,
                                replace,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            let v4a_updates = value
                .get("v4a_updates")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|update| {
                            let file_path =
                                update.get("file_path").and_then(Value::as_str)?.to_string();
                            let move_to = update
                                .get("move_to")
                                .and_then(Value::as_str)
                                .unwrap_or("")
                                .to_string();
                            let hunks = update
                                .get("hunks")
                                .and_then(Value::as_array)
                                .map(|hunks_arr| {
                                    hunks_arr
                                        .iter()
                                        .filter_map(|hunk| {
                                            let change_context = hunk
                                                .get("change_context")
                                                .and_then(Value::as_array)
                                                .map(|ctx| {
                                                    ctx.iter()
                                                        .filter_map(Value::as_str)
                                                        .map(String::from)
                                                        .collect()
                                                })
                                                .unwrap_or_default();
                                            let pre_context = hunk
                                                .get("pre_context")
                                                .and_then(Value::as_str)
                                                .unwrap_or("")
                                                .to_string();
                                            let old = hunk
                                                .get("old")
                                                .and_then(Value::as_str)
                                                .unwrap_or("")
                                                .to_string();
                                            let new = hunk
                                                .get("new")
                                                .and_then(Value::as_str)
                                                .unwrap_or("")
                                                .to_string();
                                            let post_context = hunk
                                                .get("post_context")
                                                .and_then(Value::as_str)
                                                .unwrap_or("")
                                                .to_string();
                                            Some(api::message::tool_call::apply_file_diffs::v4a_file_update::Hunk {
                                                change_context,
                                                pre_context,
                                                old,
                                                new,
                                                post_context,
                                            })
                                        })
                                        .collect()
                                })
                                .unwrap_or_default();
                            Some(api::message::tool_call::apply_file_diffs::V4aFileUpdate {
                                file_path,
                                move_to,
                                hunks,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();
            let deleted_files = value
                .get("deleted_files")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|file| {
                            let file_path = file.get("file_path").and_then(Value::as_str)?.to_string();
                            Some(api::message::tool_call::apply_file_diffs::DeleteFile { file_path })
                        })
                        .collect()
                })
                .unwrap_or_default();

            api::message::tool_call::Tool::ApplyFileDiffs(api::message::tool_call::ApplyFileDiffs {
                summary,
                new_files,
                diffs,
                v4a_updates,
                deleted_files,
            })
        }
        _ => {
            return Err(anyhow!("Unknown tool: {}", tool_name));
        }
    };

    Ok(api::message::ToolCall {
        tool_call_id,
        tool: Some(tool),
    })
}

fn build_chat_messages(request: &api::Request) -> Result<Vec<OpenAiChatMessage>, AIApiError> {
    let mut messages = vec![OpenAiChatMessage {
        role: "system",
        content: DIRECT_CUSTOM_MODEL_SYSTEM_PROMPT.to_string(),
    }];

    if let Some(root_task) = select_root_task(request) {
        for message in &root_task.messages {
            if let Some(chat_message) = history_message_to_chat_message(message) {
                messages.push(chat_message);
            }
        }
    }

    let input = request.input.as_ref().ok_or_else(|| {
        AIApiError::Other(anyhow!("custom endpoint request is missing request.input"))
    })?;
    messages.extend(request_input_to_chat_messages(input));

    Ok(messages)
}

fn history_message_to_chat_message(message: &api::Message) -> Option<OpenAiChatMessage> {
    match message.message.as_ref()? {
        api::message::Message::UserQuery(user_query) => Some(OpenAiChatMessage {
            role: "user",
            content: render_user_query(
                &user_query.query,
                user_query.context.as_ref(),
                &user_query.referenced_attachments,
            ),
        }),
        api::message::Message::AgentOutput(output) if !output.text.trim().is_empty() => {
            Some(OpenAiChatMessage {
                role: "assistant",
                content: output.text.clone(),
            })
        }
        api::message::Message::ToolCallResult(tool_call_result) => {
            // Convert tool call result to a message that the model can understand
            let result_text = render_tool_call_result(tool_call_result);
            if result_text.is_empty() {
                None
            } else {
                Some(OpenAiChatMessage {
                    role: "user",  // Tool results are presented as user context
                    content: result_text,
                })
            }
        }
        _ => None,
    }
}

fn render_tool_call_result(tool_call_result: &api::message::ToolCallResult) -> String {
    use warp_multi_agent_api::message::tool_call_result::Result as ToolCallResultType;

    match tool_call_result.result.as_ref() {
        Some(ToolCallResultType::RunShellCommand(result)) => {
            match &result.result {
                Some(api::run_shell_command_result::Result::CommandFinished(finished)) => {
                    format!(
                        "Command `{}` completed with exit code {}.\nOutput:\n```\n{}\n```",
                        result.command,
                        finished.exit_code,
                        finished.output
                    )
                }
                Some(api::run_shell_command_result::Result::LongRunningCommandSnapshot(snapshot)) => {
                    format!(
                        "Long-running command `{}` snapshot:\n```\n{}\n```",
                        result.command,
                        snapshot.output
                    )
                }
                Some(api::run_shell_command_result::Result::PermissionDenied(_)) => {
                    format!(
                        "Command `{}` was denied permission to run.",
                        result.command
                    )
                }
                None => {
                    format!(
                        "Command `{}` was cancelled or did not produce a result.",
                        result.command
                    )
                }
            }
        }
        Some(ToolCallResultType::ReadFiles(result)) => {
            match &result.result {
                Some(api::read_files_result::Result::TextFilesSuccess(success)) => {
                    let mut output = String::from("File contents:\n\n");
                    for file in &success.files {
                        output.push_str(&format!(
                            "File: {}\n```\n{}\n```\n\n",
                            file.file_path,
                            file.content
                        ));
                    }
                    output
                }
                Some(api::read_files_result::Result::AnyFilesSuccess(success)) => {
                    let mut output = String::from("File contents (any type):\n\n");
                    for file in &success.files {
                        match &file.content {
                            Some(api::any_file_content::Content::BinaryContent(binary)) => {
                                output.push_str(&format!("Binary file: {}\n", binary.file_path));
                            }
                            Some(api::any_file_content::Content::TextContent(text)) => {
                                output.push_str(&format!("Text file: {}\n```\n{}\n```\n\n", text.file_path, text.content));
                            }
                            None => {
                                output.push_str("Unknown file content\n");
                            }
                        }
                    }
                    output
                }
                Some(api::read_files_result::Result::Error(error)) => {
                    format!("Error reading files: {}", error.message)
                }
                None => String::from("No file content available."),
            }
        }
        Some(ToolCallResultType::SearchCodebase(result)) => {
            match &result.result {
                Some(api::search_codebase_result::Result::Success(success)) => {
                    let mut output = String::from("Search results:\n\n");
                    for file in &success.files {
                        output.push_str(&format!(
                            "File: {}\n```\n{}\n```\n\n",
                            file.file_path,
                            file.content
                        ));
                    }
                    output
                }
                Some(api::search_codebase_result::Result::Error(error)) => {
                    format!("Error searching codebase: {}", error.message)
                }
                None => String::from("No search results available."),
            }
        }
        Some(ToolCallResultType::Grep(result)) => {
            match &result.result {
                Some(api::grep_result::Result::Success(success)) => {
                    let mut output = String::from("Grep results:\n\n");
                    for file in &success.matched_files {
                        output.push_str(&format!("File: {}\n", file.file_path));
                        for line in &file.matched_lines {
                            output.push_str(&format!("  Line {}\n", line.line_number));
                        }
                        output.push('\n');
                    }
                    output
                }
                Some(api::grep_result::Result::Error(error)) => {
                    format!("Error running grep: {}", error.message)
                }
                None => String::from("No grep results available."),
            }
        }
        Some(ToolCallResultType::ApplyFileDiffs(result)) => {
            match &result.result {
                Some(api::apply_file_diffs_result::Result::Success(success)) => {
                    let mut output = String::from("File edit results:\n\n");
                    if !success.updated_files_v2.is_empty() {
                        output.push_str("Updated files:\n");
                        for file in &success.updated_files_v2 {
                            if let Some(file_content) = &file.file {
                                output.push_str(&format!("- {}\n", file_content.file_path));
                            }
                        }
                    }
                    if !success.deleted_files.is_empty() {
                        output.push_str("Deleted files:\n");
                        for file in &success.deleted_files {
                            output.push_str(&format!("- {}\n", file.file_path));
                        }
                    }
                    output
                }
                Some(api::apply_file_diffs_result::Result::Error(error)) => {
                    format!("Error applying file diffs: {}", error.message)
                }
                None => String::from("No file edit results available."),
            }
        }
        _ => {
            // For other tool call results, return a generic message
            String::new()
        }
    }
}

fn request_input_to_chat_messages(input: &api::request::Input) -> Vec<OpenAiChatMessage> {
    let context = input.context.as_ref();
    match input.r#type.as_ref() {
        Some(api::request::input::Type::UserInputs(user_inputs)) => user_inputs
            .inputs
            .iter()
            .filter_map(|input| match input.input.as_ref()? {
                api::request::input::user_inputs::user_input::Input::UserQuery(user_query) => {
                    Some(OpenAiChatMessage {
                        role: "user",
                        content: render_user_query(
                            &user_query.query,
                            context,
                            &user_query.referenced_attachments,
                        ),
                    })
                }
                api::request::input::user_inputs::user_input::Input::CliAgentUserQuery(
                    cli_agent_user_query,
                ) => {
                    let mut content = cli_agent_user_query
                        .user_query
                        .as_ref()
                        .map(|user_query| {
                            render_user_query(
                                &user_query.query,
                                context,
                                &user_query.referenced_attachments,
                            )
                        })
                        .unwrap_or_default();
                    if let Some(running_command) = cli_agent_user_query.running_command.as_ref() {
                        push_section(
                            &mut content,
                            "Running command",
                            render_running_shell_command(running_command),
                        );
                    }
                    (!content.trim().is_empty()).then_some(OpenAiChatMessage {
                        role: "user",
                        content,
                    })
                }
                _ => None,
            })
            .collect(),
        Some(api::request::input::Type::QueryWithCannedResponse(query)) => {
            vec![OpenAiChatMessage {
                role: "user",
                content: render_text_request(&query.query, context),
            }]
        }
        Some(api::request::input::Type::AutoCodeDiffQuery(query)) => vec![OpenAiChatMessage {
            role: "user",
            content: render_text_request(&query.query, context),
        }],
        Some(api::request::input::Type::ResumeConversation(_)) => vec![OpenAiChatMessage {
            role: "user",
            content: render_text_request("Continue the conversation.", context),
        }],
        Some(api::request::input::Type::InitProjectRules(_)) => vec![OpenAiChatMessage {
            role: "user",
            content: render_text_request("Generate project rules for this project.", context),
        }],
        Some(api::request::input::Type::CreateNewProject(query)) => vec![OpenAiChatMessage {
            role: "user",
            content: render_text_request(&query.query, context),
        }],
        Some(api::request::input::Type::CloneRepository(query)) => vec![OpenAiChatMessage {
            role: "user",
            content: render_text_request(&format!("Clone repository {}", query.url), context),
        }],
        Some(api::request::input::Type::CreateEnvironment(request)) => vec![OpenAiChatMessage {
            role: "user",
            content: render_text_request(
                &format!(
                    "Create a development environment for these repositories:\n{}",
                    request.repo_paths.join("\n")
                ),
                context,
            ),
        }],
        Some(api::request::input::Type::SummarizeConversation(request)) => {
            let prompt = if request.prompt.trim().is_empty() {
                "Summarize the conversation.".to_string()
            } else {
                format!("Summarize the conversation. Focus on: {}", request.prompt)
            };
            vec![OpenAiChatMessage {
                role: "user",
                content: render_text_request(&prompt, context),
            }]
        }
        Some(api::request::input::Type::FetchReviewComments(request)) => vec![OpenAiChatMessage {
            role: "user",
            content: render_text_request(
                &format!(
                    "Fetch or address review comments for repository path {}.",
                    request.repo_path
                ),
                context,
            ),
        }],
        Some(api::request::input::Type::StartFromAmbientRunPrompt(request)) => {
            let prompt = if request.runtime_base_prompt.trim().is_empty() {
                format!(
                    "Continue the ambient run {} using the latest known prompt.",
                    request.ambient_run_id
                )
            } else {
                request.runtime_base_prompt.clone()
            };
            vec![OpenAiChatMessage {
                role: "user",
                content: render_text_request(&prompt, context),
            }]
        }
        Some(api::request::input::Type::InvokeSkill(request)) => {
            let mut content = request
                .user_query
                .as_ref()
                .map(|user_query| {
                    render_user_query(
                        &user_query.query,
                        context,
                        &user_query.referenced_attachments,
                    )
                })
                .unwrap_or_else(|| "Run the requested skill.".to_string());
            if let Some(skill) = request.skill.as_ref() {
                if let Some(descriptor) = skill.descriptor.as_ref() {
                    push_section(
                        &mut content,
                        "Skill",
                        format!("{}\n{}", descriptor.name, descriptor.description),
                    );
                }
                if let Some(file_content) = skill.content.as_ref() {
                    push_section(
                        &mut content,
                        "Skill instructions",
                        render_file_content(file_content),
                    );
                }
            }
            vec![OpenAiChatMessage {
                role: "user",
                content,
            }]
        }
        _ => Vec::new(),
    }
}

fn render_user_query(
    query: &str,
    context: Option<&api::InputContext>,
    referenced_attachments: &HashMap<String, api::Attachment>,
) -> String {
    let mut content = query.to_string();
    if let Some(context) = context {
        let rendered = render_input_context(context);
        if !rendered.is_empty() {
            push_section(&mut content, "Context", rendered);
        }
    }
    if !referenced_attachments.is_empty() {
        push_section(
            &mut content,
            "Referenced attachments",
            render_referenced_attachments(referenced_attachments),
        );
    }
    content
}

fn render_text_request(text: &str, context: Option<&api::InputContext>) -> String {
    let mut content = text.to_string();
    if let Some(context) = context {
        let rendered = render_input_context(context);
        if !rendered.is_empty() {
            push_section(&mut content, "Context", rendered);
        }
    }
    content
}

#[allow(deprecated)]
fn render_input_context(context: &api::InputContext) -> String {
    let mut sections = Vec::new();

    if let Some(directory) = context.directory.as_ref() {
        let mut lines = Vec::new();
        if !directory.pwd.is_empty() {
            lines.push(format!("pwd: {}", directory.pwd));
        }
        if !directory.home.is_empty() {
            lines.push(format!("home: {}", directory.home));
        }
        lines.push(format!(
            "file symbols indexed: {}",
            directory.pwd_file_symbols_indexed
        ));
        sections.push(format!("Directory\n{}", lines.join("\n")));
    }

    if let Some(shell) = context.shell.as_ref() {
        let mut lines = Vec::new();
        if !shell.name.is_empty() {
            lines.push(format!("name: {}", shell.name));
        }
        if !shell.version.is_empty() {
            lines.push(format!("version: {}", shell.version));
        }
        if !lines.is_empty() {
            sections.push(format!("Shell\n{}", lines.join("\n")));
        }
    }

    if let Some(os) = context.operating_system.as_ref() {
        let mut lines = Vec::new();
        if !os.platform.is_empty() {
            lines.push(format!("platform: {}", os.platform));
        }
        if !os.distribution.is_empty() {
            lines.push(format!("distribution: {}", os.distribution));
        }
        if !lines.is_empty() {
            sections.push(format!("Operating system\n{}", lines.join("\n")));
        }
    }

    if let Some(current_time) = context.current_time.as_ref() {
        sections.push(format!("Current time\n{}", current_time.seconds));
    }

    if !context.selected_text.is_empty() {
        let selected = context
            .selected_text
            .iter()
            .map(|selected| selected.text.as_str())
            .collect::<Vec<_>>()
            .join("\n---\n");
        if !selected.is_empty() {
            sections.push(format!("Selected text\n{}", selected));
        }
    }

    if !context.executed_shell_commands.is_empty() {
        let commands = context
            .executed_shell_commands
            .iter()
            .map(render_executed_shell_command)
            .collect::<Vec<_>>()
            .join("\n\n");
        if !commands.is_empty() {
            sections.push(format!("Executed commands\n{}", commands));
        }
    }

    if !context.files.is_empty() {
        let files = context
            .files
            .iter()
            .filter_map(|file| file.content.as_ref().map(render_file_content))
            .collect::<Vec<_>>()
            .join("\n\n");
        if !files.is_empty() {
            sections.push(format!("Files\n{}", files));
        }
    }

    if !context.codebases.is_empty() {
        let codebases = context
            .codebases
            .iter()
            .map(|codebase| format!("{} ({})", codebase.name, codebase.path))
            .collect::<Vec<_>>()
            .join("\n");
        if !codebases.is_empty() {
            sections.push(format!("Codebases\n{}", codebases));
        }
    }

    if !context.project_rules.is_empty() {
        let rules = context
            .project_rules
            .iter()
            .map(render_project_rules)
            .collect::<Vec<_>>()
            .join("\n\n");
        if !rules.is_empty() {
            sections.push(format!("Project rules\n{}", rules));
        }
    }

    if let Some(git) = context.git.as_ref() {
        let mut lines = Vec::new();
        if !git.head.is_empty() {
            lines.push(format!("head: {}", git.head));
        }
        if !git.branch.is_empty() {
            lines.push(format!("branch: {}", git.branch));
        }
        if let Some(repository) = git.repository.as_ref() {
            let owner_prefix = if repository.owner.is_empty() {
                String::new()
            } else {
                format!("{}/", repository.owner)
            };
            lines.push(format!("repository: {}{}", owner_prefix, repository.name));
        }
        if let Some(pull_request) = git.pull_request.as_ref() {
            lines.push(format!("pull request: #{}", pull_request.number));
            if !pull_request.base_branch.is_empty() {
                lines.push(format!("base branch: {}", pull_request.base_branch));
            }
        }
        if !lines.is_empty() {
            sections.push(format!("Git\n{}", lines.join("\n")));
        }
    }

    if !context.images.is_empty() {
        sections.push(format!(
            "Images\n{} image attachment(s)",
            context.images.len()
        ));
    }

    sections.join("\n\n")
}

fn render_project_rules(project_rules: &api::input_context::ProjectRules) -> String {
    let mut rendered = String::new();
    if !project_rules.root_path.is_empty() {
        rendered.push_str(&format!("root: {}", project_rules.root_path));
    }
    if !project_rules.active_rule_files.is_empty() {
        push_section(
            &mut rendered,
            "Active rules",
            project_rules
                .active_rule_files
                .iter()
                .map(render_file_content)
                .collect::<Vec<_>>()
                .join("\n\n"),
        );
    }
    if !project_rules.additional_rule_file_paths.is_empty() {
        push_section(
            &mut rendered,
            "Additional rules",
            project_rules.additional_rule_file_paths.join("\n"),
        );
    }
    rendered
}

fn render_referenced_attachments(
    referenced_attachments: &HashMap<String, api::Attachment>,
) -> String {
    let mut keys = referenced_attachments.keys().cloned().collect::<Vec<_>>();
    keys.sort();
    keys.into_iter()
        .filter_map(|key| {
            referenced_attachments
                .get(&key)
                .map(|attachment| format!("{key}\n{}", render_attachment(attachment)))
        })
        .collect::<Vec<_>>()
        .join("\n\n")
}

#[allow(deprecated)]
fn render_attachment(attachment: &api::Attachment) -> String {
    match attachment.value.as_ref() {
        Some(api::attachment::Value::PlainText(text)) => text.clone(),
        Some(api::attachment::Value::ExecutedShellCommand(command)) => {
            render_executed_shell_command(command)
        }
        Some(api::attachment::Value::RunningShellCommand(command)) => {
            render_running_shell_command(command)
        }
        Some(api::attachment::Value::DriveObject(object)) => render_drive_object(object),
        Some(api::attachment::Value::DiffHunk(hunk)) => render_diff_hunk(hunk),
        Some(api::attachment::Value::DiffSet(diff_set)) => render_diff_set(diff_set),
        Some(api::attachment::Value::DocumentContent(document)) => format!(
            "document_id: {}\n{}",
            document.document_id, document.content
        ),
        Some(api::attachment::Value::FilePathReference(file_ref)) => {
            format!("file path: {}", file_ref.file_path)
        }
        None => String::new(),
    }
}

fn render_executed_shell_command(command: &api::ExecutedShellCommand) -> String {
    format!(
        "$ {}\nexit code: {}\n{}",
        command.command, command.exit_code, command.output
    )
}

fn render_running_shell_command(command: &api::RunningShellCommand) -> String {
    let mut rendered = format!("$ {}", command.command);
    if let Some(snapshot) = command.snapshot.as_ref() {
        push_section(&mut rendered, "Snapshot", snapshot.output.clone());
    }
    rendered
}

fn render_drive_object(object: &api::DriveObject) -> String {
    match object.object_payload.as_ref() {
        Some(api::drive_object::ObjectPayload::Workflow(workflow)) => format!(
            "workflow: {}\n{}\ncommand: {}",
            workflow.name, workflow.description, workflow.command
        ),
        Some(api::drive_object::ObjectPayload::Notebook(notebook)) => {
            format!("notebook: {}\n{}", notebook.title, notebook.content)
        }
        Some(api::drive_object::ObjectPayload::GenericStringObject(object)) => {
            format!("{}\n{}", object.object_type, object.payload)
        }
        None => format!("drive object: {}", object.uid),
    }
}

fn render_diff_hunk(hunk: &api::DiffHunk) -> String {
    format!("{}\n{}", hunk.file_path, hunk.diff_content)
}

fn render_diff_set(diff_set: &api::DiffSet) -> String {
    diff_set
        .hunks
        .iter()
        .map(|hunk| format!("{}\n{}", hunk.file_path, hunk.diff_content))
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn render_file_content(content: &api::FileContent) -> String {
    format!("{}\n{}", content.file_path, content.content)
}

fn resolve_root_task(request: &api::Request) -> ResolvedRootTask {
    if let Some(root_task) = select_root_task(request) {
        return ResolvedRootTask {
            task_id: root_task.id.clone(),
            create_task: None,
        };
    }

    let task = api::Task {
        id: Uuid::new_v4().to_string(),
        description: "Conversation".to_string(),
        dependencies: None,
        messages: Vec::new(),
        summary: String::new(),
        server_data: String::new(),
    };

    ResolvedRootTask {
        task_id: task.id.clone(),
        create_task: Some(task),
    }
}

fn select_root_task(request: &api::Request) -> Option<&api::Task> {
    request
        .task_context
        .as_ref()?
        .tasks
        .iter()
        .find(|task| {
            task.dependencies
                .as_ref()
                .is_none_or(|deps| deps.parent_task_id.is_empty())
        })
        .or_else(|| request.task_context.as_ref()?.tasks.first())
}

fn assistant_output_message(task_id: &str, request_id: &str, text: &str) -> api::Message {
    api::Message {
        id: Uuid::new_v4().to_string(),
        task_id: task_id.to_string(),
        request_id: request_id.to_string(),
        timestamp: Some(now_timestamp()),
        server_message_data: String::new(),
        citations: Vec::new(),
        message: Some(api::message::Message::AgentOutput(
            api::message::AgentOutput {
                text: text.to_string(),
            },
        )),
    }
}

fn custom_endpoint_usage_metadata(
    config_key: &str,
    usage: &OpenAiChatCompletionsUsage,
) -> api::response_event::stream_finished::ConversationUsageMetadata {
    let total_tokens = if usage.total_tokens > 0 {
        usage.total_tokens
    } else {
        usage.prompt_tokens.saturating_add(usage.completion_tokens)
    };

    let model_usage = api::response_event::stream_finished::ModelTokenUsage {
        total_tokens,
        token_usage_by_category: HashMap::new(),
        ..Default::default()
    };

    api::response_event::stream_finished::ConversationUsageMetadata {
        context_window_usage: 0.0,
        summarized: false,
        credits_spent: 0.0,
        tool_usage_metadata: None,
        warp_token_usage: HashMap::new(),
        byok_token_usage: HashMap::new(),
        platform_credits_spent: 0.0,
        custom_endpoint_token_usage: HashMap::from([(config_key.to_string(), model_usage)]),
        ..Default::default()
    }
}

fn extract_assistant_text(response: &OpenAiChatCompletionsResponse) -> Option<String> {
    let first_choice = response.choices.first()?;
    let text = extract_content_text(&first_choice.message.content);
    if !text.trim().is_empty() {
        return Some(text);
    }

    first_choice
        .message
        .refusal
        .as_ref()
        .map(|refusal| refusal.trim().to_string())
        .filter(|refusal| !refusal.is_empty())
}

fn extract_content_text(content: &Value) -> String {
    match content {
        Value::String(text) => text.clone(),
        Value::Array(parts) => parts
            .iter()
            .filter_map(|part| match part {
                Value::String(text) => Some(text.clone()),
                Value::Object(map) => {
                    if let Some(Value::String(text)) = map.get("text") {
                        Some(text.clone())
                    } else if let Some(Value::Object(text_object)) = map.get("text") {
                        text_object
                            .get("value")
                            .and_then(Value::as_str)
                            .map(str::to_string)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(""),
        _ => String::new(),
    }
}

fn chat_completions_url(base_url: &str) -> Result<Url, AIApiError> {
    let trimmed = base_url.trim();
    let mut url = Url::parse(trimmed)
        .with_context(|| format!("failed to parse custom endpoint URL '{trimmed}'"))
        .map_err(AIApiError::Other)?;

    if url
        .path()
        .trim_end_matches('/')
        .ends_with("/chat/completions")
    {
        return Ok(url);
    }

    let path = url.path().trim_end_matches('/');
    if path.is_empty() {
        url.set_path("/chat/completions");
        return Ok(url);
    }

    url.set_path(&format!("{path}/chat/completions"));
    Ok(url)
}

fn push_section(target: &mut String, title: &str, body: String) {
    if body.trim().is_empty() {
        return;
    }
    if !target.is_empty() {
        target.push_str("\n\n");
    }
    target.push_str(title);
    target.push_str(":\n");
    target.push_str(body.trim());
}

fn now_timestamp() -> prost_types::Timestamp {
    let now = Utc::now();
    prost_types::Timestamp {
        seconds: now.timestamp(),
        nanos: now.timestamp_subsec_nanos() as i32,
    }
}

struct ResolvedRootTask {
    task_id: String,
    create_task: Option<api::Task>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn custom_provider(
        base_url: &str,
        api_key: &str,
        models: Vec<(&str, &str)>,
    ) -> api::request::settings::custom_model_providers::CustomModelProvider {
        api::request::settings::custom_model_providers::CustomModelProvider {
            base_url: base_url.to_string(),
            api_key: api_key.to_string(),
            models: models
                .into_iter()
                .map(|(slug, config_key)| {
                    api::request::settings::custom_model_providers::CustomModel {
                        slug: slug.to_string(),
                        config_key: config_key.to_string(),
                    }
                })
                .collect(),
        }
    }

    fn make_request(tasks: Vec<api::Task>, input: api::request::Input) -> api::Request {
        api::Request {
            task_context: Some(api::request::TaskContext { tasks }),
            input: Some(input),
            settings: Some(api::request::Settings {
                model_config: Some(api::request::settings::ModelConfig {
                    base: "cfg-1".to_string(),
                    ..Default::default()
                }),
                custom_model_providers: Some(api::request::settings::CustomModelProviders {
                    providers: vec![custom_provider(
                        "https://example.com/v1",
                        "test-key",
                        vec![("gpt-custom", "cfg-1")],
                    )],
                }),
                ..Default::default()
            }),
            metadata: Some(api::request::Metadata::default()),
            existing_suggestions: None,
            mcp_context: None,
        }
    }

    fn history_user_message(query: &str) -> api::Message {
        api::Message {
            id: Uuid::new_v4().to_string(),
            task_id: "task-1".to_string(),
            request_id: "request-1".to_string(),
            timestamp: None,
            server_message_data: String::new(),
            citations: Vec::new(),
            message: Some(api::message::Message::UserQuery(api::message::UserQuery {
                query: query.to_string(),
                context: None,
                referenced_attachments: HashMap::new(),
                mode: None,
                intended_agent: api::AgentType::Primary as i32,
            })),
        }
    }

    fn history_agent_message(text: &str) -> api::Message {
        api::Message {
            id: Uuid::new_v4().to_string(),
            task_id: "task-1".to_string(),
            request_id: "request-1".to_string(),
            timestamp: None,
            server_message_data: String::new(),
            citations: Vec::new(),
            message: Some(api::message::Message::AgentOutput(
                api::message::AgentOutput {
                    text: text.to_string(),
                },
            )),
        }
    }

    fn current_user_input(query: &str) -> api::request::Input {
        api::request::Input {
            context: None,
            r#type: Some(api::request::input::Type::UserInputs(
                api::request::input::UserInputs {
                    inputs: vec![api::request::input::user_inputs::UserInput {
                        input: Some(
                            api::request::input::user_inputs::user_input::Input::UserQuery(
                                api::request::input::UserQuery {
                                    query: query.to_string(),
                                    referenced_attachments: HashMap::new(),
                                    mode: None,
                                    intended_agent: api::AgentType::Primary as i32,
                                },
                            ),
                        ),
                    }],
                },
            )),
        }
    }

    #[test]
    fn selected_custom_model_matches_request_model_config_key() {
        let request = make_request(Vec::new(), current_user_input("hello"));

        let selected =
            selected_custom_model_from_request(&request).expect("custom model should resolve");

        assert_eq!(selected.base_url, "https://example.com/v1");
        assert_eq!(selected.api_key, "test-key");
        assert_eq!(selected.model_slug, "gpt-custom");
        assert_eq!(selected.config_key, "cfg-1");
    }

    #[test]
    fn build_chat_messages_includes_root_history_and_current_input() {
        let request = make_request(
            vec![api::Task {
                id: "task-1".to_string(),
                description: String::new(),
                dependencies: None,
                messages: vec![
                    history_user_message("First question"),
                    history_agent_message("First answer"),
                ],
                summary: String::new(),
                server_data: String::new(),
            }],
            current_user_input("Second question"),
        );

        let messages = build_chat_messages(&request).expect("chat messages should build");

        assert_eq!(messages[0].role, "system");
        assert_eq!(messages[1].role, "user");
        assert_eq!(messages[1].content, "First question");
        assert_eq!(messages[2].role, "assistant");
        assert_eq!(messages[2].content, "First answer");
        assert_eq!(messages[3].role, "user");
        assert_eq!(messages[3].content, "Second question");
    }

    #[test]
    fn build_success_events_creates_root_task_when_request_has_no_server_tasks() {
        let request = make_request(Vec::new(), current_user_input("hello"));
        let events = build_success_events(
            &request,
            SuccessfulDirectResponse {
                selected_model: selected_custom_model_from_request(&request)
                    .expect("selected model should resolve"),
                text: "custom model reply".to_string(),
                usage: Some(OpenAiChatCompletionsUsage {
                    prompt_tokens: 10,
                    completion_tokens: 5,
                    total_tokens: 15,
                }),
            },
        );

        let client_actions = match events[1].r#type.as_ref() {
            Some(api::response_event::Type::ClientActions(actions)) => actions,
            other => panic!("expected client actions, got {other:?}"),
        };

        assert_eq!(client_actions.actions.len(), 2);

        let created_task_id = match client_actions.actions[0].action.as_ref() {
            Some(api::client_action::Action::CreateTask(create_task)) => create_task
                .task
                .as_ref()
                .expect("task should be present")
                .id
                .clone(),
            other => panic!("expected create_task action, got {other:?}"),
        };

        match client_actions.actions[1].action.as_ref() {
            Some(api::client_action::Action::AddMessagesToTask(add_messages)) => {
                assert_eq!(add_messages.task_id, created_task_id);
                assert_eq!(add_messages.messages.len(), 1);
            }
            other => panic!("expected add_messages action, got {other:?}"),
        }

        let finished = match events[2].r#type.as_ref() {
            Some(api::response_event::Type::Finished(finished)) => finished,
            other => panic!("expected finished event, got {other:?}"),
        };

        let usage_metadata = finished
            .conversation_usage_metadata
            .as_ref()
            .expect("usage metadata should be present");
        assert_eq!(
            usage_metadata
                .custom_endpoint_token_usage
                .get("cfg-1")
                .expect("custom endpoint usage should be present")
                .total_tokens,
            15
        );
    }

    #[test]
    fn parse_tool_calls_from_text_extracts_run_shell_command() {
        let text = r#"I will run the ls command for you.

```tool_call
{"tool": "RunShellCommand", "command": "ls -la", "is_read_only": true, "is_risky": false}
```

Done!"#;

        let (clean_text, tool_calls) = parse_tool_calls_from_text(text);

        assert_eq!(clean_text, "I will run the ls command for you.\n\nDone!");
        assert_eq!(tool_calls.len(), 1);

        let tool_call = &tool_calls[0];
        assert!(tool_call.tool_call_id.starts_with("tool_call_"));

        match tool_call.tool.as_ref() {
            Some(api::message::tool_call::Tool::RunShellCommand(cmd)) => {
                assert_eq!(cmd.command, "ls -la");
                assert_eq!(cmd.is_read_only, true);
                assert_eq!(cmd.is_risky, false);
            }
            other => panic!("expected RunShellCommand, got {other:?}"),
        }
    }

    #[test]
    fn parse_tool_calls_from_text_extracts_multiple_tools() {
        let text = r#"Let me help you with that.

```tool_call
{"tool": "ReadFiles", "files": [{"name": "test.txt", "line_ranges": [{"start": 1, "end": 10}]}]}
```

Now let me search:

```tool_call
{"tool": "SearchCodebase", "query": "function main", "path_filters": ["src/"], "codebase_path": "/project"}
```

All done!"#;

        let (clean_text, tool_calls) = parse_tool_calls_from_text(text);

        assert_eq!(tool_calls.len(), 2);
        assert!(clean_text.contains("Let me help you with that."));
        assert!(clean_text.contains("Now let me search:"));
        assert!(clean_text.contains("All done!"));

        // First tool call should be ReadFiles
        match tool_calls[0].tool.as_ref() {
            Some(api::message::tool_call::Tool::ReadFiles(rf)) => {
                assert_eq!(rf.files.len(), 1);
                assert_eq!(rf.files[0].name, "test.txt");
                assert_eq!(rf.files[0].line_ranges.len(), 1);
                assert_eq!(rf.files[0].line_ranges[0].start, 1);
                assert_eq!(rf.files[0].line_ranges[0].end, 10);
            }
            other => panic!("expected ReadFiles, got {other:?}"),
        }

        // Second tool call should be SearchCodebase
        match tool_calls[1].tool.as_ref() {
            Some(api::message::tool_call::Tool::SearchCodebase(sc)) => {
                assert_eq!(sc.query, "function main");
                assert_eq!(sc.path_filters, vec!["src/"]);
                assert_eq!(sc.codebase_path, "/project");
            }
            other => panic!("expected SearchCodebase, got {other:?}"),
        }
    }

    #[test]
    fn parse_tool_calls_from_text_handles_no_tool_calls() {
        let text = "This is just a regular response with no tools.";

        let (clean_text, tool_calls) = parse_tool_calls_from_text(text);

        assert_eq!(clean_text, "This is just a regular response with no tools.");
        assert_eq!(tool_calls.len(), 0);
    }

    #[test]
    fn parse_tool_calls_from_text_handles_invalid_json() {
        let text = r#"Here is a bad tool call:

```tool_call
this is not json
```

But the rest is fine."#;

        let (clean_text, tool_calls) = parse_tool_calls_from_text(text);

        // Invalid JSON should be skipped but text preserved
        assert!(clean_text.contains("Here is a bad tool call:"));
        assert!(clean_text.contains("But the rest is fine."));
        assert_eq!(tool_calls.len(), 0);
    }

    #[test]
    fn build_success_events_includes_tool_calls() {
        let request = make_request(Vec::new(), current_user_input("run ls"));
        let events = build_success_events(
            &request,
            SuccessfulDirectResponse {
                selected_model: selected_custom_model_from_request(&request)
                    .expect("selected model should resolve"),
                text: r#"I will run the ls command for you.

```tool_call
{"tool": "RunShellCommand", "command": "ls -la", "is_read_only": true, "is_risky": false}
```

Done!"#
                .to_string(),
                usage: Some(OpenAiChatCompletionsUsage {
                    prompt_tokens: 10,
                    completion_tokens: 5,
                    total_tokens: 15,
                }),
            },
        );

        let client_actions = match events[1].r#type.as_ref() {
            Some(api::response_event::Type::ClientActions(actions)) => actions,
            other => panic!("expected client actions, got {other:?}"),
        };

        // Should have create_task + add_messages (with 2 messages: assistant + tool_call)
        assert_eq!(client_actions.actions.len(), 2);

        let add_messages = match client_actions.actions[1].action.as_ref() {
            Some(api::client_action::Action::AddMessagesToTask(add_messages)) => add_messages,
            other => panic!("expected add_messages action, got {other:?}"),
        };

        // Should have 2 messages: assistant output + tool call
        assert_eq!(add_messages.messages.len(), 2);

        // First message should be assistant output
        match add_messages.messages[0].message.as_ref() {
            Some(api::message::Message::AgentOutput(output)) => {
                assert!(output.text.contains("I will run the ls command"));
                // Should not contain the tool_call block
                assert!(!output.text.contains("```tool_call"));
            }
            other => panic!("expected agent output, got {other:?}"),
        }

        // Second message should be tool call
        match add_messages.messages[1].message.as_ref() {
            Some(api::message::Message::ToolCall(tool_call)) => {
                match tool_call.tool.as_ref() {
                    Some(api::message::tool_call::Tool::RunShellCommand(cmd)) => {
                        assert_eq!(cmd.command, "ls -la");
                    }
                    other => panic!("expected RunShellCommand, got {other:?}"),
                }
            }
            other => panic!("expected tool call message, got {other:?}"),
        }
    }
}
