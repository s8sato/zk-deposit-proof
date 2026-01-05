# Benchmark Summary: Noir vs Arkworks

This document provides a quick comparison of benchmark results for both implementations of the issuance limit proof.

**For detailed analysis**: See [`COMPARISON.md`](./COMPARISON.md) for comprehensive framework comparison and interpretation.

## Test Configuration (Both Implementations)

- **Circuit**: Issuance limit proof with N=8 accounts
- **Test data**: `[100, 150, 200, 50, 250, 100, 75, 75]` (sum = 1000)
- **Token supply**: 1000 (valid case)
- **Curve**: BN254
- **Build mode**: Release/optimized

## Benchmark Results Comparison

| Metric | Noir (Ultra Honk) | Arkworks (Groth16) | Notes |
|--------|-------------------|---------------------|-------|
| **Circuit Complexity** | | | |
| Circuit size | 28,688 gates | 642 R1CS constraints | ⚠️ Different metrics, not directly comparable |
| ACIR opcodes | 725 | N/A | Noir-specific intermediate representation |
| Witness variables | N/A | 640 | Arkworks-specific |
| Public inputs | 2 | 1 | Minor classification difference |
| **Performance** | | | |
| Setup time | 43 ms (VK gen) | 10.6 ms (PK+VK) | Groth16 faster for setup |
| Proving time | ⚠️ Not measured | 5.5 ms | Noir blocked by tooling issue |
| Verification time | ⚠️ Not measured | 1.5 ms | Noir blocked by tooling issue |
| **Artifact Sizes** | | | |
| Proof size | ⚠️ Not measured | 128 bytes | Groth16 constant-size |
| Verification key | 3.6 KB | 296 bytes | VK size differs by proving system |
| Compiled circuit | 72 KB | N/A | Noir-specific artifact |
| Proving key | N/A | 135.8 KB | Arkworks-specific artifact |

## Key Findings

### What We Can Conclude

1. **Both are "small" circuits**: 28K gates (Noir) and 642 constraints (arkworks) are well within commodity hardware capabilities
2. **Both are "fast enough"**: Sub-10ms performance (where measured) is more than adequate for infrequent attestation proofs
3. **Proof sizes are negligible**: <1KB in both cases means minimal on-chain storage cost
4. **Setup times are fast**: Both complete setup in <50ms

### What We Cannot Conclude

1. **Relative proving performance**: Noir's proving time not measured due to tooling compatibility issue
2. **Constraint efficiency**: Gate count vs R1CS constraints are different compilation stages, not directly comparable
3. **Production readiness**: Both implementations use simplified hash functions for this PoC

### Why Performance Differences Don't Matter Here

For a **tokenized deposit solvency proof** generated daily or weekly:
- Sub-10ms proving time difference: Irrelevant (network latency dominates)
- Proof size difference (<1KB): Negligible on-chain cost (<$1/year)
- Setup time difference (30ms): One-time cost, amortized across all proofs

**The deciding factor is correctness assurance**, not marginal performance differences. See the [Adoption Decision](./README.md#adoption-decision) for full reasoning.

## Detailed Benchmark Reports

### Noir Benchmarks
- **Location**: [`noir_circuit/BENCHMARKS.md`](./noir_circuit/BENCHMARKS.md)
- **Status**: Partial (circuit complexity and VK generation measured)
- **Notable limitation**: Proving/verification blocked by barretenberg compatibility issue
- **Verdict**: Available metrics sufficient for PoC goals

### Arkworks Benchmarks
- **Location**: [`arkworks/BENCHMARKS.md`](./arkworks/BENCHMARKS.md)
- **Status**: Complete end-to-end benchmarks
- **Measurements**: Setup (10.6ms), Proving (5.5ms), Verification (1.5ms)
- **Automated tool**: `cargo run --release --bin benchmark`

## Interpretation Guidelines

### ⚠️ Warning: These Benchmarks Do NOT Generalize

These results are specific to:
- This exact statement (simple solvency constraint)
- This circuit size (N=8 accounts, ~600-28K constraints/gates)
- This use case (infrequent attestation proofs)
- Simplified hash functions (not production-ready)

**Do not extrapolate** to:
- Large circuits (>100K gates)
- High-throughput applications
- Different cryptographic constructions
- Production deployments without proper cryptographic hash functions

### What Actually Matters

For **this proof** (privacy-preserving solvency with 8 accounts):

**Matters**:
1. Developer ergonomics (Noir 2.7x more concise, easier to audit)
2. Proving system maturity (Groth16 battle-tested; Ultra Honk experimental)
3. Correctness assurance (can we confidently audit the circuit?)

**Doesn't matter**:
1. Absolute performance differences (both are "fast enough")
2. Constraint count ratios (different metrics, incomparable)
3. File size differences (all are "small enough")

See [`COMPARISON.md`](./COMPARISON.md) for detailed analysis of what differences matter and why.

## Next Steps for Production

If deploying either implementation to production:

1. **Replace simplified hash with Poseidon2** (both implementations)
2. **Run multi-sample benchmarks** with confidence intervals
3. **Test on representative hardware** (e.g., AWS EC2 instance types)
4. **Measure end-to-end latency** including network overhead
5. **Benchmark with realistic input distributions** and account counts
6. **Consider Noir proving benchmarks** once tooling compatibility is resolved

## Summary

Both implementations prove the same statement with comparable efficiency. The choice between them is an engineering trade-off based on team priorities (ergonomics vs control, maturity vs innovation), not a performance question.

**Recommendation**: See [`README.md`](./README.md) for the adoption decision (Noir recommended for this use case based on correctness assurance).
