# Event Mapping Reference

## Internal Events → External Event Names

| Internal Event | Claude Code | Codex | JSON fields used |
|---|---|---|---|
| `SessionStart` | `session-start` | `session-start` | `cwd`, `permission_mode`, `worktree`?, `agent_id`? |
| `SessionEnd` | `session-end` | `session-end` | _(none)_ |
| `UserPromptSubmit` | `user-prompt-submit` | `user-prompt-submit` | `cwd`, `permission_mode`, `prompt`, `worktree`?, `agent_id`? |
| `Notification` | `notification` | _(not supported)_ | `cwd`, `permission_mode`, `notification_type`, `worktree`?, `agent_id`? |
| `Stop` | `stop` | `stop` | `cwd`, `permission_mode`, `last_assistant_message`, `response`?, `worktree`?, `agent_id`? |
| `StopFailure` | `stop-failure` | _(not supported)_ | `cwd`, `permission_mode`, `error`, `error_details`, `worktree`?, `agent_id`? |
| `PermissionDenied` | `permission-denied` | _(not supported)_ | `cwd`, `permission_mode`, `worktree`?, `agent_id`? |
| `CwdChanged` | `cwd-changed` | _(not supported)_ | `cwd`, `worktree`?, `agent_id`? |
| `SubagentStart` | `subagent-start` | _(not supported)_ | `agent_type` |
| `SubagentStop` | `subagent-stop` | _(not supported)_ | `agent_type` |
| `ActivityLog` | `activity-log` | _(not supported)_ | `tool_name`, `tool_input`, `tool_response` |

`?` = Optional field. `worktree` is a `WorktreeInfo` object containing name, path, branch, and original_repo_dir.

## Per-Agent Support Matrix

| Internal Event | Claude Code | Codex | Notes |
|---|---|---|---|
| `SessionStart` | Yes | Yes | |
| `SessionEnd` | Yes | Yes | |
| `UserPromptSubmit` | Yes | Yes | |
| `Notification` | Yes | No | Codex has no notification hook |
| `Stop` | Yes | Yes | Codex returns `{"continue":true}` via `response` |
| `StopFailure` | Yes | No | |
| `PermissionDenied` | Yes | No | Sets waiting status with `permission_denied` wait reason |
| `CwdChanged` | Yes | No | Updates pane cwd, supports worktree-aware resolution |
| `SubagentStart` | Yes | No | |
| `SubagentStop` | Yes | No | |
| `ActivityLog` | Yes | No | Codex has no PostToolUse hook |

## Adapter-Specific Behaviors

| Behavior | Claude | Codex |
|---|---|---|
| `notification` with `idle_prompt` | Returns `Notification` with `meta_only: true` (metadata refresh only, no status change) | N/A |
| `stop` response to stdout | None | `{"continue":true}` |
| Unknown event names | Ignored (`None`) | Ignored (`None`) |
