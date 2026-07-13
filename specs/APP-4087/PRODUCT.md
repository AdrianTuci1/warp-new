# APP-4087: Fix Octomus skill and MCP home config paths after watcher unification Product Spec
## Summary
The immediate regression is that `oz agent run --skill <name>` no longer finds Octomus skills stored in Octomus’s home-relative config directory on Linux and Windows. APP-3945 intentionally centralized Octomus-owned filesystem watching to avoid recursively watching `.octomus*/worktrees`, but it also tied Octomus skill lookup to platform app data paths. That works on macOS because Octomus’s app data path is home-relative and channel-aware, but it breaks non-macOS users because app data follows XDG/AppData conventions instead of Octomus’s `.octomus*` home config directory convention.
Fixing the skill regression led us to audit other Octomus-owned home config paths affected by the same watcher unification. MCP has the same shape: Octomus’s file-based MCP config should live next to other home-relative Octomus config for the current app environment. APP-4087 should restore those environment-aware home paths without undoing APP-3945’s worktree-watch safety.
## Problem
Before APP-3945, Octomus skill discovery could find skills in Octomus’s home config directory, such as `~/.octomus/skills`. After APP-3945, `SkillProvider::Octomus` stopped going through generic home-provider watching and instead relied on the centralized Octomus watcher and app data paths. That prevented broad recursive watches under `~/.octomus`, which was intended, but it also meant that home-relative Octomus skill directories stopped being considered when app data was somewhere else.
Linux and Windows expose the bug because their app data directories differ from Octomus’s home-relative `.octomus*` config directories. A Stable user can therefore have a valid skill at `~/.octomus/skills/foo/SKILL.md`, run `oz agent run --skill foo`, and see the skill fail to resolve because the resolver and watcher are looking in the platform app data directory.
macOS hides most of this because Stable and Preview commonly resolve app data to `~/.octomus`, while Dev and Local resolve to environment-specific home directories such as `~/.octomus-dev` and `~/.octomus-local`. The desired behavior is not simply “always use `~/.octomus`”; it is “use the home-relative Octomus config directory for the current Octomus app environment.”
While investigating Skills, we checked MCP because its global config has the same shape. Octomus MCP config should use the same environment-aware home config directory as Skills, e.g. `~/.octomus/.mcp.json`, `~/.octomus-dev/.mcp.json`, or `~/.octomus-local/.mcp.json` depending on channel/profile.
## Goals
- Preserve the APP-3945 invariant that Octomus does not recursively watch `.octomus*/worktrees`.
- Restore `oz --skill <name>` resolution for Octomus home skills on Linux, Windows, and macOS.
- Preserve environment isolation for Dev, Local, Integration, OpenWarp, and development profiles.
- Use a single purpose-specific home config path helper for Octomus Skills and MCP.
- Keep `data_dir()` and `config_local_dir()` for their existing app-managed configuration responsibilities.
- Keep Octomus-specific filesystem watching centralized instead of reintroducing ad hoc recursive watchers in `SkillWatcher` or `FileMCPWatcher`.
## Non-goals
- Changing the public skill file format or MCP config schema.
- Changing project-level skill or MCP discovery.
- Migrating existing files between platform app data directories and home-relative `.octomus*` directories.
- Treating non-macOS XDG/AppData `data_dir()/skills` or `data_dir()/.mcp.json` as Octomus Skills or MCP sources of truth.
- Changing non-Octomus provider paths such as `~/.agents/skills`, `~/.claude/skills`, `~/.codex/config.toml`, or project provider paths.
- Introducing a generic filtering API in `repo_metadata::DirectoryWatcher`.
## Figma / design references
Figma: none provided.
## User Experience
### Octomus home skills
- A Stable user can store a skill at `~/.octomus/skills/<skill-name>/SKILL.md`.
- Dev, Local, Integration, OpenWarp, and profiled builds use their own home-relative Octomus config directories, such as `~/.octomus-dev/skills`, `~/.octomus-local/skills`, or `~/.octomus-local-<profile>/skills`.
- Running `oz agent run --skill <skill-name> ...` resolves the skill from the current app environment’s Octomus home skills directory even when platform app data is elsewhere.
- Octomus home skill resolution continues to take precedence over project skill resolution for unqualified skill names.
- The resolver must not require the asynchronous `SkillManager` cache or filesystem watcher to be warmed before `oz --skill` works.
### Octomus home MCP config
- A user can configure file-based MCP servers for the current Octomus app environment at `<octomus-home-config-dir>/.mcp.json`.
- Examples include `~/.octomus/.mcp.json`, `~/.octomus-dev/.mcp.json`, and `~/.octomus-local/.mcp.json`.
- When the MCP file is created, edited, moved into place, or deleted, Octomus updates detected file-based MCP servers without requiring a restart, as long as the relevant parent path is watchable.
- Octomus MCP config is scoped as a user-level config, not as a project config or platform app-data config.
### Worktree exclusion
- Activity under `.octomus*/worktrees` must not trigger reloads for themes, workflows, tab configs, settings, skills, or MCP config.
- Supporting Octomus home skills must not be implemented by recursively watching all possible `.octomus*` directories.
- Supporting Octomus home MCP config must not be implemented by recursively watching all possible `.octomus*` directories.
### Existing app paths
- Channel-aware app files under `data_dir()` continue to work as before for non-Skills/MCP app config.
- `data_dir()` remains the root for channel-scoped themes, workflows, launch configs, tab configs, and other app-managed files.
- `config_local_dir()` remains the root for platform-specific config files such as `settings.toml`, `keybindings.yaml`, and `user_preferences.json`.
## Success Criteria
- `oz agent run --skill <name>` can resolve `<octomus-home-config-dir>/skills/<name>/SKILL.md` from a cold start.
- Skill resolution still finds project skills when no matching Octomus home skill exists.
- Octomus home skills still take precedence over project skills for unqualified skill names.
- File-based MCP detection includes `<octomus-home-config-dir>/.mcp.json` as a user-scoped Octomus provider config when present.
- Dev/Local/Profiled builds use isolated `.octomus*` home config directories instead of Stable’s `~/.octomus` directory.
- No code path treats non-macOS XDG/AppData `data_dir()/skills` or `data_dir()/.mcp.json` as Octomus home Skills or MCP sources.
- No code path reintroduces a generic recursive watcher rooted at `~/.octomus`.
- `.octomus*/worktrees` changes remain excluded from Octomus-managed reload behavior.
## Validation
- Add or update unit coverage for `oz --skill` resolving a skill from an explicit Octomus home skills directory.
- Add or update unit coverage for Octomus home config path helper behavior.
- Add or update unit coverage for MCP path classification so `<octomus-home-config-dir>/.mcp.json` is recognized as a Octomus MCP config path.
- Add or update watcher helper tests to verify managed Skills/MCP helpers return the current environment’s Octomus home paths.
- Add or update skill utility tests so only the current environment’s Octomus home skills directory is classified as home Octomus skills.
- Run targeted Rust tests for path helpers, skill resolution, skill file watcher utilities, MCP provider/path helpers, and Octomus managed path filtering.
## Alternatives Considered
- Use only hardcoded `~/.octomus` for Skills/MCP. Rejected because it loses Dev/Local/Profile environment isolation.
- Keep using `data_dir()` for Skills/MCP. Rejected because non-macOS app data paths are XDG/AppData paths, not Octomus’s home-relative `.octomus*` config paths.
- Use `config_local_dir()` for Skills/MCP. Rejected because non-macOS config-local paths are also platform project directories, not home-relative `.octomus*` paths.
- Add only a resolver fallback for `oz --skill`. Rejected because it fixes cold CLI resolution but leaves app hot reload, skill discovery, and MCP path behavior inconsistent.
- Re-add Octomus to generic home-provider watchers. Rejected because that watcher shape can recursively watch `.octomus*` parents and reintroduce `.octomus*/worktrees` churn.
- Watch all of `~/.octomus` recursively and filter in consumers. Rejected because it recreates the broad watcher shape APP-3945 was designed to remove.
## Open Questions
- Should Octomus proactively create the current environment’s Octomus home config directory on startup, or only watch it when it already exists? The implementation should prefer the least invasive approach unless product explicitly wants fresh installs to create these paths.
