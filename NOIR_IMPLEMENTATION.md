# Noir Implementation Summary

## Overview

This document explains the Noir circuit implementation of the issuance limit proof defined in `/spec/issuance_limit.md`.

**Location**: `noir_circuit/src/main.nr`

**Statement proven**: The total supply of issued tokens does not exceed the sum of all internal deposit account balances, as committed by `deposit_commitment`.

## Public vs Private Inputs

| Input Type | Name | Type | Description |
|------------|------|------|-------------|
| **Public** | `token_supply` | `u64` | Total supply of tokens currently issued on-chain |
| **Public** | `deposit_commitment` | `Field` | Merkle root commitment to all deposit balances |
| **Private** | `balances` | `[u64; 8]` | Array of 8 individual deposit account balances |

**Why these are public/private:**
- `token_supply` is public because it's observable on the blockchain
- `deposit_commitment` is public because the bank publishes this to allow verification
- `balances` are private to protect customer account information

## Constraints Enforced

The circuit enforces exactly three constraints from the specification:

### 1. Non-negativity of Balances
- **Implementation**: Implicit in Noir's `u64` type
- **Enforcement**: Type system prevents negative values at compile time
- **Code**: No explicit constraint needed

### 2. Solvency Constraint
```noir
let mut sum: u64 = 0;
for i in 0..NUM_ACCOUNTS as u32 {
    sum += balances[i];
}
assert(sum >= token_supply, "Solvency constraint violated");
```
- **What it enforces**: `sum(balances) ≥ token_supply`
- **Why it matters**: Prevents over-issuance of tokens

### 3. Commitment Binding
```noir
let computed_root = compute_merkle_root(balances);
assert(computed_root == deposit_commitment, "Commitment binding violated");
```
- **What it enforces**: The private balances correspond exactly to the public commitment
- **Why it matters**: Prevents the bank from using fake balances inconsistent with their published commitment

## Merkle Tree Implementation

The commitment binding is enforced via a binary Merkle tree:

```
            ROOT (deposit_commitment)
           /     \
         H12     H34
        /  \     /  \
      H1  H2   H3  H4
     / \ / \ / \ / \
    b0 b1.. .. .. b7   (8 balances)
```

**Implementation details:**
- **Hash function**: Pedersen hash (ZK-friendly)
- **Tree structure**: Complete binary tree with 8 leaves
- **Computation**: Bottom-up, left-to-right
- **Levels**: 3 levels of hashing (8→4→2→1)

**Simplified approach:**
- The circuit recomputes the entire tree from all 8 balances
- This is simpler but requires all balances as private inputs
- Production systems would typically use a Merkle proof with only log₂(N) sibling hashes

## Key Assumptions

### 1. Fixed Account Count (N=8)
- Configured for exactly 8 deposit accounts
- Must be a power of 2 for simple binary Merkle tree
- Chosen for clarity in this PoC
- Modifiable by changing `NUM_ACCOUNTS` constant

### 2. No Merkle Proof Optimization
- All balances provided as private inputs
- Full tree recomputed in-circuit
- Trade-off: simplicity over efficiency
- Acceptable for PoC with small N

### 3. Hash Function
- Uses Pedersen hash (available in Noir stdlib)
- ZK-friendly but not the most efficient
- Production might use Poseidon2 or other modern hashes
- Doesn't affect security model, only performance

## Circuit Complexity

```
Expression Width: 725
ACIR Opcodes: 725
Circuit Size: 28,688 gates
```

This is a small, efficient circuit suitable for proof generation with standard hardware. See `noir_circuit/BENCHMARKS.md` for detailed performance metrics.

## What This Does NOT Enforce

As per the specification, this circuit deliberately does NOT enforce:

1. **Fiat custody** - Whether actual fiat exists
2. **Liquidity** - Whether deposits are withdrawable
3. **Temporal consistency** - Whether commitments are reused across time
4. **Completeness** - Whether all accounts are included
5. **Reserve requirements** - Any ratio beyond 1:1
6. **Account validity** - Whether accounts are real/active
7. **Double-counting** - Cross-system deposit usage

These are explicitly out of scope and handled by external systems.

## Testing

Four test cases verify correct behavior:

1. ✅ `test_valid_issuance` - Proves when sum = token_supply
2. ✅ `test_valid_issuance_with_excess_deposits` - Proves when sum > token_supply
3. ❌ `test_over_issuance_fails` - Fails when sum < token_supply (over-issuance)
4. ❌ `test_fake_balances_fails` - Fails when balances don't match commitment

All tests pass successfully.

## Additional Documentation

This Noir implementation is part of a complete comparative analysis demonstrating ZK proof system choice as an engineering trade-off.

**For the complete picture**, see:
- Top-level [`README.md`](../README.md) for adoption decision and framework comparison
- [`COMPARISON.md`](../COMPARISON.md) for detailed Noir vs arkworks analysis
- [`arkworks/`](../arkworks/) for the equivalent Groth16 implementation proving the same statement
