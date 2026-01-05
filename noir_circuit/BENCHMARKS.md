# Noir Circuit Benchmarks

## Overview

This document reports performance characteristics of the issuance limit proof Noir circuit with N=8 accounts.

**Benchmark date**: 2026-01-05
**Noir version**: nargo 1.0.0-beta.17
**Barretenberg version**: 3.0.0-nightly.20251104
**Proving system**: Ultra Honk
**Hardware**: 16 threads (specific CPU details not captured)

## Benchmark Inputs

All measurements use consistent dummy data:

```toml
token_supply = "1000"
deposit_commitment = "0x2ec2053fe442969d333de1c22685807f823bb7f32156e6884650dbbaa309edac"
balances = ["100", "150", "200", "50", "250", "100", "75", "75"]
```

- Sum of balances: 1000
- Relationship: sum(balances) = token_supply (exact match, valid case)

## Circuit Complexity Metrics

| Metric | Value | Source |
|--------|-------|--------|
| **ACIR Opcodes** | 725 | `bb gates` |
| **Circuit Size (gates)** | 28,688 | `bb gates` |
| **Expression Width** | 725 | `nargo info` |
| **Compiled Circuit Size** | 72 KB | `target/noir_circuit.json` |
| **Witness Size** | 3.2 KB | `target/noir_circuit.gz` (compressed) |
| **Verification Key Size** | 3.6 KB | `target/vk/vk` |

### Commands Used

```bash
# Get circuit complexity info
nargo info

# Get gate count from barretenberg
bb gates -b target/noir_circuit.json

# Generate verification key (includes proving key generation time)
bb write_vk -b target/noir_circuit.json -o target/vk
```

## Verification Key Generation

| Metric | Value |
|--------|-------|
| **Proving key generation time** | 43 ms |
| **Peak memory (VK generation)** | 66.09 MiB |

*Note*: VK generation includes proving key computation, which is typically done once per circuit and amortized across many proofs.

## Proving & Verification Benchmarks

**Status**: ✅ **Complete** (resolved via correct witness file path)

### Benchmarking Methodology

- **Warmup**: 1 run (excluded from timing)
- **Measurement**: 3 runs averaged
- **Command**: `bb prove -b ./target/noir_circuit.json -w ./target/noir_circuit.gz -o ./target`
- **Note**: The witness file is `noir_circuit.gz` (not `witness.gz` as initially attempted)

### Results

| Operation | Average Time (3 runs) |
|-----------|----------------------|
| **Proving** | 271 ms |
| **Verification** | 13 ms |

### Artifact Sizes

| Artifact | Size |
|----------|------|
| **Proof** | 16,256 bytes (~16 KB) |
| **Verification Key** | 3,680 bytes (~3.6 KB) |

### Individual Run Times

**Proving**:
- Run 1: 286 ms
- Run 2: 266 ms
- Run 3: 262 ms
- Average: 271 ms

**Verification**:
- Run 1: 14 ms
- Run 2: 14 ms
- Run 3: 13 ms
- Average: 13 ms

## Interpretation of Available Metrics

### Circuit Size (28,688 gates)

- **Meaning**: The number of arithmetic gates in the compiled constraint system
- **Context**: This is a relatively small circuit, well within the range of commodity hardware
- **Comparison baseline**: Typical ZK circuits range from ~10K gates (very simple) to millions (complex DeFi protocols)

### ACIR Opcodes (725)

- **Meaning**: High-level operations in the ACIR intermediate representation before expansion to gates
- **Ratio**: ~40 gates per opcode (28,688 / 725 ≈ 39.6)
- **Note**: The ratio indicates moderate gate expansion during compilation

### File Sizes

- **Circuit (72 KB)**: Reasonable for distribution and storage
- **Witness (3.2 KB)**: Very compact, suitable for on-chain or networked transmission
- **VK (3.6 KB)**: Small enough for on-chain storage in most blockchain systems

## Performance Interpretation

### Proving Time (271 ms)

**What it means**:
- Ultra Honk proof generation takes ~271ms for this 28K-gate circuit
- Significantly slower than Groth16 (arkworks: 5.5ms) for the same statement
- Still "fast enough" for infrequent attestation proofs (daily/weekly cadence)

**Context**:
- For daily proofs: 271ms is negligible user impact
- For high-throughput: Would be a bottleneck (but not the intended use case)
- Trade-off: Ultra Honk offers other advantages (no trusted setup, transparent, potentially better for recursion)

### Verification Time (13 ms)

**What it means**:
- On-chain or client-side verification takes ~13ms
- About 8.6x slower than Groth16 (1.5ms)
- Still well within acceptable range for blockchain or server verification

**Context**:
- Ethereum block time: 12 seconds → verification latency irrelevant
- API response time budget: typically 100-500ms → 13ms is acceptable

### Proof Size (16 KB)

**What it means**:
- Ultra Honk proofs are ~127x larger than Groth16 (128 bytes)
- At Ethereum gas prices (~30 gwei, $3000 ETH):
  - Storing 16KB: ~$1.50-$3 per proof
  - Storing 128 bytes: ~$0.01-$0.02 per proof
- Annual cost difference (daily proofs): ~$500-$1000 vs ~$4-$7

**Trade-off analysis**:
- For high-frequency on-chain storage: Groth16 is clearly superior
- For infrequent attestations: Cost difference is acceptable
- For off-chain verification: Size difference is irrelevant

### What These Numbers Do NOT Imply

1. **Production Performance**:
   - Synthetic benchmarks with dummy data
   - Real-world performance depends on hardware, parallelization, CRS availability

2. **Security Guarantees**:
   - Circuit size does not directly correlate with security level
   - Security depends on cryptographic assumptions (BN254 curve, discrete log hardness)

3. **Scalability Bounds**:
   - Measured with N=8 accounts (small, fixed size)
   - Circuit size grows with N, but relationship not characterized here

## Uncertainty and Variance

### Sources of Variance
- **Proving/verification times**: 3-run average, variance <10%
- **Hardware variability**: Specific CPU/memory configuration not fully documented
- **Toolchain stability**: Using beta Noir (1.0.0-beta.17) with nightly barretenberg
- **Environmental factors**: System load, thermal throttling, background processes not controlled

### Measurement Confidence
- ✅ **High confidence**: Gate counts, file sizes (deterministic)
- ✅ **Good confidence**: Proving/verification times (3-run average, low variance)
- ⚠️ **Medium confidence**: VK generation time (single measurement from earlier run)

### Variance Observed
- **Proving**: 262-286ms range (9% variance)
- **Verification**: 13-14ms range (7% variance)
- Both show acceptable consistency for benchmark purposes

## Comparison to Arkworks

For detailed framework comparison, see [`../COMPARISON.md`](../COMPARISON.md).

### Quick Comparison

| Metric | Noir (Ultra Honk) | Arkworks (Groth16) | Ratio |
|--------|-------------------|---------------------|-------|
| **Proving time** | 271 ms | 5.5 ms | 49x slower |
| **Verification time** | 13 ms | 1.5 ms | 8.6x slower |
| **Proof size** | 16,256 bytes | 128 bytes | 127x larger |
| **Setup time** | 43 ms | 10.6 ms | 4x slower |

### Why Performance Differences Don't Disqualify Noir

For **this use case** (infrequent solvency attestations):
- 271ms proving time is imperceptible to users (daily/weekly proofs)
- 13ms verification is negligible compared to network latency (100-500ms)
- 16KB proof size costs ~$1-3 vs ~$0.01-0.02 for on-chain storage (acceptable for infrequent use)

**The deciding factor remains**: Ease of correctness verification (Noir's 57-line circuit vs arkworks' 155-line manual R1CS) outweighs marginal performance costs.

## Recommendations

### For Production Use
1. These benchmarks are sufficient for capacity planning for this specific use case
2. For high-throughput applications, consider Groth16 (arkworks) for performance
3. Conduct multi-run benchmarks with confidence intervals if precision is critical
4. Measure on representative hardware (e.g., AWS EC2 instance types)

### For This PoC
All metrics are now complete:
- ✅ Circuit complexity characterized
- ✅ End-to-end proving/verification measured
- ✅ Proof sizes known
- ✅ Cross-framework comparison possible

## Reproducing These Benchmarks

```bash
# Ensure correct Noir/barretenberg versions
nargo --version  # Should be 1.0.0-beta.17
bb --version     # Should be 3.0.0-nightly.20251104

# Run benchmark script
./bench_prove.sh
```

**Critical note**: Use `./target/noir_circuit.gz` as the witness file, NOT `./target/witness.gz`.

## Summary

✅ **Complete end-to-end benchmarks**:
- Circuit complexity: 28,688 gates, 725 ACIR opcodes
- Setup time: 43ms (VK generation)
- Proving time: 271ms (average of 3 runs)
- Verification time: 13ms (average of 3 runs)
- Proof size: 16,256 bytes (~16 KB)
- VK size: 3,680 bytes (~3.6 KB)

**Conclusion**: Noir/Ultra Honk is 49x slower at proving than Groth16 but remains "fast enough" for this use case. The ergonomic advantages (57 vs 155 lines, easier audit) justify the performance trade-off for infrequent attestation proofs.
