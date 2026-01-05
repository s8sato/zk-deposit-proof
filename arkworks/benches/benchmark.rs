use arkworks::{compute_simple_commitment, IssuanceLimitCircuit, NUM_ACCOUNTS};
use ark_bn254::{Bn254, Fr};
use ark_groth16::Groth16;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
use ark_snark::SNARK;
use ark_serialize::CanonicalSerialize;
use rand::thread_rng;
use std::time::Instant;

fn main() {
    println!("=== Arkworks Circuit Benchmarks ===\n");

    let mut rng = thread_rng();

    // Standard test data (matching Noir)
    let balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
    let token_supply: u64 = 1000;
    let commitment = compute_simple_commitment(&balances);

    // Create circuit
    let circuit = IssuanceLimitCircuit {
        token_supply: Some(token_supply),
        deposit_commitment: Some(commitment),
        balances: Some(balances),
    };

    println!("[1/6] Analyzing constraint system...");
    let cs = ConstraintSystem::<Fr>::new_ref();
    circuit.clone().generate_constraints(cs.clone()).unwrap();

    let num_constraints = cs.num_constraints();
    let num_witness_vars = cs.num_witness_variables();
    let num_instance_vars = cs.num_instance_variables();

    println!("  Constraints: {}", num_constraints);
    println!("  Witness variables: {}", num_witness_vars);
    println!("  Instance variables: {}", num_instance_vars);
    println!();

    println!("[2/6] Generating proving/verification keys...");
    let start = Instant::now();
    let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
    let setup_time = start.elapsed();
    println!("  Setup time: {:?}", setup_time);
    println!();

    println!("[3/6] Measuring proof generation...");
    // Warmup
    let _ = Groth16::<Bn254>::prove(&pk, circuit.clone(), &mut rng).unwrap();

    // Actual measurement
    let start = Instant::now();
    let proof = Groth16::<Bn254>::prove(&pk, circuit.clone(), &mut rng).unwrap();
    let prove_time = start.elapsed();
    println!("  Proving time: {:?}", prove_time);
    println!();

    println!("[4/6] Measuring verification...");
    let public_inputs = vec![commitment];

    // Warmup
    let _ = Groth16::<Bn254>::verify(&vk, &public_inputs, &proof).unwrap();

    // Actual measurement
    let start = Instant::now();
    let valid = Groth16::<Bn254>::verify(&vk, &public_inputs, &proof).unwrap();
    let verify_time = start.elapsed();
    println!("  Verification time: {:?}", verify_time);
    println!("  Proof valid: {}", valid);
    println!();

    println!("[5/6] Measuring artifact sizes...");

    // Proof size
    let mut proof_bytes = Vec::new();
    proof.serialize_compressed(&mut proof_bytes).unwrap();
    println!("  Proof size: {} bytes", proof_bytes.len());

    // VK size
    let mut vk_bytes = Vec::new();
    vk.serialize_compressed(&mut vk_bytes).unwrap();
    println!("  Verification key size: {} bytes", vk_bytes.len());

    // PK size
    let mut pk_bytes = Vec::new();
    pk.serialize_compressed(&mut pk_bytes).unwrap();
    println!("  Proving key size: {} bytes", pk_bytes.len());
    println!();

    println!("[6/6] Summary");
    println!("  Circuit:");
    println!("    - Constraints: {}", num_constraints);
    println!("    - Witness vars: {}", num_witness_vars);
    println!("    - Instance vars: {}", num_instance_vars);
    println!("  Performance:");
    println!("    - Setup: {:?}", setup_time);
    println!("    - Prove: {:?}", prove_time);
    println!("    - Verify: {:?}", verify_time);
    println!("  Artifacts:");
    println!("    - Proof: {} bytes", proof_bytes.len());
    println!("    - VK: {} bytes", vk_bytes.len());
    println!("    - PK: {} bytes", pk_bytes.len());
    println!();

    println!("=== Benchmark Complete ===");
}
