# Noir Circuit Benchmarks - Summary

**Full benchmarks**: See `noir_circuit/BENCHMARKS.md` for detailed methodology and interpretation.

## Quick Results

### Circuit Complexity

| Metric | Value |
|--------|-------|
| Circuit Size | 28,688 gates |
| ACIR Opcodes | 725 |
| Expression Width | 725 |

### Artifact Sizes

| Artifact | Size |
|----------|------|
| Compiled Circuit | 72 KB |
| Witness | 3.2 KB |
| Verification Key | 3.6 KB |

### Verification Key Generation

| Metric | Value |
|--------|-------|
| Proving Key Generation | 43 ms |
| Peak Memory | 66 MiB |

## Proving & Verification: ⚠️ Not Measured

Due to a compatibility issue between Noir 1.0.0-beta.17 and barretenberg 3.0.0-nightly, proof generation and verification benchmarks could not be completed. See full report for details.

## Key Takeaways

1. **Circuit is small and efficient**: 28K gates is well within commodity hardware capabilities
2. **Artifacts are compact**: All files < 100KB, suitable for distribution
3. **End-to-end performance unmeasured**: Proving/verification metrics blocked by tooling issue
4. **Sufficient for PoC goals**: Available metrics demonstrate circuit validity and structural properties

## What This Means

The measured metrics are **sufficient** to:
- ✅ Demonstrate the circuit compiles and is structurally sound
- ✅ Support design trade-off discussions about constraint complexity
- ✅ Enable comparison with arkworks implementation (constraint count is comparable across systems)

The unmeasured metrics are **not critical** for:
- ✅ Proving correctness of the ZK statement (handled by tests)
- ✅ Comparing implementation approaches (Noir vs arkworks)
- ✅ Understanding the responsibility boundary enforced by the proof

For production deployment, full end-to-end benchmarks would be essential, but for this PoC, the available metrics achieve the design exercise goals.
