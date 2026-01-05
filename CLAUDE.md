# CLAUDE.md

## Project Overview

This repository is a finance-oriented Zero-Knowledge Proof PoC.

The goal is to demonstrate the correct use of ZK proofs as a
**responsibility boundary** in a tokenized deposit system.

This is NOT:

- a generic ZK tutorial
- a privacy-first confidential computation demo
- a full production system

This IS:

- a minimal, rigorous design exercise
- focused on real financial constraints
- intended to be understandable and reviewable by senior protocol engineers

## Core Design Principle

Zero-Knowledge Proofs are used as a **responsibility boundary**, not as a
general-purpose confidential computation engine.

In this system:

- Banks retain full control over their internal accounting systems.
- The blockchain does NOT replicate bank ledgers.
- ZK proofs are used to constrain what banks are allowed to claim publicly.

Specifically:

- A bank must be able to prove it is NOT over-issuing tokenized deposits.
- The bank must do so WITHOUT revealing internal account balances.

## Canonical Statement

The canonical statement of this repository is:

> The total supply of issued deposit tokens does not exceed the total
> amount of corresponding bank deposits, without revealing individual
> account balances.

This statement MUST remain identical across all implementations.

## Repository Structure

- `/spec`: Formal specification and threat model
- `/noir`: Noir implementation of the canonical statement
- `/arkworks`: arkworks (Groth16) implementation of the SAME statement
- `/bench`: Benchmarking scripts and measurement results

## Process Rules (Strict)

Claude MUST follow these rules at all times:

1. Work proceeds STRICTLY step by step.
2. Do NOT jump ahead to future steps.
3. Do NOT introduce new features or scope.
4. Do NOT modify the canonical statement.
5. Do NOT implement code unless explicitly instructed.
6. After completing a step, STOP and wait for confirmation.

Violating these rules is considered a failure.

## Implementation Order

All implementations MUST follow this order:

1. `/spec` — specification only
2. `/noir` — minimal working circuit
3. `/bench` — Noir benchmarks
4. `/arkworks` — same statement, same semantics
5. `/bench` — comparative benchmarks
6. `README.md` — conclusions and design trade-offs

## Specification Rules (`/spec`)

When working on `/spec`:

- Focus on threat models and responsibility boundaries.
- Explicitly state what is proven and what is NOT proven.
- Avoid implementation details.
- Avoid generic ZK explanations.
- Assume the reader understands ZK and blockchains.

The specification is the single source of truth.

## Noir Implementation Rules (`/noir`)

When implementing in Noir:

- Keep circuits minimal.
- Prefer clarity over optimization.
- Use finance-realistic variable names.
- Avoid unnecessary abstractions.
- Dummy data is acceptable if semantics are correct.

Noir code exists to make the statement executable and reviewable.

## arkworks Implementation Rules (`/arkworks`)

When implementing in arkworks:

- The statement MUST be semantically identical to the Noir version.
- Do NOT add additional constraints.
- Do NOT change public/private input boundaries.
- Groth16 is the default proving system unless stated otherwise.

arkworks exists for comparison and design validation.

## Benchmarking Rules (`/bench`)

Benchmarks MUST measure:

- Constraint count
- Proving time
- Verification time
- Proof size (when applicable)

Benchmarks are comparative, not absolute.

## What This Project Explicitly Does NOT Do

Claude MUST NOT attempt to add:

- On-chain verifier integration
- Gas cost analysis
- Production-grade key management
- AML/KYC logic beyond placeholders
- UI, APIs, or frontend components
- Multi-bank aggregation logic

If any of these appear necessary, STOP and ask.

## Tone and Output Expectations

- Be precise and minimal.
- Avoid marketing language.
- Avoid hype.
- Prefer explicit trade-offs over optimism.
- Write as if the reader will challenge every assumption.

## Final Reminder

This repository is evaluated primarily on:

- correctness of the specification
- clarity of responsibility boundaries
- quality of technical judgment

Code quantity is secondary to design quality.
