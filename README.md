# AgentShell

AgentShell is the host shell and UI layer for OpenClaw-based agent setups.

It is intentionally thin:
- OpenClaw owns runtime execution, sandboxing, approvals, and backend routing.
- AgentSecrets owns zero-trust secret mediation.
- AgentShell owns the user-facing shell, workflows, presets, and OpenClaw integration glue.

This repo is the start of a separate project, not a Mission Control subfolder.

## Scope

AgentShell should:
- wrap OpenClaw instead of reimplementing it
- provide a clean UI and launch flow
- expose safe defaults and per-project profiles
- plug into AgentSecrets for secret-dependent actions
- stay usable across OpenClaw setups, not just one app

## Non-goals

AgentShell should not:
- reimplement OpenClaw's sandbox or exec engine
- become a second secret broker
- hardcode Mission Control assumptions into its contract

## License

Apache-2.0
