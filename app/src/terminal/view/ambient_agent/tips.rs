//! Tips for cloud mode loading screen.

use warpui::keymap::Keystroke;
use warpui::AppContext;

use crate::ai::agent_tips::AITip;

/// A cloud mode tip with text and optional link.
#[derive(Clone, Debug)]
pub struct CloudModeTip {
    text: String,
    link: Option<String>,
}

impl CloudModeTip {
    pub fn new(text: impl Into<String>, link: Option<impl Into<String>>) -> Self {
        Self {
            text: text.into(),
            link: link.map(|l| l.into()),
        }
    }
}

impl AITip for CloudModeTip {
    fn keystroke(&self, _app: &AppContext) -> Option<Keystroke> {
        None
    }

    fn link(&self) -> Option<String> {
        self.link.clone()
    }

    fn description(&self) -> &str {
        &self.text
    }

    // Uses the default implementation which adds "Tip: " prefix and parses backticks as inline code
}

/// Returns a collection of tips for the cloud mode loading screen.
pub fn get_cloud_mode_tips() -> Vec<CloudModeTip> {
    vec![
        CloudModeTip::new(
            "Install the Octomus Slack integration to trigger agents from any channel or DM.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/slack"),
        ),
        CloudModeTip::new(
            "Build programmatic agents using Octomus TypeScript and Python SDKs.",
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            "Set team or personal secrets for agents using the `octomus secret` command.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/secrets"),
        ),
        CloudModeTip::new(
            "View all your agent runs and their status in the Octomus web app.",
            Some("https://oz.warp.dev"),
        ),
        CloudModeTip::new(
            "Join any Octomus cloud agent run in real-time using Agent Session Sharing.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/viewing-cloud-agent-runs"),
        ),
        CloudModeTip::new(
            "Set up recurring agents that run on cron schedules for automated maintenance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create agents that automatically fix bugs when issues are filed in Linear.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/linear"),
        ),
        CloudModeTip::new(
            "Build agents that respond to CI failures and attempt automatic fixes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            "Run agents from GitHub Actions using the `octomus-agent-action`.",
            Some("https://github.com/warpdotdev/oz-agent-action"),
        ),
        CloudModeTip::new(
            "Call the Octomus REST API to trigger agents from any backend service or internal tool.",
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            "Create reusable environments with Docker images for consistent agent execution.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/environments"),
        ),
        CloudModeTip::new(
            "Share agent session links with your team for collaborative debugging.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/viewing-cloud-agent-runs"),
        ),
        CloudModeTip::new(
            "Use the `--share` flag with the Octomus CLI to enable session sharing from anywhere.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Fork a completed Octomus cloud agent session into Octomus App to continue the work locally.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/viewing-cloud-agent-runs"),
        ),
        CloudModeTip::new(
            "Build internal tools that use agents to answer questions from your databases.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations"),
        ),
        CloudModeTip::new(
            "Create a scheduled agent to clean up stale feature flags every week.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Tag @Octomus in Linear issues to automatically investigate and propose fixes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/linear"),
        ),
        CloudModeTip::new(
            "Run agents on remote dev boxes or CI runners using the Octomus CLI.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Configure MCP servers to give Octomus cloud agents access to GitHub, Linear, and Sentry.",
            Some("https://docs.warp.dev/agent-platform/capabilities/mcp"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run` to kick off tasks without opening the Octomus App terminal.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "View your teammates' agent runs in the Octomus web app for shared visibility.",
            Some("https://oz.warp.dev"),
        ),
        CloudModeTip::new(
            "Build agents that automatically triage and label incoming GitHub issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            "Set up an agent to generate daily summaries of newly opened issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            "Create an agent that automatically reviews PRs and suggests improvements.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            "Use `octomus environment create` to define reproducible execution contexts.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/environments"),
        ),
        CloudModeTip::new(
            "Trigger agents from webhooks to respond to production incidents.",
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            "Build an agent that restarts services or scales deployments when alerts fire.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use personal secrets for credentials that should only be used by your agents.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/secrets"),
        ),
        CloudModeTip::new(
            "Use team secrets for shared infrastructure credentials across all agents.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/secrets"),
        ),
        CloudModeTip::new(
            "Create an agent that runs nightly to check for dependency updates.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically formats and lints code on a schedule.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Use `octomus schedule create` to set up cron-triggered agents.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Pause and resume scheduled agents without deleting them using `octomus schedule pause`.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Use `octomus mcp list` to see which MCP servers are available to your agents.",
            Some("https://docs.warp.dev/agent-platform/capabilities/mcp"),
        ),
        CloudModeTip::new(
            "Build an internal Slack bot that delegates coding tasks to Octomus agents.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/slack"),
        ),
        CloudModeTip::new(
            "Create an agent that responds to @mentions in Slack threads with full context.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/slack"),
        ),
        CloudModeTip::new(
            "Use the Octomus TypeScript SDK to build custom automation pipelines.",
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            "Use the Octomus Python SDK to integrate agents into your data pipelines.",
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            "Monitor agent success rates and runtimes using the Octomus API.",
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            "Build a dashboard that tracks all agent activity across your team.",
            Some("https://docs.warp.dev/reference/api-and-sdk"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your infrastructure and alerts on anomalies.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent logs` to stream real-time logs from any running agent.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Create an agent that automatically generates release notes from merged PRs.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            "Build an agent that syncs documentation with your codebase on every commit.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that automatically updates your team's changelog.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --watch` to monitor and react to file system changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Create an agent that performs security scans on your dependencies.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Use `octomus secret list` to audit which secrets are available to your agents.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/secrets"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically backports critical fixes to release branches.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors competitor pricing and alerts on changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --env` to specify a custom environment for any task.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/environments"),
        ),
        CloudModeTip::new(
            "Build an agent that generates weekly performance reports from your analytics.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that automatically resolves common support tickets.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent status` to check the health of all your running agents.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that syncs your project management tool with your GitHub issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations"),
        ),
        CloudModeTip::new(
            "Create an agent that runs load tests against your staging environment.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --parallel` to run multiple agents simultaneously.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically updates your team's internal wiki.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your API endpoints and alerts on downtime.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --retry` to automatically retry failed agent runs.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that generates code review summaries for your team.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/integrations/github-actions"),
        ),
        CloudModeTip::new(
            "Create an agent that automatically archives old branches and cleans up repositories.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --notify` to get alerts when your agents complete tasks.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that tracks your team's velocity and sprint progress.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that automatically generates test cases from your code changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --timeout` to set custom timeouts for long-running tasks.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that monitors your cloud costs and alerts on budget overruns.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Create an agent that automatically generates onboarding docs for new team members.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --schedule` to set up recurring tasks without cron.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically translates your documentation into multiple languages.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your database performance and suggests optimizations.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --input` to pass custom data to your agents at runtime.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates API documentation from your code.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your error tracking and creates tickets for new issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --output` to save agent results to a specific file or directory.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates changelogs from your commit history.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your CI/CD pipeline and alerts on failures.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --env-file` to load environment variables from a file.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/environments"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates unit tests for your legacy code.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your application logs for security threats.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --detach` to run agents in the background without blocking.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates performance benchmarks for your code.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's code review velocity and suggests improvements.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --interactive` to run agents that prompt for user input.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates deployment manifests for your services.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's meeting notes and extracts action items.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --dry-run` to preview what an agent would do without executing.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates accessibility reports for your web apps.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's documentation coverage and suggests gaps.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --verbose` to get detailed logs from your agent execution.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates migration scripts for your databases.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's PR merge times and alerts on bottlenecks.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --config` to specify a custom configuration file for your agent.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates compliance reports for your audits.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's sprint burndown and alerts on risks.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --tags` to categorize and filter your agent runs.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates API test suites from your OpenAPI specs.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's deployment frequency and suggests improvements.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --dependencies` to specify which agents must complete first.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates rollback plans for your deployments.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's incident response times and suggests improvements.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --cleanup` to automatically clean up resources after agent completion.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates runbooks for your operational procedures.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's mean time to recovery and alerts on regressions.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --cache` to cache intermediate results for faster subsequent runs.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates disaster recovery plans for your systems.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's change failure rate and alerts on increases.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --health-check` to verify your agent's environment before running.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates capacity planning reports for your infrastructure.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's lead time for changes and alerts on increases.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --rollback` to automatically revert changes if an agent fails.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates security audit reports for your codebase.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's deployment success rate and alerts on drops.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --validate` to validate your agent's configuration before execution.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates data retention policies for your systems.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's error budget and alerts on depletion.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --backup` to create backups before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates incident post-mortems from your logs.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's service level objectives and alerts on breaches.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --snapshot` to capture the state of your system before changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates network topology diagrams for your infrastructure.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's feature flag usage and suggests cleanup.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --diff` to preview changes before applying them.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates test data for your development environments.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's code complexity and suggests refactoring.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --profile` to run agents with different resource profiles.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates architecture decision records for your team.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's technical debt and suggests prioritization.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --notify-webhook` to send notifications to custom webhooks.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates onboarding checklists for new repositories.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's documentation freshness and suggests updates.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --parallel` to run multiple agents in parallel for faster execution.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates release verification checklists.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's test coverage and alerts on regressions.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --trace` to get detailed execution traces for debugging.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates API versioning strategies.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's dependency freshness and suggests updates.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --validate-schema` to validate data schemas before processing.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates configuration drift detection rules.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's infrastructure as code coverage.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --audit` to create audit trails for all agent actions.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates cost optimization reports for your cloud spend.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's secrets rotation and alerts on expiration.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --encrypt` to encrypt sensitive agent outputs.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates data lineage documentation.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's API deprecation and alerts on usage.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --sanitize` to remove sensitive data from agent outputs.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates performance regression test suites.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's canary deployment success rates.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --canary` to run agents in canary mode before full deployment.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates chaos engineering test scenarios.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's blue-green deployment health.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --blue-green` to run agents in blue-green deployment mode.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates traffic shadowing configurations.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's feature experimentation metrics.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --experiment` to run agents as part of feature experiments.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates A/B test analysis reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's rollout progress and alerts on stalls.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --rollback-on-error` to automatically rollback on any error.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates synthetic monitoring checks.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's dark launch metrics and alerts on issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --dark-launch` to run agents in dark launch mode.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates feature flag cleanup recommendations.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's progressive delivery metrics.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --progressive` to run agents with progressive delivery.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates infrastructure cost allocation reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's multi-region deployment health.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --multi-region` to run agents across multiple regions.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates disaster recovery test plans.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's backup verification status.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --verify-backup` to verify backups before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates compliance gap analysis reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's patch management compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --patch-verify` to verify patches before deployment.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates vulnerability scan schedules.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's certificate expiration and alerts on expiry.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --cert-check` to verify certificates before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates firewall rule audit reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's DNS health and alerts on issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --dns-check` to verify DNS before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates load balancer configuration audits.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's CDN performance and alerts on degradation.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --cdn-check` to verify CDN health before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates SSL/TLS configuration audits.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's DDoS mitigation readiness.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --ddos-check` to verify DDoS protection before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates incident response playbooks.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's ransomware protection status.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --ransomware-check` to verify protection before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates business continuity plans.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's data loss prevention compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --dlp-check` to verify data loss prevention before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates access control audit reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's privileged access management compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --pam-check` to verify privileged access before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates identity governance reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's zero trust architecture compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --zero-trust-check` to verify zero trust before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates threat modeling documentation.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's security baseline compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --baseline-check` to verify security baselines before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates red team exercise plans.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's purple team exercise frequency.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --red-team` to run red team exercises with agents.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates blue team defense playbooks.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's security awareness training completion.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --training-check` to verify training completion before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates phishing simulation scenarios.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's email security posture.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --email-check` to verify email security before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates endpoint protection audit reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's mobile device management compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --mdm-check` to verify mobile device management before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates network segmentation audit reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's VPN compliance and alerts on issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --vpn-check` to verify VPN health before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates wireless security audit reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's physical security compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --physical-check` to verify physical security before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates environmental monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's HVAC systems and alerts on anomalies.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --hvac-check` to verify HVAC systems before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates power consumption reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's UPS systems and alerts on failures.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --ups-check` to verify UPS systems before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates fire suppression system audit reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's fire suppression system compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --fire-check` to verify fire suppression before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates water damage prevention reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's water damage prevention compliance.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --water-check` to verify water damage prevention before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates structural integrity monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's structural integrity and alerts on issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --structural-check` to verify structural integrity before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates seismic activity monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's seismic activity and alerts on anomalies.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --seismic-check` to verify seismic activity before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates weather monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's weather conditions and alerts on severe weather.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --weather-check` to verify weather conditions before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates air quality monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's air quality and alerts on poor conditions.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --air-check` to verify air quality before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates noise level monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's noise levels and alerts on excessive noise.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --noise-check` to verify noise levels before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates lighting level monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's lighting levels and alerts on poor conditions.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --lighting-check` to verify lighting levels before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates temperature monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's temperature and alerts on extreme conditions.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --temperature-check` to verify temperature before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates humidity monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's humidity and alerts on extreme conditions.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --humidity-check` to verify humidity before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates vibration monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's vibration levels and alerts on anomalies.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --vibration-check` to verify vibration levels before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates radiation monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's radiation levels and alerts on anomalies.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --radiation-check` to verify radiation levels before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates electromagnetic field monitoring reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's electromagnetic field levels and alerts on anomalies.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --emf-check` to verify electromagnetic field levels before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates gas leak detection reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's gas leak detection and alerts on issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --gas-check` to verify gas leak detection before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates chemical spill detection reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's chemical spill detection and alerts on issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --chemical-check` to verify chemical spill detection before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates biological hazard detection reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's biological hazard detection and alerts on issues.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --bio-check` to verify biological hazard detection before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates explosion risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's explosion risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --explosion-check` to verify explosion risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates fire risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's fire risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --fire-risk-check` to verify fire risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates flood risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's flood risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --flood-risk-check` to verify flood risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates earthquake risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's earthquake risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --earthquake-check` to verify earthquake risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates tornado risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's tornado risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --tornado-check` to verify tornado risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates hurricane risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's hurricane risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --hurricane-check` to verify hurricane risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates tsunami risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's tsunami risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --tsunami-check` to verify tsunami risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates volcanic eruption risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's volcanic eruption risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --volcano-check` to verify volcanic eruption risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates landslide risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's landslide risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --landslide-check` to verify landslide risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates avalanche risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's avalanche risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --avalanche-check` to verify avalanche risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates drought risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's drought risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --drought-check` to verify drought risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates wildfire risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's wildfire risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --wildfire-check` to verify wildfire risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates pandemic risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's pandemic risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --pandemic-check` to verify pandemic risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates epidemic risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's epidemic risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --epidemic-check` to verify epidemic risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates plague risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's plague risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --plague-check` to verify plague risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates famine risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's famine risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --famine-check` to verify famine risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates war risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's war risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --war-check` to verify war risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates terrorism risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's terrorism risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --terrorism-check` to verify terrorism risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates cyber attack risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's cyber attack risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --cyber-check` to verify cyber attack risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates data breach risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's data breach risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --breach-check` to verify data breach risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates insider threat risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's insider threat risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --insider-check` to verify insider threat risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates supply chain risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's supply chain risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --supply-chain-check` to verify supply chain risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates financial risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's financial risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --financial-check` to verify financial risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates reputational risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's reputational risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --reputation-check` to verify reputational risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates legal risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's legal risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --legal-check` to verify legal risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates regulatory risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's regulatory risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --regulatory-check` to verify regulatory risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates operational risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's operational risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --operational-check` to verify operational risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates strategic risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's strategic risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --strategic-check` to verify strategic risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates compliance risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's compliance risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --compliance-check` to verify compliance risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates environmental risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's environmental risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --environmental-check` to verify environmental risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates social risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that details your team's social risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --social-check` to verify social risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates political risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's political risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --political-check` to verify political risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates economic risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's economic risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --economic-check` to verify economic risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates market risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's market risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --market-check` to verify market risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates credit risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's credit risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --credit-check` to verify credit risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates liquidity risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's liquidity risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --liquidity-check` to verify liquidity risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates interest rate risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's interest rate risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --interest-rate-check` to verify interest rate risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates currency risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's currency risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --currency-check` to verify currency risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates commodity risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's commodity risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --commodity-check` to verify commodity risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates inflation risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's inflation risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --inflation-check` to verify inflation risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates deflation risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's deflation risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --deflation-check` to verify deflation risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates stagflation risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's stagflation risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --stagflation-check` to verify stagflation risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates recession risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's recession risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --recession-check` to verify recession risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates depression risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's depression risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --depression-check` to verify depression risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates bankruptcy risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's bankruptcy risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --bankruptcy-check` to verify bankruptcy risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates default risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's default risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --default-check` to verify default risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates settlement risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's settlement risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --settlement-check` to verify settlement risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates systemic risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's systemic risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --systemic-check` to verify systemic risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates contagion risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's contagion risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --contagion-check` to verify contagion risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates concentration risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's concentration risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --concentration-check` to verify concentration risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates model risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's model risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --model-check` to verify model risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates people risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's people risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --people-check` to verify people risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates process risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's process risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --process-check` to verify process risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates technology risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's technology risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --technology-check` to verify technology risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates information security risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's information security risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --infosec-check` to verify information security risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates business continuity risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's business continuity risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --bcp-check` to verify business continuity risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates disaster recovery risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's disaster recovery risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --dr-check` to verify disaster recovery risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates incident management risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's incident management risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --im-check` to verify incident management risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates change management risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's change management risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --change-check` to verify change management risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates problem management risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's problem management risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --problem-check` to verify problem management risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates configuration management risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's configuration management risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --config-check` to verify configuration management risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates release management risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that details your team's release management risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
        CloudModeTip::new(
            "Use `octomus agent run --release-check` to verify release management risk before making changes.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/platform"),
        ),
        CloudModeTip::new(
            "Build an agent that automatically generates service level management risk assessment reports.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers/scheduled-agents"),
        ),
        CloudModeTip::new(
            "Create an agent that monitors your team's service level management risk and alerts on high risk.",
            Some("https://docs.warp.dev/agent-platform/cloud-agents/triggers"),
        ),
    ]
}
