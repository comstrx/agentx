# Rust CLI infra/ops tool — single fast binary monolith

A self-contained command-line tool written in Rust and shipped as one fast, statically-linked binary that is
**both a library and a CLI**. It automates an infrastructure, build, or developer-ops workflow — scaffolding,
orchestration, deployment, codegen, or system control — with no runtime, no daemon, and no external services
to stand up. Opinionated and singular: one crate, one architecture, one path.

## Stack
- **Rust (edition 2024, recent MSRV)** — `#![forbid(unsafe_code)]`, a strict gate (`cargo clippy
  --all-targets -D warnings` + `cargo build` + `cargo test`).
- **clap** (derive) for the CLI surface; the binary is a thin dispatcher over the library API.
- **Async is contained** — Tokio lives only at the I/O edges (process spawn, network) behind a sync facade;
  upper layers stay synchronous.
- **serde** + `toml`/`json` for config and state; atomic file writes (tmp + fsync + rename) for anything
  durable.
- Single self-contained binary — prefer the standard library and a small set of vetted crates over heavy
  frameworks; no plugin system, no dynamic loading.

## Architecture (the "magic")
Strict downward-only layering: a neutral `core` (errors + a reusable `support` std-lib) → `config` (constants
and typed config) → `app` (the tool logic and its CLI). Every command is a thin wrapper over one method on a
public `App` type, so the library does exactly what the CLI does — call it from another Rust program and get
identical results. Reusable logic lives in the lowest layer that fits (a `support` helper or a shared type);
upper layers read like the use case. State is an explicit, serializable, atomically-checkpointed struct, so
long operations are resumable and safe to stop.

## Best fit when the project is
- A **single-binary CLI / ops / infra tool** in Rust — orchestrator, scaffolder, deployer, codegen, migrator,
  or system controller.
- Meant to be **both a library and a binary** from one crate, with the CLI as a thin layer over a stable
  public API.
- Valuing **a fast static binary, strong separation of concerns, resumable/atomic state, and a clean,
  hand-crafted std-lib** over breadth of dependencies.

## Not the fit when
- A web service, API backend, daemon, or GUI app; a multi-crate workspace or plugin platform; or a
  throwaway script where the layered architecture and library/binary parity would be over-engineering.
