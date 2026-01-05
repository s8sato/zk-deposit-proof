# Noir Circuit Implementation

This directory contains the Noir implementation of the issuance limit proof as specified in `/spec/issuance_limit.md`.

## Circuit Overview

The circuit proves the canonical statement:

> **The total supply of issued tokens does not exceed the sum of all internal deposit account balances, as committed by `deposit_commitment`.**

## Public vs Private Inputs

### Public Inputs
- `token_supply: u64` - The total number of tokens issued on-chain (publicly observable)
- `deposit_commitment: Field` - Cryptographic commitment (Merkle root) to the complete set of deposit balances

### Private Inputs
- `balances: [u64; 8]` - Array of 8 individual deposit account balances (kept confidential)

Note: The Merkle authentication path is implicitly handled by recomputing the entire Merkle tree from the balances array.

## Enforced Constraints

The circuit enforces exactly three constraints:

### 1. Non-negativity
Each balance is non-negative. This is **implicitly enforced** by Noir's type system - the `u64` type cannot represent negative values.

### 2. Solvency Constraint
```
sum(balances) >= token_supply
```
The circuit computes the sum of all balances and asserts it is greater than or equal to the token supply. This prevents over-issuance.

### 3. Commitment Binding
```
compute_merkle_root(balances) == deposit_commitment
```
The circuit computes a Merkle root from the private balances and asserts it matches the public commitment. This ensures the bank cannot use fake balances inconsistent with their published commitment.

## Implementation Details

### Merkle Tree Structure
- **Hash function**: Pedersen hash (ZK-friendly, available in Noir stdlib)
- **Tree type**: Binary Merkle tree
- **Number of leaves**: 8 (fixed, must be power of 2)
- **Tree depth**: 3 levels (8 → 4 → 2 → 1)

### Merkle Tree Construction
1. **Level 0**: Hash each balance individually to create 8 leaf nodes
2. **Level 1**: Pairwise hash 8 leaves → 4 intermediate nodes
3. **Level 2**: Pairwise hash 4 nodes → 2 intermediate nodes
4. **Level 3**: Hash final 2 nodes → 1 root

Each balance is hashed individually at the leaf level, then pairs of nodes are hashed together at each subsequent level until a single root is obtained.

## Assumptions and Limitations

### Fixed Account Count
- The circuit is configured for exactly **8 accounts** (defined by `NUM_ACCOUNTS`)
- This is a design choice for clarity and simplicity in this PoC
- Must be a power of 2 for the simple binary Merkle tree implementation
- Can be changed by modifying the `NUM_ACCOUNTS` constant and adjusting Merkle tree logic accordingly

### Simplified Merkle Proof
- The circuit recomputes the **entire Merkle tree** from the full balance array
- This means all balances must be provided as private inputs
- In a production system, you would typically provide only a Merkle authentication path (log₂(N) sibling hashes) instead of the full tree
- This simplification makes the circuit easier to understand and reason about for this PoC

### Hash Function Choice
- Uses **Pedersen hash** (ZK-friendly, available in Noir standard library)
- In production, consider Poseidon2 or other more efficient ZK-native hashes
- The choice doesn't affect the security model - only performance

### No Temporal Consistency
- The circuit proves a statement about a single snapshot in time
- It does NOT enforce that the same commitment is used across multiple proofs
- Temporal auditability is handled externally (out of scope per specification)

## Building and Testing

### Check compilation
```bash
nargo check
```

### Run tests
```bash
nargo test
```

### Tests included
1. `test_valid_issuance` - Token supply equals deposit sum (exact match)
2. `test_valid_issuance_with_excess_deposits` - Token supply less than deposit sum (excess reserves)
3. `test_over_issuance_fails` - Token supply exceeds deposit sum (should fail)
4. `test_fake_balances_fails` - Balances don't match commitment (should fail)

## Circuit Constraints

To view the number of gates and constraints:
```bash
nargo info
```

This will show the circuit size and complexity metrics.
