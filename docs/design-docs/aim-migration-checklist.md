# AIM Migration Implementation Checklist

Everything that needs to be created in `CryptoToolsAISkills/` at the workspace root.
Check off each item as it's completed. Reference the design doc for rationale.

## Directory Structure

- [x] `CryptoToolsAISkills/` root directory

## Conventions (shared definitions referenced by all SOPs)

- [x] `agent-sops/conventions.md` — severity levels, smart quotes, duvet commands, annotation path conventions, annotation types, formatting rules, error construction patterns, status keywords, review categories, orchestrator boundaries

## Skills (on-demand expertise, loaded via progressive disclosure)

- [x] `skills/duvet-annotation-patterns/SKILL.md` — placement patterns (7 patterns from duvet-patterns-rust.md), formatting rules, cross-reference following, annotation stacking rules, reason lines. This is the big one — merges duvet-patterns-rust.md + 04-spec-annotation-principles.md
  - [x] `skills/duvet-annotation-patterns/references/rust-patterns.md` — full detailed patterns with code examples (the stuff that makes SKILL.md too long if inlined)
- [x] `skills/duvet-workflow/SKILL.md` — phased duvet workflow (extract → plan → implement → test → verify). Converted from 01-duvet-requirements.md
- [x] `skills/spec-driven-dev/SKILL.md` — how to read specs, find requirements in compliance TOMLs, map requirements to code locations
- [x] `skills/rust-conventions/SKILL.md` — Rust style, error handling (ser_err, let-else), testing patterns (V1/V2 loop, raw byte parsing, boundary conditions, known-value assertions), no `as` casts for narrowing
- [x] `skills/review-checklist/SKILL.md` — the context-reset technique, annotation isolation evaluation, anti-rationalization check. Review methodology as a skill (not the full SOP, just the technique)

## Agent SOPs (step-by-step workflows with MUST/SHOULD/MAY constraints)

- [x] `agent-sops/duvet-discover.sop.md` — converted from agent-1-formulate.md. Steps: understand project → check regressions → run duvet query → prioritize → gather exact requirements → analyze spec structure → write work-item.md
- [x] `agent-sops/duvet-implement.sop.md` — converted from agent-2-implement.md. Steps: read context → pre-implementation reasoning → implement code with annotations → write tests → verify (duvet query loop) → verify test coverage → report completion
- [x] `agent-sops/duvet-review.sop.md` — converted from agent-3-review.md. Steps: load context → check stacking (hard gate) → check quote accuracy → check placement quality → check test coverage → run live verification → anti-rationalization check → write output
- [x] `agent-sops/duvet-cycle.sop.md` — converted from coordinator.md. Orchestration: validate inputs → dispatch discover → dispatch implement → pre-review gate → dispatch review → handle result → commit gates → re-discover loop
- [x] `agent-sops/duvet-commit.sop.md` — commit gate sequence extracted from coordinator + agent-3. Steps: cargo check → cargo test → cargo clippy → duvet query (scoped) → make duvet → git add (only modified files) → git commit → git log verify

## Context Files (always-loaded background info)

- [x] `context/esdk-overview.md` — high-level ESDK concepts shared across repos (what the ESDK is, spec structure, cross-language nature, Dafny transpilation)
- [x] `context/duvet-tool-reference.md` — duvet command reference, TOML format, how to find requirements, path conventions, compliance/ directory structure

## Agent Specs (agent definitions with composition)

- [x] `agents/crypto-dev.agent-spec.json` — base dev agent that team members extend via `includes`. Skills: `duvet-*`, `spec-*`, `rust-*`, `review-*`. Context: `esdk-*`, `duvet-*`
- [x] `agents/duvet-pipeline.agent-spec.json` — orchestrator agent. References duvet-cycle.sop.md. Subagents: duvet-formulate, duvet-implement, duvet-review. Tool restrictions matching current coordinator.json
- [x] `agents/duvet-formulate.agent-spec.json` — discovery agent. References duvet-discover.sop.md. Tool restrictions matching current agent-1-formulate.json
- [x] `agents/duvet-implement.agent-spec.json` — implementation agent. References duvet-implement.sop.md. Tool restrictions matching current agent-2-implement.json
- [x] `agents/duvet-review.agent-spec.json` — review agent. References duvet-review.sop.md. Tool restrictions matching current agent-3-review.json. Pre-spawn hook for make agent-validate

## Agent Prompts (system prompts referenced by agent specs)

- [x] `context/duvet-pipeline-prompt.md` — orchestrator system prompt (role, boundaries, operating modes, sub-coordinator spawning). Slimmed from coordinator.md — the workflow details are in duvet-cycle.sop.md
- [x] `context/duvet-formulate-prompt.md` — discovery agent system prompt (role, boundaries). Workflow details in duvet-discover.sop.md
- [x] `context/duvet-implement-prompt.md` — implementation agent system prompt (role, boundaries, anti-hallucination rules). Workflow details in duvet-implement.sop.md
- [x] `context/duvet-review-prompt.md` — review agent system prompt (role, boundaries, adversarial stance). Workflow details in duvet-review.sop.md

## Totals

- 1 conventions file
- 5 skills (1 with a references/ subdirectory)
- 5 SOPs
- 2 context files
- 5 agent specs
- 4 agent prompt files
- **22 files total**
