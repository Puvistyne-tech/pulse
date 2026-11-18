# Pulse Lessons

Imperative rules distilled from guardian rejections, hook blocks, and failed gates. Max 100 lines. Prune obsolete rules before adding new ones.

- Read CLAUDE.md and this file before every task.
- Run `.claude/hooks/enforce-hygiene.sh --all` before reporting any task complete.
- Never add a glob to the root Cargo.toml `members` that matches nothing; cargo errors. Add `apps/*` members in the same commit that creates those crates.
- Never put `#[serde(transparent)]` on types deriving `TS`; ts-rs warns on every build and newtype structs already serialize transparently.
- Create new directories with separate plain `mkdir` calls or the Write tool; compound multi-path shell commands are refused inside worktrees.
- Branch every builder worktree from current local `main`, never from the worktree's default base, so CLAUDE.md and the hook are present.
- Build wire frames in server tests by serializing `pulse-protocol` types; hand-write raw JSON only for malformed-payload cases.
