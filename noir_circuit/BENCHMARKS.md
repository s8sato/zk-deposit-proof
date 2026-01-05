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

**Status**: ⚠️ **Unable to complete**

### Issue Encountered

Attempted to generate proofs using `bb prove` but encountered a memory allocation failure:

```
terminate called after throwing an instance of 'std::bad_alloc'
  what():  std::bad_alloc
```

This error persists across multiple configurations:
- Default ultra_honk scheme
- With `--slow_low_memory` flag
- With explicit VK path

### Root Cause Assessment

This appears to be a compatibility issue between:
- Noir compiler version (1.0.0-beta.17)
- Barretenberg backend version (3.0.0-nightly.20251104)
- Ultra Honk proving system (nightly implementation)

The nightly version of bb may have incomplete or unstable ultra_honk implementation.

### Impact on Benchmarking

The following metrics **could not be measured**:
- ❌ Proving time
- ❌ Verification time
- ❌ Proof size

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

## What These Numbers Do NOT Imply

### 1. Production Performance
- These are synthetic benchmarks with dummy data
- Real-world performance depends on: hardware specs, parallelization, CRS availability, network latency
- Proving time was not measured due to technical limitations

### 2. Security Guarantees
- Circuit size does not directly correlate with security level
- Security depends on the underlying cryptographic assumptions (e.g., BN254 curve, discrete log hardness)
- These metrics say nothing about soundness of the circuit logic

### 3. Comparison to Other Systems
- No cross-framework comparison (e.g., vs arkworks, Halo2, Circom) is provided
- Proving system choice (Ultra Honk) is specific to this toolchain
- Different backends may yield vastly different performance profiles

### 4. Scalability Bounds
- Measured with N=8 accounts (small, fixed size)
- Circuit size grows with N, but relationship not characterized here
- Larger N would significantly impact all metrics

### 5. Completeness
- **Proving and verification benchmarks are incomplete due to tooling issues**
- Cannot draw conclusions about end-to-end proof generation/verification performance
- The most critical metrics for production deployment are missing

## Uncertainty and Variance

### Sources of Variance
- **VK generation time (43ms)**: Single measurement, no statistical sampling
- **Hardware variability**: Specific CPU/memory configuration not documented
- **Toolchain instability**: Using nightly/beta versions of both Noir and bb
- **Environmental factors**: System load, thermal throttling, background processes not controlled

### Measurement Confidence
- ✅ **High confidence**: Gate counts, file sizes (deterministic)
- ⚠️ **Medium confidence**: VK generation time (single run, but stable operation)
- ❌ **No confidence**: Proving/verification metrics (not measured)

## Recommendations

### For Production Use
1. **Do not rely on these benchmarks** for production capacity planning
2. Use stable (non-nightly) toolchain versions
3. Conduct multi-run statistical analysis with confidence intervals
4. Measure on representative hardware (e.g., AWS instance type, validator specs)
5. Test with realistic input distributions and sizes

### For Tool Evaluation
1. **Resolve bb/Noir compatibility** before proving benchmarks
2. Consider testing with stable barretenberg release (not nightly)
3. Evaluate alternative proving systems (e.g., Groth16 via older bb, or UltraPlonk)
4. Document exact hardware specifications for reproducibility

### For This PoC
The available metrics (gate count, circuit size) are **sufficient to demonstrate**:
- The circuit compiles successfully
- The constraint system size is reasonable
- The circuit is structurally sound (passes tests)

The unavailable metrics (proving time, verification time) are **not critical for**:
- Demonstrating correctness of the ZK statement
- Comparing Noir vs arkworks implementations (both will face same measurement challenges)
- Understanding the responsibility boundary enforced by the proof

## Next Steps

If proving/verification benchmarks are required:
1. Test with stable barretenberg backend (non-nightly)
2. Consider using alternative Noir-compatible backends (if available)
3. Or accept that benchmark comparisons will be qualitative (circuit size, constraint count) rather than performance-based

## Summary

✅ **Successfully measured**:
- Circuit complexity: 28,688 gates, 725 ACIR opcodes
- Artifact sizes: 72KB circuit, 3.6KB VK
- VK generation: 43ms

❌ **Not measured** (technical limitation):
- Proving time
- Verification time
- Proof size

The available metrics are sufficient to characterize the circuit's structural properties and support design trade-off discussions, even without end-to-end proving benchmarks.
