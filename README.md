# AgentShell

AgentShell is the host shell and UI layer for OpenClaw-based agent setups.

It is intentionally thin:
- OpenClaw owns runtime execution, sandboxing, approvals, and backend routing.
- AgentSecrets owns zero-trust secret mediation.
- AgentShell owns the user-facing shell, workflows, presets, and OpenClaw integration glue.

This repo is the start of a separate project, not a Mission Control subfolder.

## Current State

This repo currently ships a minimal Rust skeleton:
- CLI entrypoint
- typed config from environment
- local `/healthz` endpoint
- explicit OpenClaw and AgentSecrets wiring
- session launch planning for OpenClaw
- secret approval planning for AgentSecrets
- HTTP dispatch clients for OpenClaw and AgentSecrets
- persistent task engine with task trees, message privacy, approvals, and event history
- mission-style task projections for consumer UIs
- task-scoped approval request/resolve state machine with expiry metadata

## Scope

AgentShell should:
- wrap OpenClaw instead of reimplementing it
- provide a clean UI and launch flow
- expose safe defaults and per-project profiles
- plug into AgentSecrets for secret-dependent actions
- stay usable across OpenClaw setups, not just one app
- generate request plans for session launch and secret approval
- dispatch those plans over HTTP when asked
- treat each task as the unit of permission and control

## Roadmap

See [docs/ROADMAP.md](docs/ROADMAP.md) for the current AgentShell milestone plan.
See [docs/PERMISSIONS.md](docs/PERMISSIONS.md) for `plan`, `edit`, and `yolo` permission modes.
See [docs/PERMISSION_MATRIX.md](docs/PERMISSION_MATRIX.md) for the mode transition and profile model.
See [docs/TASK_MODEL.md](docs/TASK_MODEL.md) for task-scoped permissions and parallel subagents.
See [docs/TASK_UI.md](docs/TASK_UI.md) for the chat-agent task surface and child-task tree.
See [docs/TASK_LIFECYCLE.md](docs/TASK_LIFECYCLE.md) for task states and execution flow.
See [docs/TASK_CONTRACT.md](docs/TASK_CONTRACT.md) for the task wire contract and events.
See [docs/MISSION_BRIDGE.md](docs/MISSION_BRIDGE.md) for how mission-style consumers project AgentShell tasks.
See [docs/APPROVAL_FLOW.md](docs/APPROVAL_FLOW.md) for secret, desktop, and operator approvals.
See [docs/APPROVAL_UI.md](docs/APPROVAL_UI.md) for approval prompt copy and visual states.
See [docs/MESSAGE_PRIVACY.md](docs/MESSAGE_PRIVACY.md) for message visibility consent.
See [docs/OPENCLAW_HAZARDS.md](docs/OPENCLAW_HAZARDS.md) for the OpenClaw risk map.
See [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md) for host compromise and recovery assumptions.

## Non-goals

AgentShell should not:
- reimplement OpenClaw's sandbox or exec engine
- become a second secret broker
- hardcode Mission Control assumptions into its contract

## License

AGPL-3.0-or-later
