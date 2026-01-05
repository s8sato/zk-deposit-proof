#!/bin/bash
set -e

echo "=== Noir Circuit Proving Benchmarks ==="
echo ""
echo "Configuration:"
echo "  Noir: $(nargo --version | head -1)"
echo "  Barretenberg: $(bb --version)"
echo ""

# Ensure circuit is compiled and witness generated
echo "[1/5] Compiling circuit..."
nargo compile > /dev/null 2>&1
echo "  ✓ Compiled"

echo ""
echo "[2/5] Generating witness..."
nargo execute > /dev/null 2>&1
echo "  ✓ Witness generated"

echo ""
echo "[3/5] Benchmarking proof generation..."

# Warmup run
bb prove -b ./target/noir_circuit.json -w ./target/noir_circuit.gz --write_vk -o ./target > /dev/null 2>&1

# Timed runs
total_time=0
runs=3

for i in $(seq 1 $runs); do
  start=$(date +%s%N)
  bb prove -b ./target/noir_circuit.json -w ./target/noir_circuit.gz -o ./target > /dev/null 2>&1
  end=$(date +%s%N)
  elapsed_ms=$(( (end - start) / 1000000 ))
  echo "  Run $i: ${elapsed_ms} ms"
  total_time=$((total_time + elapsed_ms))
done

avg_prove_ms=$((total_time / runs))
echo "  Average proving time: ${avg_prove_ms} ms"

echo ""
echo "[4/5] Benchmarking verification..."

# Read public inputs and proof
total_verify=0
for i in $(seq 1 $runs); do
  start=$(date +%s%N)
  bb verify -k ./target/vk -p ./target/proof -i ./target/public_inputs > /dev/null 2>&1
  end=$(date +%s%N)
  elapsed_ms=$(( (end - start) / 1000000 ))
  echo "  Run $i: ${elapsed_ms} ms"
  total_verify=$((total_verify + elapsed_ms))
done

avg_verify_ms=$((total_verify / runs))
echo "  Average verification time: ${avg_verify_ms} ms"

echo ""
echo "[5/5] Measuring artifact sizes..."
proof_size=$(wc -c < ./target/proof)
vk_size=$(wc -c < ./target/vk)
echo "  Proof size: ${proof_size} bytes"
echo "  VK size: ${vk_size} bytes"

echo ""
echo "=== Summary ==="
echo "  Average proving time: ${avg_prove_ms} ms"
echo "  Average verification time: ${avg_verify_ms} ms"
echo "  Proof size: ${proof_size} bytes"
echo "  VK size: ${vk_size} bytes"
