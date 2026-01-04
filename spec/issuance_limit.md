# Issuance Limit Proof Specification

## Problem Statement

A financial institution issues tokenized deposits on a public blockchain, where each token represents a claim on off-chain fiat currency held in the institution's internal deposit accounts. The token supply is publicly observable on-chain, but the institution's internal account balances are confidential business information. Without transparency, the institution could over-issue tokens—creating more on-chain claims than it has deposit liabilities—effectively creating unbacked liabilities that would fail in a bank run scenario.

This proof allows the institution to demonstrate that it has not over-issued tokens relative to its deposit base, without revealing the balance of any individual deposit account. The ZK proof serves as a responsibility boundary: it enforces the solvency constraint (tokens ≤ deposits) while preserving the confidentiality of internal account-level data.

## Statement Being Proven

**The total supply of issued tokens does not exceed the sum of all internal deposit account balances, as committed by `deposit_commitment`.**

## Public Inputs

| Input | Type | Description |
|-------|------|-------------|
| `token_supply` | `u64` | Total supply of tokens currently issued on-chain (publicly observable) |
| `deposit_commitment` | `bytes32` | Commitment to the complete set of deposit balances (e.g., Merkle root over all accounts) |

## Private Inputs

| Input | Type | Description |
|-------|------|-------------|
| `balances` | `[u64; N]` | Complete set of deposit account balances included in `deposit_commitment` |
| `merkle_proof` | `bytes[]` | Authentication path proving `balances` correspond to `deposit_commitment` (if commitment scheme requires it) |

## Threat Model

### What the Bank CAN Lie About (Privacy-Preserved)

- Individual account balances
- Number of accounts
- Distribution of balances
- Identity of account holders
- Historical balance changes

### What the Bank MUST NOT Lie About (Enforced by Proof)

- The relationship: `sum(balances) ≥ token_supply`
- That the balances committed to are the actual balances used in the sum

### Attack Vectors Addressed

- **Over-issuance**: Bank cannot issue more tokens than it has deposit backing
- **Fake backing**: Bank cannot use balances inconsistent with the published commitment; however, correctness of the commitment itself is out of scope

### Attack Vectors NOT Addressed

- **Commitment manipulation**: Bank could publish a new commitment with fake data (requires external verification or temporal consistency checks)
- **Selective disclosure**: Bank could omit accounts from the commitment (requires external account enumeration or regulatory oversight)

## What This Proof Enforces

1. **Solvency constraint**: `sum(balances) ≥ token_supply`
2. **Commitment binding**: The balances used in the sum are exactly those committed to in `deposit_commitment`
3. **Non-negativity**: Each balance in `balances` is non-negative (implicit in `u64` type)

## What This Proof Does NOT Enforce

1. **Fiat custody**: Whether the institution actually holds the corresponding fiat currency
2. **Liquidity**: Whether deposits are available for withdrawal (could be illiquid assets)
3. **Temporal consistency**: Whether the same commitment is used across multiple proofs over time
4. **Completeness**: Whether all actual deposit accounts are included in the commitment
5. **Reserve requirements**: Any regulatory capital or reserve ratios beyond 1:1 backing
6. **Account validity**: Whether the deposit accounts are real, active, or legally sound
7. **Double-counting**: Whether the same deposits back multiple token systems (requires cross-system coordination)

## Design Rationale

This proof is intentionally minimal. It enforces a single responsibility: proving non-over-issuance. All other concerns (custody verification, regulatory compliance, temporal auditability) are handled by external systems. This separation of concerns makes the ZK proof easy to reason about, implement correctly in multiple proof systems, and audit for soundness.
