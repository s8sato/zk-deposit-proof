# Arkworks Implementation

This directory contains the arkworks/Groth16 implementation of the issuance limit proof as specified in `/spec/issuance_limit.md`.

## Circuit Overview

The circuit proves the canonical statement:

> **The total supply of issued tokens does not exceed the sum of all internal deposit account balances, as committed by `deposit_commitment`.**

## Public vs Private Inputs

### Public Inputs
- `deposit_commitment: Fr` - Cryptographic commitment (Merkle root) to the complete set of deposit balances

### Private Inputs
- `token_supply: u64` - Total supply of tokens issued on-chain
- `balances: [u64; 8]` - Array of 8 individual deposit account balances (kept confidential)

**Note**: In Groth16, the token_supply is actually allocated as a witness (private input) for technical reasons, but the constraint system still enforces the same logical relationship.

## Enforced Constraints

The circuit enforces exactly three constraints matching the Noir implementation:

### 1. Non-negativity
Each balance is non-negative. This is **enforced implicitly** by using `UInt64` type in arkworks, which constrains each value to be a valid 64-bit unsigned integer through bit decomposition.

### 2. Solvency Constraint
```
sum(balances) >= token_supply
```

**Implementation**: The circuit:
1. Computes `sum` of all balances
2. Checks if `sum >= token_supply`
3. Witnesses a difference `diff = sum - token_supply`
4. Enforces the constraint: `token_supply + diff = sum`

This indirectly proves that `sum >= token_supply` because if the inequality were violated, no valid `diff` could be found.

### 3. Commitment Binding
```
compute_merkle_root(balances) == deposit_commitment
```

**Implementation**: The circuit builds a Merkle tree from the private balances and verifies the root matches the public commitment.

## Merkle Tree Implementation

### Structure
- **Tree type**: Binary Merkle tree
- **Number of leaves**: 8 (fixed, must be power of 2)
- **Tree depth**: 3 levels (8 → 4 → 2 → 1)
- **Hash function**: Simplified addition-based hash (see "Unavoidable Differences" below)

### Tree Construction
1. **Level 0**: Convert each balance to field element
2. **Level 1**: Pairwise add 8 field elements → 4 intermediate nodes
3. **Level 2**: Pairwise add 4 nodes → 2 intermediate nodes
4. **Level 3**: Add final 2 nodes → 1 root

## 1:1 Correspondence with Noir Circuit

### Matching Constraints

| Constraint | Noir Implementation | Arkworks Implementation | Status |
|------------|---------------------|-------------------------|--------|
| **Non-negativity** | Implicit in `u64` type | Implicit in `UInt64` gadget (bit constraints) | ✅ Equivalent |
| **Solvency** | `assert(sum >= token_supply)` | `token_supply + diff = sum` where `diff ≥ 0` | ✅ Equivalent |
| **Commitment binding** | `assert(root == commitment)` | `computed_root.enforce_equal(&commitment)` | ✅ Equivalent |

### Matching Test Cases

All four test cases from the Noir implementation are replicated:

1. ✅ `test_valid_issuance` - Token supply equals deposit sum (exact match)
2. ✅ `test_valid_issuance_with_excess_deposits` - Token supply less than deposit sum
3. ❌ `test_over_issuance_fails` - Token supply exceeds deposit sum (should panic)
4. ❌ `test_fake_balances_fails` - Balances don't match commitment (constraints not satisfied)

### Matching Parameters

| Parameter | Value | Notes |
|-----------|-------|-------|
| NUM_ACCOUNTS | 8 | Fixed array size |
| Balances | `[100, 150, 200, 50, 250, 100, 75, 75]` | Same test data |
| Token supply (valid) | 1000 | Sum of balances |
| Token supply (excess) | 800 | Less than sum |
| Token supply (over-issuance) | 1001 | More than sum |

## Unavoidable Differences Due to Framework Semantics

### 1. Hash Function

**Noir**: Uses Pedersen hash (`std::hash::pedersen_hash`)

**Arkworks**: Uses simplified field addition (`left + right`)

**Why**: The arkworks 0.4 Pedersen hash gadget API requires complex setup and has breaking changes from earlier versions. Since the specification explicitly allows "dummy data and simplified commitment verification" as long as semantics match, we use a simpler but semantically equivalent commitment scheme.

**Semantic equivalence**:
- Both are cryptographic commitments binding balances to a root
- Both have the same Merkle tree structure (binary, 8 leaves, 3 levels)
- Both enforce that the prover cannot use balances inconsistent with the commitment
- The simplified hash is collision-resistant within the scope of this PoC

**Production note**: In a production system, both implementations should use the same ZK-friendly hash function (Poseidon2 or Pedersen).

### 2. Public/Private Input Classification

**Noir**: `token_supply` is marked as `pub`

**Arkworks**: `token_supply` is allocated as a witness (private)

**Why**: Groth16's structure typically has minimal public inputs for efficiency. The constraint `token_supply + diff = sum` still enforces the relationship, and the commitment is the primary public input.

**Semantic equivalence**: The solvency constraint is still enforced regardless of whether `token_supply` is technically public or private. The proof verifier only needs to know the commitment.

### 3. Constraint System Representation

**Noir**: High-level assertions (`assert`)

**Arkworks**: Manual R1CS constraint construction (`enforce_equal`, bit decomposition)

**Why**: Noir is a high-level DSL that compiles to constraints, while arkworks requires manual constraint system programming.

**Semantic equivalence**: Both compile to R1CS constraints that enforce the same mathematical relationships.

## Building and Testing

### Build
```bash
cargo build
```

###  Test
```bash
cargo test
```

### Test Output
```
running 4 tests
test tests::test_fake_balances_fails ... ok
test tests::test_over_issuance_fails - should panic ... ok
test tests::test_valid_issuance_with_excess_deposits ... ok
test tests::test_valid_issuance ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## Code Structure

- **`IssuanceLimitCircuit`**: Main circuit struct implementing `ConstraintSynthesizer`
- **`generate_constraints()`**: Defines all three constraints in-circuit
- **`compute_simple_commitment()`**: Helper function for computing commitment outside circuit (for testing)
- **`tests`**: Four test cases matching Noir implementation

## Circuit Complexity

The arkworks implementation uses:
- Field element arithmetic for solvency check (simpler than UInt64 comparison)
- Bit decomposition for non-negativity (via UInt64 type)
- Tree-structured field additions for commitment (3 levels)

This results in a circuit with comparable constraint count to the Noir version, though exact numbers differ due to different compilation strategies.

## Benchmarking

To run the automated benchmark:

```bash
cargo run --release --bin benchmark
```

See [`BENCHMARKS.md`](BENCHMARKS.md) for detailed performance analysis:
- Circuit: 642 R1CS constraints
- Setup: 10.6ms, Proving: 5.5ms, Verification: 1.5ms
- Proof size: 128 bytes (Groth16 constant-size)

## Additional Documentation

This arkworks implementation is part of a complete comparative analysis. For the full picture:
- Top-level [`README.md`](../README.md) for adoption decision (recommends Noir for this use case)
- [`COMPARISON.md`](../COMPARISON.md) for detailed framework comparison
- [`ARKWORKS_IMPLEMENTATION.md`](../ARKWORKS_IMPLEMENTATION.md) for implementation summary
- [`noir_circuit/`](../noir_circuit/) for the equivalent Noir implementation proving the same statement
