# AIM Migration: Shared AI Capabilities for Crypto Tools

**Date**: 2026-04-21
**Status**: Draft
**Author**: lucmcdon

## Problem

The native Rust ESDK project has a sophisticated multi-agent duvet compliance pipeline
(coordinator → formulate → implement → review) with ~3,000 lines of agent prompts,
6 steering files, and 2 memory bank reference docs. This knowledge is trapped in one repo,
understood by one person. The team needs to:

1. Share agent skills (duvet, rust, spec-driven dev) across repos (ESDK, MPL, DBE SDK, S3EC)
2. Share agent skills across team members who are all experimenting simultaneously
3. Improve agent reliability for complex checklists (especially the reviewer)
4. Establish a quality bar for shared prompts without blocking experimentation

## Design Principles

Drawn from studying AWSKMSAIAgents and AVP-ESDK:

### From KMS: Skill = Runbook

Every skill must be followable by a human operator without the LLM. If a human can't
follow it as a manual runbook, it's not good enough to ship. This gives skills a second
audience and a reason to stay maintained.

### From KMS: One Agent Per Domain

Alarm-driven agents are domain experts with scoped skills/context/tools. Human-invoked
agents stay small (~3). Don't build a generalist that loads everything.

### From KMS: Glob-Based Skill Scoping

`common-dev-*`, `klu-*` patterns let you add skills without updating agent specs.
Never use `skillNames: ["*"]`.

### From AVP-ESDK: SOPs as Orchestration Contracts

SOPs define the workflow. Each step has explicit MUST/SHOULD/MAY constraints.
Progress is tracked via state files (state.md, progress.md). Steps produce artifacts
that later steps consume — creating natural checkpoints.

### From AVP-ESDK: Conventions File

Shared definitions (severity levels, smart quotes, duvet commands, annotation placement)
live in one `conventions.md` referenced by all SOPs. No duplication across prompts.

### From AVP-ESDK: Orchestrator Never Implements

The orchestrator delegates via SOPs, tracks state, and runs commit gates.
It never writes code, fixes annotations, or debugs errors. "Small fix, faster to do
it myself" is the trigger for violation.

### From AVP-ESDK: Cycle-Based File Flow

Each work item gets its own directory with briefs, plans, reviews, and state tracking.
This is the same pattern as the current `.agent-reports/{SLUG}/` approach but formalized.

### Our Addition: Personal Branches for Experimentation

Everyone works on their own branch of the shared package during the experimentation phase.
Skills converge to mainline via CR when they stabilize.

## Architecture

### Two Packages

```
CryptoToolsAISkills/              ← Shared AIM package (new Brazil package)
  Config                          ← aim-build, AIMBuild 1.0
  agents/
    crypto-dev.agent-spec.json    ← Base dev agent (team extends via includes)
    duvet-pipeline.agent-spec.json ← Multi-agent duvet pipeline orchestrator
  skills/
    duvet-annotation-patterns/    ← How to write duvet annotations (any language)
    duvet-workflow/                ← Phased duvet workflow (extract → implement → verify)
    spec-driven-dev/              ← Read spec → find requirements → implement
    rust-conventions/              ← Rust style, error handling, testing patterns
    review-checklist/              ← Annotation review methodology (context-reset technique)
  agent-sops/
    conventions.md                ← Shared definitions (severity, smart quotes, commands)
    duvet-cycle.sop.md            ← Full discover → implement → review → commit cycle
    duvet-implement.sop.md        ← Implementation with annotations (from Agent 2)
    duvet-review.sop.md           ← Step-by-step annotation review (from Agent 3)
    duvet-discover.sop.md         ← Coverage gap analysis (from Agent 1)
    duvet-commit.sop.md           ← Commit gates (test, coverage, lint, duvet)
  context/
    esdk-overview.md              ← High-level ESDK concepts (shared across repos)
    duvet-tool-reference.md       ← Duvet command reference, TOML format, path conventions

aws-encryption-sdk/               ← This project repo (existing)
  .kiro/
    steering/
      00-project-context.md       ← Repo-specific (stays)
      02-file-locations.md        ← Repo-specific (stays)
      03-testing-strategy.md      ← Repo-specific (stays)
      05-pr-template.md           ← Repo-specific (stays)
    agents/                       ← Pipeline agent configs (migrated to AIM format)
    settings/
      mcp.json                    ← MCP config if needed
```

### What Moves to the Shared Package

| Current Location | Destination | AIM Artifact Type |
|---|---|---|
| `llm_context/memory-bank/duvet-patterns-rust.md` | `skills/duvet-annotation-patterns/SKILL.md` | Skill (on-demand) |
| `.kiro/steering/01-duvet-requirements.md` | `skills/duvet-workflow/SKILL.md` | Skill (on-demand) |
| `.kiro/steering/04-spec-annotation-principles.md` | Folded into `skills/duvet-annotation-patterns/` | Skill (on-demand) |
| `llm_context/memory-bank/duvet-patterns.md` | Folded into `skills/duvet-annotation-patterns/` | Skill (on-demand) |
| Agent 2 annotation rules (~400 lines) | `agent-sops/duvet-implement.sop.md` | SOP |
| Agent 3 review checklist (~500 lines) | `agent-sops/duvet-review.sop.md` | SOP |
| Agent 1 discovery process (~300 lines) | `agent-sops/duvet-discover.sop.md` | SOP |
| Coordinator orchestration (~400 lines) | `agent-sops/duvet-cycle.sop.md` | SOP |
| Smart quotes, path conventions, error patterns | `agent-sops/conventions.md` | SOP reference |

### What Stays in the Project Repo

| File | Why |
|---|---|
| `00-project-context.md` | Repo-specific: what this project is, HKeyring scope, deliverables |
| `02-file-locations.md` | Repo-specific: directory structure, build commands |
| `03-testing-strategy.md` | Repo-specific: test categories, running tests |
| `05-pr-template.md` | Repo-specific: PR format |
| `.agent-reports/` | Pipeline state (per-run, not shared) |
| `llm_context/work-queue*.md` | Pipeline state (per-run, not shared) |

### Agent Composition

The project repo's pipeline agents compose from the shared package:

```jsonc
// agents/duvet-pipeline.agent-spec.json (in CryptoToolsAISkills)
{
  "schemaVersion": "1",
  "name": "duvet-pipeline",
  "config": {
    "description": "Multi-agent duvet compliance pipeline orchestrator",
    "systemPrompt": "{{aim:include:context/duvet-pipeline-prompt.md}}",
    "model": "claude-opus-4.6-1m"
  },
  "dependencies": {
    "skills": {
      "skillNames": ["duvet-*", "spec-driven-*", "rust-*", "review-*"]
    },
    "agentSops": {
      "agentSopNames": ["duvet-*"]
    },
    "context": {
      "contextNames": ["esdk-*", "duvet-*"]
    },
    "subagents": {
      "agentNames": ["duvet-formulate", "duvet-implement", "duvet-review"]
    }
  },
  "clientConfig": {
    "kiroCli": {
      "tools": ["read", "use_subagent", "shell", "thinking"],
      "allowedTools": ["read", "thinking"],
      "hooks": {
        "preToolUse": [
          {
            "command": "{{aim:filepath:agents/hooks/mainline-guardrail.sh}}",
            "matcher": "execute_bash",
            "timeout_ms": 2000
          }
        ]
      }
    }
  }
}
```

Each sub-agent (formulate, implement, review) is also an AIM agent spec with:
- Scoped tool permissions via `clientConfig.kiroCli.toolsSettings`
- System prompt that references the corresponding SOP
- The SOP provides the step-by-step workflow; the prompt provides role identity and boundaries

### SOP Structure (Following AVP-ESDK Patterns)

Each SOP follows this structure:

```markdown
# SOP Name

## Overview
What this SOP does and when to use it.
Scope: what it covers and what it does NOT cover.
Shared definitions: see conventions.md.

## Parameters
| Parameter | Required | Type | Description |
|---|---|---|---|

## Steps

### 1. Step Name
Description.

**Constraints:**
- You MUST ...
- You SHOULD ...
- You MUST NOT proceed if ... because ...

**Failure handling:** What to do if this step fails.

### 2. Next Step
...

## Output
What artifacts this SOP produces and where they go.
```

### Key SOP: duvet-review.sop.md

This is the highest-impact rewrite. Current Agent 3 is ~1100 lines of rules
evaluated simultaneously. The SOP decomposes it into sequential steps:

1. **Load Context** — Read work-item, implementation-summary, compliance TOML, patterns reference
2. **Check Annotation Stacking** — Scan for 3+ annotations before a single code line. Hard gate: any violation = CHANGES_REQUESTED, skip remaining steps
3. **Check Quote Accuracy** — Character-for-character comparison against TOML. Smart quote verification
4. **Check Placement Quality** — Context-reset technique per annotation block. Cross-reference verification
5. **Check Test Coverage** — Implementation↔test annotation pairing. V1/V2 coverage. Raw byte parsing
6. **Run Live Verification** — `duvet query` / `make duvet` to confirm current state
7. **Anti-Rationalization Check** — Review own findings for "wrong but acceptable" patterns
8. **Write Output** — CHANGES_REQUESTED or APPROVED based on findings

Each step writes findings to `notes-agent-3.md`. Steps 2-6 each have a PASS/FAIL gate.
Any FAIL routes directly to step 8 with CHANGES_REQUESTED.

### Key SOP: duvet-cycle.sop.md

The orchestration SOP (replacing coordinator.md). Follows AVP-ESDK's cycle pattern:

1. **Validate Inputs** — Spec exists, crate exists, duvet config valid, create rollback tag
2. **Discovery** — Dispatch duvet-discover.sop.md, receive work-item.md
3. **Implementation** — Dispatch duvet-implement.sop.md, receive implementation-summary.md
4. **Pre-Review Gate** — Annotation count, test count, compilation check
5. **Review** — Dispatch duvet-review.sop.md, receive review decision
6. **Handle Result** — Route based on APPROVED/CHANGES_REQUESTED/TESTS_FAILED
7. **Commit Gates** — Strict order: cargo check → cargo test → cargo clippy → duvet → git commit
8. **Re-Discover** — Check for remaining gaps, loop or complete

State tracked in `.agent-reports/{slug}/state.md` with markdown checklists.
Orchestrator creates checkpoints between steps.

### conventions.md

Shared definitions referenced by all SOPs (following AVP-ESDK pattern):

- Severity levels (BLOCKER / WARNING / NIT) with clear definitions
- Smart quote rules and byte-for-byte matching
- Duvet command reference (make duvet_extract, make duvet_report, duvet query)
- Annotation path conventions (specification/ prefix, not aws-encryption-sdk-specification/)
- Error construction patterns (ser_err(), let-else with try_from)
- Trust boundary definitions (what the orchestrator can/cannot do)
- Valid annotation types and when to use each

## Migration Plan

### Phase 1: Create the Shared Package (Week 1)

1. Create `CryptoToolsAISkills` Brazil package with AIM build
2. Set up directory structure (agents/, skills/, agent-sops/, context/)
3. Write `conventions.md` — extract shared definitions from current prompts
4. Convert `duvet-patterns-rust.md` → `skills/duvet-annotation-patterns/SKILL.md`
5. Convert `01-duvet-requirements.md` → `skills/duvet-workflow/SKILL.md`
6. Write `context/esdk-overview.md` and `context/duvet-tool-reference.md`
7. Test locally: `aim agents install --local . --watch`

**Deliverable:** Shared package with skills and context. No SOPs or agents yet.
Team members can install and use the skills in their own repos.

### Phase 2: Write SOPs (Week 2)

1. Write `duvet-review.sop.md` — decompose Agent 3's 1100-line prompt into sequential steps
2. Write `duvet-implement.sop.md` — decompose Agent 2's prompt into SOP format
3. Write `duvet-discover.sop.md` — decompose Agent 1's prompt into SOP format
4. Write `duvet-cycle.sop.md` — orchestration SOP replacing coordinator.md
5. Write `duvet-commit.sop.md` — commit gate sequence
6. Test each SOP individually with manual invocation

**Deliverable:** SOPs that can be invoked standalone. Test reviewer SOP against
known-good and known-bad annotation examples to measure reliability improvement.

### Phase 3: Migrate Pipeline Agents (Week 3)

1. Convert `.kiro/agents/*.json` → AIM agent-spec.json format
2. Move system prompts to `context/` with `{{aim:include:...}}` references
3. Wire agent specs to reference SOPs and skills from the shared package
4. Preserve tool sandboxing via `clientConfig.kiroCli.toolsSettings`
5. Test full pipeline: coordinator → formulate → implement → review → commit
6. Compare results against pre-migration baseline

**Deliverable:** Full pipeline running on AIM. Old `.kiro/agents/` and
`llm_context/agent-prompts/` can be removed.

### Phase 4: Team Adoption (Week 4)

1. Set up personal branches workflow — each team member gets a branch
2. Onboard one teammate: install package, try skills, give feedback
3. Write `rust-conventions/SKILL.md` and `spec-driven-dev/SKILL.md`
4. Have teammate contribute a skill from their own work
5. Set up version set and pipeline for the shared package
6. Configure package permissions for team access

**Deliverable:** At least 2 people using the shared package. Personal branches
workflow proven. Pipeline publishing skills to the team.

### Phase 5: Cross-Repo Validation (Week 5+)

1. Have another SDK repo (MPL, DBE, or S3EC) consume skills via `$from`
2. Validate that duvet skills work for a non-ESDK spec
3. Collect feedback, iterate on skill content
4. Promote stabilized skills from personal branches to mainline via CR
5. Publish to AIM registry for broader discoverability

**Deliverable:** Skills proven across multiple repos. Promotion story complete.

## Patterns to Follow

### Naming Convention

| Prefix | Category | Examples |
|---|---|---|
| `duvet-*` | Duvet annotation and compliance | `duvet-annotation-patterns`, `duvet-workflow` |
| `rust-*` | Rust language conventions | `rust-conventions`, `rust-testing` |
| `spec-*` | Spec-driven development | `spec-driven-dev`, `spec-reading` |
| `review-*` | Code review methodology | `review-checklist` |

### SOP Naming Convention

| Pattern | Category | Examples |
|---|---|---|
| `duvet-*.sop.md` | Duvet pipeline steps | `duvet-cycle`, `duvet-review`, `duvet-implement` |

### Skill Size Guidelines

- SKILL.md body: <500 lines (following KMS team's guideline)
- If approaching 500 lines, add `references/` subdirectory with detailed docs
- Agent loads SKILL.md on-demand; references load only when the skill needs them

### SOP Step Guidelines (from AVP-ESDK)

- Each step has explicit MUST/SHOULD/MAY constraints
- Each step produces an artifact (file) that later steps consume
- Hard gates between steps: "MUST NOT proceed if X fails"
- Failure handling per step: what to do when things go wrong
- State tracking via markdown checklists in state.md

### Agent Boundaries (from AVP-ESDK)

- Orchestrator NEVER writes implementation code
- Each agent has a clear "DOES / NEVER DOES" table
- Tool permissions enforced via clientConfig.kiroCli.toolsSettings
- Sub-agents are always fresh (no session reuse across work items)

## Success Criteria

1. Shared package installed and used by ≥2 team members
2. ≥1 other repo consuming skills via composition ($from)
3. Reviewer SOP demonstrably more consistent than current 1100-line prompt
   (measured by: fewer review rounds needed, fewer missed annotation issues)
4. Team can experiment on personal branches without blocking each other
5. Skills pass brazil-build validation
6. Full duvet pipeline runs end-to-end on AIM

## Risks

| Risk | Mitigation |
|---|---|
| Migration breaks working pipeline | Phase 3 runs new pipeline in parallel with old before cutting over |
| SOPs don't improve reliability | Phase 2 tests SOPs against known examples before full migration |
| Team doesn't adopt | Phase 4 starts with one teammate, not a big-bang rollout |
| AIM tooling has gaps | clientConfig.kiroCli is the escape hatch for Kiro-specific config |
| Experimentation slows down | Personal branches preserve full autonomy during experimentation |
