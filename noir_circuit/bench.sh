#!/usr/bin/env bash
# Benchmark script for Noir issuance limit proof circuit
# Usage: ./bench.sh

set -e

echo "=== Noir Circuit Benchmarks ==="
echo ""

# Check tooling versions
echo "Environment:"
echo "  Nargo: $(nargo --version | head -1)"
echo "  Barretenberg: $(bb --version)"
echo ""

# Step 1: Get circuit info from Noir
echo "[1/5] Getting circuit complexity info..."
nargo info
echo ""

# Step 2: Execute to generate witness
echo "[2/5] Executing circuit to generate witness..."
nargo execute
echo ""

# Step 3: Compile to ACIR
echo "[3/5] Compiling circuit to ACIR..."
nargo compile
echo ""

# Step 4: Get gate count from barretenberg
echo "[4/5] Analyzing circuit with barretenberg..."
bb gates -b target/noir_circuit.json
echo ""

# Step 5: Generate verification key (includes proving key generation)
echo "[5/5] Generating verification key..."
rm -rf target/vk
bb write_vk -b target/noir_circuit.json -o target/vk
echo ""

# Report file sizes
echo "=== Artifact Sizes ==="
echo "  Compiled circuit: $(ls -lh target/noir_circuit.json | awk '{print $5}')"
echo "  Witness: $(ls -lh target/noir_circuit.gz | awk '{print $5}')"
echo "  Verification key: $(ls -lh target/vk/vk | awk '{print $5}')"
echo ""

# Note about proving/verification
echo "=== Proving & Verification ==="
echo "⚠️  Proof generation is currently blocked by a compatibility issue"
echo "    between Noir 1.0.0-beta.17 and barretenberg 3.0.0-nightly."
echo ""
echo "    If using a compatible version, run:"
echo "      bb prove -b target/noir_circuit.json -w target/noir_circuit.gz -o target/proof"
echo "      bb verify -k target/vk -p target/proof"
echo ""

echo "=== Benchmark Complete ==="
echo "See BENCHMARKS.md for detailed analysis and interpretation."
