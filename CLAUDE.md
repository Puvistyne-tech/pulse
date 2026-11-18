# Pulse Engineering Constitution

Pulse is an open-source, self-hostable, clean-room team communication platform built as a Shared Rust Core with Thin UI Shells. Every rule in this file is binding for humans, the orchestrator, and every subagent.

## Domain Vocabulary (mandatory)

Use only these nouns in code, identifiers, tests, migrations, and UI strings. Never use the name of any existing commercial chat product, its brand colors, or its product-specific nouns.

| Concept | Pulse term | Storage / type |
|---|---|---|
| Application | `Pulse` | |
| Workspace / organization | `Campus` | `campuses` |
| Multi-org federation | `Federation` | `federations` |
| Channel | `Room` | `rooms` (defaults `#commons`, `#courtyard`) |
| Direct message | `Note` | `direct_notes` |
| Reply chain | `Thread` | `threads` (optional `topic_title`) |
| Drop-in voice/video room | `Lounge` | `lounges` |
| Bot / AI agent member | `Steward` | `stewards` |
| External org bridge | `Portal` | `portals` |
| Collaborative doc | `Blueprint` | `blueprints` |
| Async voice/video clip | `Memo` | `memos` |
| Workflow automation | `Routine` | `routines` |
| Saved / deferred queue | `Shelf` | `shelf_items` |

## Monorepo Layout (Option A: single Rust brain)

- `crates/pulse-protocol`: shared DTOs, WebSocket event enums, RBAC permissions, `AgentStreamChunk` frames, `ts-rs` TypeScript bindings. Must compile for native and `wasm32-unknown-unknown`. No I/O, no async runtime.
- `crates/pulse-core`: sans-I/O client engine. Delta-sync state machine, optimistic message queue keyed by `client_nonce`, Unified Thread Inbox and subscription tracker, local search index. All I/O lives behind async traits `LocalStore` and `NetworkTransport`. Must compile for `wasm32-unknown-unknown` with default features off; SQLite FTS5 lives behind a native-only feature.
- `crates/pulse-core-wasm`: `wasm-bindgen` adapter exposing `pulse-core` to browsers. Adapter code only.
- `crates/pulse-server`: Axum HTTP + WebSocket gateway, SQLx on PostgreSQL with strict `campus_id` isolation, Valkey pub/sub fan-out and TTL presence/typing, S3/MinIO presigned uploads.
- `apps/web-desktop`: React 19 + TypeScript + Tailwind + TipTap inside Tauri v2. Thin view layer. Every `#[tauri::command]` is `async`. No business logic in React state; React renders snapshots produced by `pulse-core`.
- `apps/mobile`: Flutter + Riverpod calling `pulse-core` through `flutter_rust_bridge` v2. Thin view layer. No business logic in Dart.
- `docker-compose.yml`: PostgreSQL, Valkey, MinIO for local development.

Dependency direction is strictly `pulse-protocol` <- `pulse-core` <- (`pulse-core-wasm`, Tauri shell, mobile bridge) and `pulse-protocol` <- `pulse-server`. `pulse-server` never depends on `pulse-core`. UI shells never redefine a type that exists in `pulse-protocol`; TypeScript types come from `ts-rs` output.

## Non-Negotiable Hygiene Rules

1. NO CODE COMMENTS. Zero inline or doc comments (`//`, `/* */`, `#`, `///`, `//!`, `--`) in any `.rs`, `.ts`, `.tsx`, `.dart`, or `.sql` file. Express intent through naming and types.
2. NO AUTO-GENERATED MARKDOWN. The only permitted `.md` files are `CLAUDE.md`, `.claude/agents/*.md`, and `.claude/lessons.md`. Plans, summaries, and notes go in agent responses, never in files.
3. ZERO CODE DUPLICATION. Structs, DTOs, validation, WebSocket schemas, and fixtures exist exactly once. Shared contracts live in `crates/pulse-protocol`.
4. NO DUPLICATE TESTS. Consolidate cases into table-driven tests (`#[rstest]`, `test.each`, parameterized Flutter groups). Share setup through one fixture module per crate or app.
5. STRICT MULTI-TENANCY. Every SQL query, cache key, pub/sub channel, and WebSocket room is scoped by `campus_id`.

## Verification Gates

A change is mergeable only when all of these pass:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo check --workspace`
- `cargo check -p pulse-core-wasm --target wasm32-unknown-unknown`
- `cargo test --workspace`
- `pnpm --dir apps/web-desktop typecheck` and `pnpm --dir apps/web-desktop test`
- `flutter analyze` and `flutter test` in `apps/mobile` when the Flutter SDK is present
- `.claude/hooks/enforce-hygiene.sh --all` exits 0
- `architecture-guardian` returns `VERDICT: APPROVE`

## Factory Workflow

Work ships in vertical slices. For each slice the orchestrator runs: plan (`planner`), parallel build (`rust-core-builder`, `ui-shell-builder` in isolated worktrees), verify (`qa-verifier`), audit (`architecture-guardian`), then `git merge --no-ff` and full verification. Every guardian rejection, hook block, or failed gate is distilled into one imperative rule in `.claude/lessons.md`. Read `.claude/lessons.md` before starting any task.
