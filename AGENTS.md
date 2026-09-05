# Repository agent instructions

This repository is the canonical Fiducia ORM and named-database-operation boundary.
Apply `ORESoftware/my-ai/AGENTS.md` plus these narrower rules.

- Keep at least 90% of Fiducia ORM entities, connection adapters, generated SQL,
  parity logic, and named database operations in this repository. Servers may
  own request orchestration, authorization, and DTO mapping, but not duplicate
  Diesel/SeaORM queries or expose raw ORM sessions.
- TypeSpec and JSON Schema are independent peer authorities. Generate candidate
  Rust/SQL from both only after normalized parity; any discrepancy must stop for
  evaluation instead of choosing a winner.
- SeaORM and Diesel execute independently against the same reviewed catalog.
  Neither ORM may generate, wrap, or certify the other.
- Raw connections, query builders, backend errors, credentials, and database
  URLs stay private. Export opaque contexts and named tenant/domain operations.
- Web consumers use read-only contexts and database principals. API consumers
  opt into the write surface. Cargo features express intent; grants and workload
  identity are the security boundary.
- This crate never applies production DDL at startup. Reviewed declarative SQL
  remains migration authority and is applied by a fenced one-shot migrator.
- Never rebase, force-push, reset, or stash. Resolve conflicts semantically and
  publish verified work through a feature branch and pull request.

## Repository-local Git worktrees

- Create or use a Git worktree only when the human operator explicitly authorizes it for the current task. Concurrency or a dirty checkout is not permission by itself.
- Put every authorized worktree at `<repository-root>/tmp/worktrees/<name>`; from the repository root, use `./tmp/worktrees/<name>`. Never place worktrees beside repositories or organization directories.
- Keep `tmp`, `temp`, `tmp/worktrees`, and `temp/worktrees` ignored in the repository-root `.gitignore`. Do not commit files from those directories.
- Relocate or remove a worktree only when the operator explicitly requests it. Before removal, preserve and publish intended changes, verify its commit is represented on the target branch, and confirm there are no tracked, untracked, ignored-sensitive, or in-use files that must survive. Remove it with `git worktree remove <path>` without `--force`; never delete a worktree directory with `rm`.
