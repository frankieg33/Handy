# Downstream Handy Development Policy

This repository is a downstream build of `cjpais/Handy`.

- `release` is the canonical product branch for all development, builds, tests, and PRs.
- `main` is a pristine mirror of upstream Handy tracking `origin/main`.
- Feature branches start from `release` and merge into `release` via squash merge.
- Never commit downstream changes directly to `main` or merge `release` into `main`.
- Upstream syncs fast-forward `main` to `upstream/main` and merge `main` into `release` via merge commit.

For detailed branch policies and workflows, see [docs/DOWNSTREAM.md](docs/DOWNSTREAM.md).
For upstream developer guidelines and architecture, see [AGENTS.md](AGENTS.md).
