# Pulse Lessons

Imperative rules distilled from guardian rejections, hook blocks, and failed gates. Max 100 lines. Prune obsolete rules before adding new ones.

- Read CLAUDE.md and this file before every task.
- Run `.claude/hooks/enforce-hygiene.sh --all` before reporting any task complete.
- Never add a glob to the root Cargo.toml `members` that matches nothing; cargo errors. Add `apps/*` members in the same commit that creates those crates.
- Never put `#[serde(transparent)]` on types deriving `TS`; ts-rs warns on every build and newtype structs already serialize transparently.
- Create new directories with separate plain `mkdir` calls or the Write tool; compound multi-path shell commands are refused inside worktrees.
- Branch every builder worktree from current local `main`, never from the worktree's default base, so CLAUDE.md and the hook are present.
- Build wire frames in server tests by serializing `pulse-protocol` types; hand-write raw JSON only for malformed-payload cases.
- Never leave a table case that is a strict prefix of another case; merge them.
- A backoff cap case must run past the policy maximum and assert the plateau.
- Wrap every multi-client network test in `tokio::time::timeout` so a stalled server fails instead of hanging.
- Never put data-carrying enum variants in `pulse-protocol` types that cross the frb boundary; frb makes them opaque. Use fieldless enums plus sibling fields.
- Use the hyphenated package name `pulse-protocol` in `flutter_rust_bridge.yaml` `rust_input`, and create an empty `rust/src/frb_generated.rs` before the first codegen run.
- Under pnpm 11 allow esbuild with `allowBuilds: { esbuild: true }` in `pnpm-workspace.yaml`; add `@types/node` when `vite.config.ts` imports `node:*`.
- Run multi-step shell edits inside worktrees as separate plain Bash calls or scripts written with the Write tool; compound `cd`/heredoc/`mkdir` chains are refused.
