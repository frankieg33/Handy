# HANDOFF — PR 1: Deterministic Personal Lexicon & Explicit Aliases

## Current Repository State

- **Fork / Origin**: `https://github.com/frankieg33/Handy.git`
- **Upstream**: `https://github.com/cjpais/Handy.git`
- **Base Upstream Commit**: `c62a5fcdef4196e0ab36ea56cd3863f8f17fd9c5`
- **Canonical Product Branch**: `release` (tracks `origin/release`, default branch on GitHub)
- **Upstream Mirror Branch**: `main` (tracks `origin/main`, clean upstream mirror)
- **PR 0 Status**: Completed and merged into `release` via squash merge (#1).
- **Updater Safety**: Hard-disabled via `src-tauri/src/downstream/mod.rs` (`DISABLE_UPDATER = true`) hooked into `update_checks_forced_disabled()` in `src-tauri/src/settings.rs`, with `createUpdaterArtifacts = false` and `endpoints = []` in `tauri.conf.json`.

---

## PR 1 Objective

Build the foundation of the personal vocabulary subsystem: **deterministic personal canonical terms and explicit misrecognition aliases**, completely on-device, with zero LLM, cloud, or external network dependencies.

### Scope for PR 1
1. **Isolated Storage (`personalization.db`)**:
   - Independent SQLite database (`<app_data_dir>/personalization.db`) using existing `rusqlite`.
   - Separate from `history.db` and `settings_store.json` to keep migrations cleanly isolated.
2. **Data Model**:
   - **Terms (`terms`)**: `id`, `canonical`, `category` (optional), `enabled`, `created_at`, `updated_at`, `use_count`.
   - **Aliases (`aliases`)**: `id`, `term_id`, `alias` (e.g. "siege" -> "Sietch", "see itch" -> "Sietch"), `source` (manual/imported), `created_at`, `observation_count`.
3. **Deterministic Alias Replacement Engine**:
   - Longest-match first, boundary-aware (respects word boundaries).
   - Multi-word alias support ("see itch" -> "Sietch").
   - Punctuation preservation ("ask siege, please" -> "ask Sietch, please").
   - Canonical casing preserved ("ap i" -> "API").
   - Idempotent and fail-open (any error returns untouched transcript).
4. **Pipeline Integration**:
   - Insert alias replacement into the transcription pipeline immediately after raw ASR output and before existing normalization/custom-word logic.
   - Works across all transcription engines (Whisper and non-Whisper).
   - Injects active canonical terms into ASR initial prompt bias (Whisper prompt path) alongside existing `settings.custom_words`.
5. **Tauri Commands & State**:
   - `PersonalizationManager` initialized in Tauri state.
   - CRUD Tauri commands for terms and aliases.
6. **Frontend UI**:
   - Simple Personal Vocabulary / Lexicon management tab or section under Settings.
   - Add/edit/delete canonical terms and explicit aliases.
7. **Comprehensive Tests**:
   - Unit tests for database CRUD, migrations, boundary matching, multi-word matching, punctuation preservation, casing, Unicode, and fail-open behavior.

### Explicitly Non-Goals for PR 1 (Deferred to subsequent PRs)
- No automatic correction mining / learning (PR 2/3).
- No transcript editing in History (PR 2).
- No repository / local filesystem scanning (PR 5).
- No LLMs, embeddings, or cloud APIs.

---

## Architecture & Isolation Seams

All backend personalization code must reside exclusively in:
```text
src-tauri/src/personalization/
├── mod.rs          # Manager & public API
├── db.rs           # SQLite schema, migrations, connection pool
├── lexicon.rs      # Terms & aliases data structures and CRUD
└── engine.rs       # Deterministic alias matching & replacement algorithm
```

Integration points with upstream Handy core:
1. **`src-tauri/src/lib.rs`**:
   - Register `pub mod personalization;`
   - Initialize `PersonalizationManager` in Tauri `.manage()`.
2. **`src-tauri/src/managers/transcription.rs`**:
   - In `transcribe()`, after raw text generation, call `personalization_manager.apply_aliases(&raw_text)`.
   - In Whisper initial prompt generation, merge `settings.custom_words` + `personalization_manager.get_prompt_terms()`.
3. **`src-tauri/src/commands/mod.rs`** (or `personalization.rs`):
   - Expose Tauri Specta commands for frontend IPC.

---

## Verification Commands for Next Session

```bash
# Frontend linting & formatting
bun run format:check
bun run lint
bun run check:translations
bun run check:model-languages

# Backend compilation & tests
cd src-tauri
cargo test
cargo clippy -- -D warnings

# Build application
bun run tauri build --no-bundle
```

---

## Feature Branch Workflow for PR 1

```bash
git switch release
git pull origin release
git switch -c feat/personal-lexicon release

# ... implement and verify ...

git push -u origin feat/personal-lexicon
gh pr create --repo frankieg33/Handy --base release --head feat/personal-lexicon
gh pr merge --squash --delete-branch
```
