# APP-3945: Channel-aware Octomus home watching Product Spec

## Summary
Octomus should hot-reload the current channel's Octomus-managed files without reacting to unrelated files under `.octomus*/worktrees`. This includes continuing to reload `settings.toml` correctly on platforms where settings live under `config_local_dir()` instead of `data_dir()`.

## Problem
Octomus currently relies on filesystem watching for several user-visible behaviors: reloading themes, workflows, launch configs, tab configs, Octomus home MCP config, Octomus home skills, and public settings from `settings.toml`. The watcher surface is easy to regress because Octomus-managed files are split across different directories depending on platform and channel.

The specific failure modes this work addresses are:
- changes under `.octomus*/worktrees` can produce false-positive updates for Octomus home watchers
- a watcher rooted only at `data_dir()` can miss `settings.toml` on Linux and Windows, where `config_local_dir()` differs from `data_dir()`
- fresh installs or hermetic test environments can fail to watch missing directories unless Octomus prepares those roots before registering the watcher

## Goals
- Watch the current channel's Octomus-managed directories through a single Octomus-specific watcher model.
- Ignore filesystem activity under `.octomus*/worktrees` so worktree contents do not trigger Octomus home reload behavior.
- Continue reloading `settings.toml` when it changes on every supported platform, including platforms where settings live outside `data_dir()`.
- Preserve existing hot-reload behavior for themes, workflows, launch configs, tab configs, Octomus home MCP config, and Octomus home skills.

## Non-goals
- Changing where any Octomus-managed file is stored.
- Changing the semantics of settings parsing, settings migration, or settings validation.
- Adding new user-facing UI for watcher state or diagnostics.
- Expanding watch coverage to arbitrary files outside Octomus-managed directories.
- Changing the generic repository watcher APIs used for project repositories.

## Figma / design references
Figma: none provided

## User Experience

### Watch scope
- Octomus watches the current channel's Octomus-owned filesystem roots through a single singleton watcher.
- `data_dir()` remains the source of truth for channel-scoped Octomus home content such as themes, workflows, launch configs, tab configs, MCP config, and skills.
- `config_local_dir()` is also watched when it is a different directory from `data_dir()`.
- When both path helpers resolve to the same directory, Octomus behaves as before and does not create duplicate logical coverage.

### Settings hot reload
- When `settings.toml` changes, Octomus reloads public settings from disk and applies the new values to in-memory settings models.
- This behavior must work whether `settings.toml` lives in the same directory as the rest of Octomus home files or in a separate config directory.
- Creating, modifying, renaming into place, or deleting `settings.toml` must continue to flow through the existing `WarpConfigUpdateEvent::Settings` path.

### Worktree exclusion
- Files under `.octomus`, `.octomus-dev`, `.octomus-local`, or equivalent channel-scoped Octomus home directories that are nested inside `worktrees/` must not trigger Octomus home reload behavior.
- Editing files inside a cloned repository stored under `.octomus*/worktrees/...` must not cause Octomus to reload themes, workflows, tab configs, MCP config, skills, or settings.

### Channel awareness
- Octomus only reacts to files under the active channel's directories.
- A stable or dev install should not reload in response to files written into another channel's Octomus home.

### Fresh-install and test-environment behavior
- If a watched Octomus-owned root directory does not exist yet, Octomus should create it during startup/setup before registering the watcher.
- Missing directories must not silently disable hot reload for the rest of the session.

### No regressions for existing consumers
- Editing a theme file in Octomus home still updates the available theme set.
- Editing workflows, launch configs, or tab configs in Octomus home still refreshes those objects.
- Editing Octomus home MCP config still updates file-based MCP servers.
- Editing Octomus home skills still refreshes Octomus-provided skills.

## Success Criteria
- `settings.toml` hot reload works on macOS, Linux, and Windows.
- Worktree activity under `.octomus*/worktrees` no longer triggers Octomus home reloads.
- Themes, workflows, launch configs, tab configs, Octomus MCP config, and Octomus skills continue to hot reload from the current channel's Octomus home.
- Octomus prepares missing watch roots before attempting to register watchers.
- The watcher architecture remains centralized behind a Octomus-specific singleton instead of reintroducing separate ad hoc watchers for individual consumers.

## Validation
- Unit-test the watcher filtering behavior so updates outside the kept prefix are excluded and cross-boundary moves are handled correctly.
- Run the end-to-end settings hot-reload integration test that edits `settings.toml` multiple times and verifies the in-memory settings model changes after each write.
- Manually or through existing automated coverage, verify that editing Octomus home themes, skills, and MCP config still produces the expected reload behavior.
- Confirm via code review that only `data_dir()` receives the `worktrees` exclusion and `config_local_dir()` remains unfiltered.

## Open questions
- None currently.
