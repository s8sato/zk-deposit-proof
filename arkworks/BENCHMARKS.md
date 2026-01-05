# Arkworks Circuit Benchmarks

## Overview

Performance measurements for the arkworks/Groth16 implementation of the issuance limit proof.

**Test Configuration**:
- Circuit: Issuance limit proof (8 accounts)
- Test data: `[100, 150, 200, 50, 250, 100, 75, 75]` (sum = 1000)
- Token supply: 1000
- Curve: BN254
- Proving system: Groth16
- Build mode: Release (optimized)

## Results

### Circuit Complexity

| Metric | Value |
|--------|-------|
| R1CS Constraints | 642 |
| Witness Variables | 640 |
| Public Inputs | 1 (deposit_commitment) |
| Private Inputs | 9 (token_supply + 8 balances) |

### Performance

| Operation | Time |
|-----------|------|
| Setup (PK + VK generation) | 10.6 ms |
| Proving | 5.5 ms |
| Verification | 1.5 ms |

### Artifact Sizes

| Artifact | Size |
|----------|------|
| Proof | 128 bytes |
| Verification Key | 296 bytes |
| Proving Key | 135.8 KB |

## Interpretation

### Circuit Complexity (642 constraints)

**What it means**:
- This is the raw R1CS constraint count before backend compilation
- Comparable to ~600-700 gates in other ZK systems
- Very small circuit, well within commodity hardware capabilities

**Breakdown**:
- Non-negativity: ~64 constraints (8 balances × 8 bits each for range checks)
- Solvency: ~20 constraints (field arithmetic for sum and comparison)
- Commitment: ~558 constraints (Merkle tree computation with field additions)

### Performance Metrics

**Setup (10.6 ms)**:
- One-time cost per circuit
- Includes both proving key and verification key generation
- Fast because circuit is small

**Proving (5.5 ms)**:
- Per-proof cost
- Includes witness generation and proof computation
- Sub-millisecond would be achievable with further optimization
- Fast enough for any production use case (daily/weekly attestations)

**Verification (1.5 ms)**:
- Per-verification cost
- Very fast (suitable for on-chain verification if needed)
- Groth16 verification time is constant regardless of circuit size

### Proof Size (128 bytes)

**What it means**:
- Groth16 proofs are constant-size (always 128 bytes for BN254)
- Tiny footprint for on-chain storage
- At Ethereum gas prices, storing one proof costs ~$0.01-0.10
- Annual cost for daily proofs: ~$4-40

### Artifact Sizes

**Verification Key (296 bytes)**:
- Stored once on-chain (if doing on-chain verification)
- Cost: ~$0.02-0.20 one-time
- Small enough for any blockchain

**Proving Key (135.8 KB)**:
- Kept off-chain by the prover
- Not a concern for on-chain systems
- Small enough to distribute easily

## Running the Benchmark

```bash
cargo run --release --bin benchmark
```

## Comparison to Noir

See `/COMPARISON.md` for detailed comparison with the Noir implementation.

**Quick summary**:
- Noir: 28,688 gates (different metric, not directly comparable)
- Arkworks: 642 constraints
- Both prove the same statement
- Both are "small" and "fast enough" for this use case
- Noir is more ergonomic; arkworks offers more control

## Caveats

1. **Single measurement**: These are single-run measurements, not statistical averages
2. **Simplified hash**: This benchmark uses field addition, not cryptographic hash (acceptable for PoC)
3. **Small circuit**: Performance characteristics may differ for larger circuits
4. **No optimization**: Circuit was not optimized; further improvements possible
5. **Hardware-specific**: Run on specific machine; results may vary on other hardware

## Next Steps

For production deployment:
1. Replace field addition with Poseidon2 hash
2. Run multi-sample benchmarks with confidence intervals
3. Test on representative hardware (e.g., AWS instance types)
4. Measure end-to-end latency including network overhead
5. Benchmark with realistic input distributions
