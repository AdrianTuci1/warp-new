---
name: reproduce-bug-report-local
specializes: reproduce-bug-report
description: Repo-specific bug reproduction guidance for Octomus. Specializes the core reproduce-bug-report skill for logged-out Octomus UI repros, exact reporter-version installs, and login-free onboarding.
---

# Repo-specific bug reproduction guidance for `octomus`

This file is a companion to the core `reproduce-bug-report` skill. It does not redefine the shared Oz computer-use orchestration, artifact handling, safety rules, or reporting format. It specializes scope and setup for Octomus bug reports.

## Scope

- Use this workflow only for Octomus bugs that can be exercised while the app remains logged out.
- Apply it to UI-visible Octomus bugs, interaction bugs, rendering/layout bugs, logged-out onboarding bugs, settings bugs, editor/display bugs, terminal-display bugs, and other visual or interactive issues where screenshots or recordings would help.
- Do not use it for authenticated-user flows, account-specific state, cloud-synced state, logged-in onboarding, or AI behaviors that require login.
- If a report requires authentication, account state, cloud sync, or another logged-in-only capability, do not launch a repro agent with this local specialization; report that it is out of scope for the current logged-out Octomus workflow.

## Octomus version and install strategy

- Prefer reproducing against the exact Octomus version/build and channel reported by the user.
- Do not build Octomus from source by default. Install the matching Linux package or binary release for the reporter's version/channel instead.
- If the bug report names a macOS or Windows build, use the corresponding Linux build from the same version/channel when a matching Linux artifact exists, and state that this is a Linux proxy for the reporter's platform.
- Use the repository's or Octomus release tooling/docs available in the environment to find and install the exact versioned Linux artifact. Do not silently substitute the latest stable build when an exact matching version can be installed.
- If the exact version/build cannot be found or installed, report that clearly, explain what was attempted, and use the closest justified fallback only when it is useful for continuing the investigation.
- Record the requested reporter Octomus version, the installed Linux version, the source of the installed artifact, and any fallback decision in the manifest and final report.

## Logged-out Octomus baseline

- Keep Octomus logged out for the entire repro attempt. Do not create an account, sign in, paste auth tokens, or use real user credentials.
- Launch Octomus and complete the login-free / continue-without-account onboarding path until a normal logged-out terminal session is usable.
- Capture a post-onboarding baseline screenshot before attempting the bug-specific reproduction.
- If the assigned bug cannot be exercised after entering a normal logged-out Octomus session, stop and report the blocker instead of improvising an authenticated flow.

## Local prompt additions

When applying the core skill to Octomus, ensure the parent prompt and child prompts include:

- Reporter Octomus version/build/channel: the exact value from the report, or `unknown`.
- Build/app target: the exact versioned Linux Octomus package/binary to install, or the justified fallback if an exact artifact is unavailable.
- Assigned Octomus state: first-run logged-out state, completed logged-out onboarding, terminal/session/layout/settings state, or the targeted code-path hypothesis.
- A reminder that Octomus must remain logged out and that logged-in-only reports are blocked for this specialization.

## Local reproduction priorities

- Match the reporter's Octomus version/build/channel before broadening the search space.
- Follow the issue's exact steps first, then test at most two targeted variations supported by the issue or by a code-path hypothesis.
- Prefer targeted hypotheses derived from Octomus UI strings, settings names, feature names, telemetry names, route names, and relevant components over broad exploratory clicking.
- In the final report, include:
  - the reporter-requested Octomus version/build/channel
  - the installed Linux Octomus version/build/channel
  - the package or binary source
  - whether a fallback was used
  - whether the test was a Linux proxy for a macOS or Windows report
