# Downstream Handy Maintenance Guide

This repository is a personal downstream fork of [cjpais/Handy](https://github.com/cjpais/Handy).

## Repository & Remotes

- **`upstream`**: `https://github.com/cjpais/Handy.git` (authoritative upstream Handy)
- **`origin`**: `https://github.com/frankieg33/Handy.git` (downstream fork)

## Branch Semantics & Policy

### `main` — Upstream Mirror

- `main` is a pristine mirror of `upstream/main`.
- `main` tracks `origin/main` so normal push operations remain safe.
- **Never** place downstream product customizations directly on `main`.
- **Never** develop features directly from `main`.
- **Never** merge `release` back into `main`.

### `release` — Canonical Downstream Product

- `release` is the canonical downstream product branch.
- `release` tracks `origin/release` and is the default branch of the downstream fork.
- All downstream feature branches start from `release` and merge back into `release`.
- All local development, testing, builds, and installations are performed from `release`.
- **Never** rebase or force-push `release` (preserve stable history).

---

## Workflows

### 1. Downstream Feature Workflow

```text
release
   │
   └── feat/my-feature
             │
       (PR / squash)
             ↓
          release
```

1. Create a feature branch from current `release`: `git switch -c feat/my-feature release`
2. Implement, test, and verify.
3. Open PR targeting `release`.
4. Squash merge into `release`.
5. Delete the feature branch locally and remotely.

### 2. Upstream Synchronization Workflow

When upstream Handy releases updates:

```bash
# 1. Fetch upstream changes
git fetch upstream

# 2. Fast-forward local main to upstream/main
git switch main
git merge --ff-only upstream/main

# 3. Push pristine main to fork
git push origin main

# 4. Open and test PR main -> release
# If conflicts occur, use a temporary sync branch:
git switch -c sync/handy-YYYY-MM-DD release
git merge main
# Resolve conflicts, run tests, build
# Merge sync branch into release with a regular merge commit (NOT squash/rebase)
```

### 3. Upstream Contributions

If a downstream capability is generalizable and suitable for upstream:

```text
main
 │
 └── upstream-pr/feature-name
               │
               └── PR -> cjpais/Handy:main
```

- Create branch from `main`: `git switch -c upstream-pr/feature-name main`
- Cherry-pick/implement the clean upstreamable change.
- Submit PR to `cjpais/Handy:main`.
- Upstream contribution branches are independent exports and are not merged into `release`.

---

## Downstream Updater Policy

Downstream builds hard-disable update checks (`src-tauri/src/downstream/mod.rs` & `update_checks_forced_disabled()`) and disable updater artifact generation (`createUpdaterArtifacts: false`).

This guarantees the custom downstream installation will never silently overwrite itself with vanilla upstream Handy releases.

---

## Inspecting Downstream State

- **View downstream delta against upstream base**:
  ```bash
  git diff main...release
  ```
- **View integration & merge history**:
  ```bash
  git log --first-parent release
  ```
- **Inspect current upstream base SHA**:
  ```bash
  git rev-parse main
  git merge-base main release
  ```
