# Architecture

## Dependencies

OpenClaw
  -> execution runtime, sandboxing, approvals, tools, routing

AgentSecrets
  -> secret brokerage, capability tokens, manual approval for secret use

AgentShell
  -> shell UI, workflow orchestration, safe defaults, profiles
  -> planning layer for OpenClaw session launches and AgentSecrets approvals

Mission Control
  -> product UI that can compose AgentShell and AgentSecrets

## Design Rule

If OpenClaw already owns the primitive, AgentShell should configure it.
If the primitive is about secret exposure, delegate to AgentSecrets.
If the primitive is about user workflow or UI, keep it in AgentShell.
If the primitive is a request plan, keep it in AgentShell and emit a typed contract.
