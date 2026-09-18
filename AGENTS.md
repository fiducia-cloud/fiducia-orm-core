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

<!-- BEGIN ores-agents-pointer: managed by ORESoftware/my-ai; edit there, not here -->

## Canonical agent instructions

Before doing anything else in this repository, also read:

    .ores/agents/AGENTS.md

That path is a symlink to `~/codes/oresoftware/my-ai/AGENTS.md`, whose canonical copy is
<https://github.com/ORESoftware/my-ai/blob/main/AGENTS.md>.

It exists at a fixed path *inside* the repository because some agents cannot walk up past
the repository root, so machine-wide instructions one or more directories above are
invisible to them. This pointer plus that path make the same file reachable from a working
directory anywhere in the tree.

The symlink is deliberately **not committed**: it names an absolute path that is only valid
on a machine with `~/codes/oresoftware/my-ai` checked out, so committing it would produce a
broken link for everyone else and for CI. `.ores/` is git-ignored for that reason. If
`.ores/agents/AGENTS.md` is missing on your machine, create it with:

    mkdir -p .ores/agents
    ln -sfn "$HOME/codes/oresoftware/my-ai/AGENTS.md" .ores/agents/AGENTS.md

or run `~/codes/oresoftware/my-ai/scripts/link-repo-agents.sh` once to do it for every git
repository under `~/codes`, and `--check` to verify them.

A missing `.ores/agents/AGENTS.md` is a setup gap on the reader's machine, never a reason to
skip the canonical instructions: fetch them from the URL above instead.

<!-- END ores-agents-pointer -->
