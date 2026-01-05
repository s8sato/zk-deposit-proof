# Zero-Knowledge Proof of Deposit Solvency

## Repository Purpose

This repository demonstrates the use of **Zero-Knowledge Proofs as a responsibility boundary** in a tokenized deposit system.

**Problem**: A financial institution issues blockchain-based tokens representing claims on off-chain fiat deposits. The token supply is publicly observable, but account-level balances are confidential. Without transparency, the institution could over-issue tokens, creating unbacked liabilities that would fail in a bank run.

**Solution**: A ZK proof that enforces the solvency constraint (`sum(deposits) ≥ token_supply`) without revealing individual account balances. The proof serves as a **responsibility boundary**: it separates the confidentiality requirement (private balances) from the accountability requirement (provable non-over-issuance).

**Why ZK**: The proof is the minimal cryptographic commitment that enforces "no over-issuance" while preserving customer privacy. It does NOT attempt to solve custody verification, temporal auditability, or regulatory compliance—those are external concerns handled by other systems.

This is a **design exercise**, not a production system. The goal is to demonstrate how to correctly use ZK to enforce a single, well-defined constraint at a system boundary.

## Canonical Statement

The proof enforces:

> **The total supply of issued tokens does not exceed the sum of all internal deposit account balances, as committed by `deposit_commitment`.**

**Public inputs**:
- `token_supply` (u64): Total issued tokens (observable on-chain)
- `deposit_commitment` (bytes32): Cryptographic commitment to deposit balances

**Private inputs**:
- `balances` ([u64; 8]): Individual account balances (confidential)

**Constraints enforced**:
1. Non-negativity of balances (implicit in u64 type)
2. Solvency: `sum(balances) ≥ token_supply`
3. Commitment binding: `balances` correspond to `deposit_commitment`

**Full specification**: [`/spec/issuance_limit.md`](./spec/issuance_limit.md)

The specification explicitly documents the threat model, attack vectors addressed, and attack vectors NOT addressed (e.g., temporal consistency, fiat custody).

## Implementations

This repository contains **two independent implementations** of the identical statement:

### Noir Implementation

**What it demonstrates**: High-level, type-safe ZK circuit development using a domain-specific language.

- **Core circuit**: 57 lines (109 total with tests)
- **Proving system**: Ultra Honk (via Noir/Barretenberg)
- **Key advantage**: Semantic clarity—the circuit is obviously correct by inspection
- **Limitation**: Newer toolchain (beta stability), proving benchmarks blocked by tooling issue

**Documentation**:
- Implementation details: [`NOIR_IMPLEMENTATION.md`](./NOIR_IMPLEMENTATION.md)
- Circuit documentation: [`noir_circuit/README.md`](./noir_circuit/README.md)
- Benchmarks: [`noir_circuit/BENCHMARKS.md`](./noir_circuit/BENCHMARKS.md)

### Arkworks Implementation

**What it demonstrates**: Low-level, manual R1CS constraint construction with full control over the proving system.

- **Core circuit**: 155 lines (278 total with tests)
- **Proving system**: Groth16 (battle-tested, production-ready)
- **Key advantage**: Mature toolchain, measurable end-to-end performance
- **Limitation**: Manual constraint construction increases risk of accidental semantic drift

**Documentation**:
- Implementation details: [`ARKWORKS_IMPLEMENTATION.md`](./ARKWORKS_IMPLEMENTATION.md)
- Circuit documentation: [`arkworks/README.md`](./arkworks/README.md)
- Benchmarks: [`arkworks/BENCHMARKS.md`](./arkworks/BENCHMARKS.md)

**Critical note**: Both implementations use simplified commitment schemes for this PoC. Production systems would require Poseidon2 or Pedersen hash. This is a PoC limitation, not a framework limitation.

## Benchmarks

### Noir (Complete)
- **Circuit size**: 28,688 gates (compiled)
- **Setup time**: 43 ms (VK generation)
- **Proving time**: 271 ms
- **Verification time**: 13 ms
- **Proof size**: 16,256 bytes (~16 KB)

### Arkworks (Complete)
- **Constraint count**: 642 R1CS constraints
- **Setup time**: 10.6 ms
- **Proving time**: 5.5 ms
- **Verification time**: 1.5 ms
- **Proof size**: 128 bytes (Groth16 constant-size)

**Key finding**: Groth16 is 49x faster at proving and has 127x smaller proofs. However, both are "fast enough" for this use case—271ms proving time is imperceptible for infrequent attestation proofs (daily/weekly), and even 16KB proofs have acceptable on-chain cost (~$1-3 per proof) for infrequent use.

**Full benchmarks**:
- Noir: [`noir_circuit/BENCHMARKS.md`](./noir_circuit/BENCHMARKS.md)
- Arkworks: [`arkworks/BENCHMARKS.md`](./arkworks/BENCHMARKS.md)
- Summary: [`BENCHMARKS.md`](./BENCHMARKS.md)

## Comparative Analysis

**Differences that matter**:
1. **Developer ergonomics**: Noir is 2.7x more concise (57 vs 155 lines), significantly easier to review
2. **Proving system maturity**: Groth16 is battle-tested; Ultra Honk is experimental
3. **Risk of constraint drift**: Noir's type system catches errors at compile time; arkworks relies on manual correctness

**Differences that don't matter**:
1. **Performance**: Both are sub-10ms proving time—irrelevant for infrequent attestations
2. **Proof size**: Both are <1KB—negligible on-chain cost difference
3. **Constraint count**: Different compilation stages, not directly comparable (28K gates vs 642 constraints)

**Critical insight**: The framework choice is an engineering trade-off, not a correctness question. Both prove the **exact same statement** correctly. Neither is objectively superior—they serve different team priorities.

**Full comparison**: [`COMPARISON.md`](./COMPARISON.md)

**Warning**: This comparison applies ONLY to simple arithmetic circuits of this size and structure. It does NOT generalize to large circuits, novel cryptographic constructions, or different use cases.

## Adoption Decision

### Recommended Choice: **Noir**

**For both prototype and production deployment of this specific proof.**

### Reasoning

#### 1. Correctness Assurance is Paramount

For a **finance-oriented solvency proof**, the single most important property is: "Can we be confident this circuit enforces exactly the specification—no more, no less?"

- **Noir**: The 57-line core circuit is readable in ~10 minutes. A reviewer can directly verify it matches [`/spec/issuance_limit.md`](./spec/issuance_limit.md) without deep cryptographic knowledge. The type system enforces non-negativity. The `assert` statements are unambiguous.

- **Arkworks**: The 155-line manual R1CS circuit requires careful constraint analysis. Each `enforce_equal` must be verified to implement the intended semantics. The indirection (witness allocation, field arithmetic, bit decomposition) obscures the logical flow.

**Audit cost**: Noir's circuit can be fully audited in hours. Arkworks' circuit requires days of expert cryptographic review.

For a system where over-issuance could cause financial collapse, **semantic clarity is not negotiable**.

#### 2. Performance Gap is Significant but Irrelevant for This Use Case

**Measured performance**:
- Noir (Ultra Honk): 271ms proving, 13ms verification, 16KB proofs
- Arkworks (Groth16): 5.5ms proving, 1.5ms verification, 128-byte proofs
- **Gap**: Groth16 is 49x faster and produces 127x smaller proofs

**Why this doesn't change the recommendation**:

For a proof generated **daily or weekly**:
- **271ms proving time**: Imperceptible to users in this cadence
- **Network latency (100-500ms)**: Dominates total time
- **User experience**: Completely unaffected

For **on-chain storage cost**:
- Noir: ~$1.50-$3.00 per proof (~$500-$1000/year for daily proofs)
- Arkworks: ~$0.01-$0.02 per proof (~$4-$7/year for daily proofs)
- For a financial institution, this cost difference is acceptable

**The trade-off**: Groth16's performance advantage does not outweigh Noir's correctness assurance (hours of audit time vs days) for this specific use case. For high-throughput applications, the calculation would be different.

#### 3. Proving System Maturity Risk is Manageable

**Concern**: Ultra Honk is newer than Groth16. Does this disqualify Noir for production?

**Assessment**: For a 642-constraint circuit, the proving system risk is low:
- Circuit is simple (arithmetic only, no exotic gates)
- Extensive testing across 4 failure modes validates correctness
- Noir toolchain is production-used (Aztec Network, privacy applications)
- For a circuit this small, even a full proving system audit is feasible

**Mitigation**: Monitor Noir's stability roadmap. If Ultra Honk remains experimental, Noir may support Groth16 or other backends. The circuit code would remain unchanged.

The **certainty** that the circuit matches the spec outweighs the **risk** that the proving system has bugs (which would be caught in testing/audits).

#### 4. Hash Function Limitation Applies to Both

Both implementations use simplified commitment schemes (Noir: Pedersen, Arkworks: field addition). Both require Poseidon2 for production. This is **not a Noir disadvantage**—it's a PoC scope limitation that affects both equally.

### When Arkworks Might Be Preferred

**Different teams might choose arkworks if**:
1. **High-throughput requirements**: Generating thousands of proofs per day where 49x performance difference matters
2. **Cost-sensitive on-chain storage**: Frequent on-chain proof submission where 127x size difference significantly impacts costs
3. **Existing Rust/arkworks expertise**: Team already maintains arkworks circuits, adding one more is lower friction than adopting Noir
4. **Maximum control required**: Need to hand-tune constraint counts or use custom gadgets not available in Noir's stdlib
5. **Groth16 non-negotiable**: Specific requirement for battle-tested Groth16 (e.g., regulatory or insurance requirements)

**But for THIS proof** (infrequent attestations, simple statement): The simplicity of the statement means Noir's abstractions are not limiting, and the use case means performance differences don't matter. The ergonomic advantage dominates.

### Dual Implementation Strategy

**Recommendation**: Retain both implementations in this repository for **different purposes**:

- **Noir**: Primary implementation for deployment
- **Arkworks**: Reference implementation for cross-validation

**Value of dual implementation**:
1. **Specification validation**: If two independent implementations (different languages, different constraint systems) produce equivalent proofs, the specification is likely correct
2. **Portability demonstration**: Shows the statement can be implemented across proving systems
3. **Risk mitigation**: If Noir toolchain becomes unavailable, arkworks provides a fallback

**Maintenance cost**: Both circuits are small (< 300 lines each). Keeping both in sync is minimal overhead.

### Decision Summary

| Use Case | Choice | Rationale |
|----------|--------|-----------|
| **Prototype** | Noir | Faster development, easier iteration |
| **Production (infrequent attestations)** | Noir | Correctness assurance > 49x performance gap for this use case |
| **Production (high-throughput)** | Arkworks | 49x faster proving, 127x smaller proofs matter at scale |
| **Production (fallback)** | Arkworks | If Noir stability concerns materialize |
| **Audit/validation** | Both | Cross-validate specification correctness |

**The deciding factor**: For a **privacy-preserving financial solvency constraint with infrequent attestations**, the ability to confidently audit the circuit in hours (Noir) outweighs Groth16's significant performance advantages (arkworks). The 49x speed difference is objectively large but subjectively irrelevant for daily/weekly proofs.

## Non-Goals and Scope Boundaries

This repository **explicitly does NOT**:

1. ❌ **Prove fiat custody**: The proof shows `sum(balances) ≥ token_supply`, NOT that fiat currency exists in a bank vault
2. ❌ **Enforce temporal consistency**: Each proof is a snapshot; the same commitment could be reused across time (requires external verification)
3. ❌ **Prevent selective disclosure**: The bank could omit accounts from the commitment (requires external enumeration or regulatory oversight)
4. ❌ **Guarantee liquidity**: Deposits could be illiquid or encumbered (out of scope)
5. ❌ **Enforce reserve ratios**: Only proves 1:1 backing, not fractional reserve requirements
6. ❌ **Validate account integrity**: Does not verify accounts are real, active, or legally sound
7. ❌ **Prevent double-counting**: Same deposits could back multiple token systems (requires cross-system coordination)

**Why these are out of scope**: This proof is a **responsibility boundary**. It enforces one constraint (non-over-issuance) and deliberately does NOT attempt to solve orthogonal problems. Each concern above requires different verification mechanisms (auditors, regulators, custody proofs, etc.).

**Separation of concerns**: Mixing multiple responsibilities into a single proof would:
- Make the circuit complex and difficult to audit
- Obscure which property is being enforced
- Create a single point of failure for unrelated concerns

The design philosophy is: **one proof, one responsibility, maximum clarity**.

## How to Read This Repository

**For protocol reviewers** (recommended order):
1. **Specification**: [`spec/issuance_limit.md`](./spec/issuance_limit.md) — understand the canonical statement and threat model
2. **Noir implementation**: [`noir_circuit/src/main.nr`](./noir_circuit/src/main.nr) — verify the circuit matches the spec
3. **Comparative analysis**: [`COMPARISON.md`](./COMPARISON.md) — understand framework trade-offs
4. **This README** — contextualize the adoption decision

**For ZK engineers**:
1. **Specification**: [`spec/issuance_limit.md`](./spec/issuance_limit.md)
2. **Both implementations**: [`noir_circuit/src/main.nr`](./noir_circuit/src/main.nr) and [`arkworks/src/lib.rs`](./arkworks/src/lib.rs)
3. **Implementation docs**: [`NOIR_IMPLEMENTATION.md`](./NOIR_IMPLEMENTATION.md) and [`ARKWORKS_IMPLEMENTATION.md`](./ARKWORKS_IMPLEMENTATION.md)
4. **Benchmarks**: [`COMPARISON.md`](./COMPARISON.md)

**For business stakeholders**:
1. **This README** (Repository Purpose and Adoption Decision sections)
2. **Specification**: [`spec/issuance_limit.md`](./spec/issuance_limit.md) (Problem Statement and What This Proof Does NOT Enforce)
3. **Non-Goals** (this document, above section)

**For interviewers/auditors**:
1. **Specification**: [`spec/issuance_limit.md`](./spec/issuance_limit.md) — what should be proven
2. **Noir circuit**: [`noir_circuit/src/main.nr`](./noir_circuit/src/main.nr) — what IS proven (57 lines, audit in ~1 hour)
3. **Test coverage**: Run `cd noir_circuit && nargo test` — verify all 4 failure modes
4. **Adoption decision**: This README (above section) — understand the reasoning

## Repository Structure

```
.
├── README.md                      # This file
├── spec/
│   └── issuance_limit.md          # Canonical statement (immutable)
├── noir_circuit/
│   ├── src/main.nr                # Noir implementation (57 lines core)
│   ├── README.md                  # Circuit documentation
│   └── BENCHMARKS.md              # Noir performance metrics
├── arkworks/
│   ├── src/lib.rs                 # Arkworks implementation (155 lines core)
│   ├── README.md                  # Circuit documentation
│   ├── BENCHMARKS.md              # Arkworks performance metrics
│   └── benches/benchmark.rs       # Automated benchmark
├── NOIR_IMPLEMENTATION.md         # Noir implementation summary
├── ARKWORKS_IMPLEMENTATION.md     # Arkworks implementation summary
├── BENCHMARKS.md                  # Cross-implementation benchmark summary
└── COMPARISON.md                  # Comprehensive framework comparison
```

## License

This is a research/educational proof-of-concept. No warranty. Use at your own risk.

## Acknowledgments

This repository was created as a **design exercise** to demonstrate:
1. How to correctly scope a ZK proof as a responsibility boundary
2. How to implement the same statement in multiple frameworks
3. How to make reasoned framework choices based on use-case priorities

The goal was not to build a production system, but to show **how to think about** building production ZK systems for finance.
