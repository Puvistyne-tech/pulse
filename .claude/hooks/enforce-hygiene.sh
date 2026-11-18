#!/usr/bin/env bash
set -uo pipefail

repo_root="${CLAUDE_PROJECT_DIR:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
forbidden_term="sl""ack"
comment_pattern='^[[:space:]]*(//|/\*|\*([[:space:]]|/|$)|--[[:space:]])'
violations=()

relative_path() {
  local path="$1"
  case "$path" in
    "$repo_root"/*) printf '%s' "${path#"$repo_root"/}" ;;
    *) printf '%s' "$path" ;;
  esac
}

is_permitted_markdown() {
  case "$1" in
    CLAUDE.md|.claude/lessons.md) return 0 ;;
    .claude/agents/*.md) [[ "$1" != .claude/agents/*/* ]] && return 0 ;;
  esac
  return 1
}

is_brand_exempt() {
  case "$1" in
    .claude/hooks/*|*/import-adapters/*|*/migration-adapters/*|*.lock|*-lock.yaml|*-lock.json) return 0 ;;
  esac
  return 1
}

inspect_file() {
  local rel="$1"
  local abs="$repo_root/$rel"
  case "$rel" in
    .git/*|*/node_modules/*|node_modules/*|target/*|*/target/*|*/build/*|*/.dart_tool/*|*/dist/*) return ;;
  esac
  if [[ "$rel" == *.md ]] && ! is_permitted_markdown "$rel"; then
    violations+=("$rel: markdown files are forbidden outside CLAUDE.md, .claude/agents/*.md, .claude/lessons.md")
  fi
  [[ -f "$abs" ]] || return
  case "$rel" in
    *.rs|*.ts|*.tsx|*.dart|*.sql)
      local hits
      hits="$(grep -nE "$comment_pattern" "$abs" | head -n 5)"
      if [[ -n "$hits" ]]; then
        while IFS= read -r hit; do
          violations+=("$rel:${hit%%:*}: comments are forbidden in code files")
        done <<< "$hits"
      fi
      ;;
  esac
  if ! is_brand_exempt "$rel"; then
    local brand_hits
    brand_hits="$(grep -nIi "$forbidden_term" "$abs" | head -n 5)"
    if [[ -n "$brand_hits" ]]; then
      while IFS= read -r hit; do
        violations+=("$rel:${hit%%:*}: forbidden trademark term, use Pulse domain vocabulary")
      done <<< "$brand_hits"
    fi
  fi
}

changed_files() {
  git -C "$repo_root" status --porcelain --untracked-files=all | sed -E 's/^.{3}//; s/.* -> //'
}

all_files() {
  git -C "$repo_root" ls-files --cached --others --exclude-standard
}

mode="${1:-}"
if [[ "$mode" == "--all" ]]; then
  while IFS= read -r file; do [[ -n "$file" ]] && inspect_file "$file"; done < <(all_files)
else
  payload="$(cat 2>/dev/null || true)"
  target="$(printf '%s' "$payload" | jq -r '.tool_input.file_path // empty' 2>/dev/null || true)"
  if [[ -n "$target" ]]; then
    inspect_file "$(relative_path "$target")"
  else
    while IFS= read -r file; do [[ -n "$file" ]] && inspect_file "$file"; done < <(changed_files)
  fi
fi

if (( ${#violations[@]} > 0 )); then
  {
    echo "Pulse hygiene violation. Fix these before continuing and record the root cause in .claude/lessons.md:"
    printf '  %s\n' "${violations[@]}"
  } >&2
  exit 2
fi
exit 0
