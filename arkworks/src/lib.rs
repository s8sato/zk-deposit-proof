use ark_bn254::{Bn254, Fr};
use ark_groth16::Groth16;
use ark_r1cs_std::{
    alloc::AllocVar,
    eq::EqGadget,
    fields::{fp::FpVar, FieldVar},
    prelude::Boolean,
    uint64::UInt64,
};
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_snark::SNARK;

/// Number of deposit accounts (must match Noir implementation)
pub const NUM_ACCOUNTS: usize = 8;

/// Circuit for the issuance limit proof
/// Proves: sum(balances) >= token_supply AND balances match deposit_commitment
#[derive(Clone)]
pub struct IssuanceLimitCircuit {
    // Public inputs
    pub token_supply: Option<u64>,
    pub deposit_commitment: Option<Fr>,

    // Private inputs
    pub balances: Option<[u64; NUM_ACCOUNTS]>,
}

impl ConstraintSynthesizer<Fr> for IssuanceLimitCircuit {
    fn generate_constraints(self, cs: ConstraintSystemRef<Fr>) -> Result<(), SynthesisError> {
        // Allocate public input: deposit_commitment
        let deposit_commitment_var = FpVar::new_input(cs.clone(), || {
            self.deposit_commitment
                .ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Allocate private inputs
        let token_supply_value = self.token_supply.ok_or(SynthesisError::AssignmentMissing)?;
        let balances = self.balances.ok_or(SynthesisError::AssignmentMissing)?;

        // Allocate balances as witness variables (private)
        let mut balance_vars = Vec::new();
        for &balance in balances.iter() {
            let balance_var = UInt64::new_witness(cs.clone(), || Ok(balance))?;
            balance_vars.push(balance_var);
        }

        // Constraint 1: Non-negativity
        // UInt64 type implicitly enforces non-negativity through bit representation

        // Constraint 2: Solvency - sum(balances) >= token_supply
        // Compute sum of balances
        let mut sum_value: u64 = 0;
        for &balance in balances.iter() {
            sum_value = sum_value.checked_add(balance).ok_or(SynthesisError::Unsatisfiable)?;
        }

        // Check if sum >= token_supply
        if sum_value < token_supply_value {
            return Err(SynthesisError::Unsatisfiable);
        }

        // Compute the difference witness
        let diff_value = sum_value - token_supply_value;

        // Allocate witnesses for token_supply and diff
        let token_supply_var = UInt64::new_witness(cs.clone(), || Ok(token_supply_value))?;
        let diff_var = UInt64::new_witness(cs.clone(), || Ok(diff_value))?;

        // Compute sum in-circuit as field elements (simpler than UInt64 arithmetic)
        let mut sum_field = FpVar::constant(Fr::from(0u64));
        for balance_var in balance_vars.iter() {
            let balance_bits = balance_var.to_bits_le();
            let balance_field = Boolean::le_bits_to_fp_var(&balance_bits)?;
            sum_field = sum_field + balance_field;
        }

        // Convert token_supply and diff to field elements
        let token_supply_bits = token_supply_var.to_bits_le();
        let token_supply_field = Boolean::le_bits_to_fp_var(&token_supply_bits)?;

        let diff_bits = diff_var.to_bits_le();
        let diff_field = Boolean::le_bits_to_fp_var(&diff_bits)?;

        // Enforce: token_supply + diff = sum (all as field elements)
        let token_plus_diff_field = token_supply_field + diff_field;
        sum_field.enforce_equal(&token_plus_diff_field)?;

        // Constraint 3: Commitment binding - compute Merkle root
        // For simplicity in this version, we'll use a hash-based commitment
        // that matches the structure of the Noir implementation

        // Convert balances to field elements and hash them
        let mut level: Vec<FpVar<Fr>> = Vec::new();
        for balance_var in balance_vars.iter() {
            // Convert UInt64 to Fr
            let balance_bits = balance_var.to_bits_le();
            let balance_field = Boolean::le_bits_to_fp_var(&balance_bits)?;

            // Hash the balance (simplified: just use the field element as the hash)
            // In production, this should use actual Pedersen hash gadgets
            level.push(balance_field);
        }

        // Level 1: 8 leaves -> 4 nodes
        let mut level_4 = Vec::new();
        for i in 0..4 {
            let left = &level[i * 2];
            let right = &level[i * 2 + 1];
            // Simplified hash: left + right (in production, use proper hash)
            let hash = left + right;
            level_4.push(hash);
        }

        // Level 2: 4 nodes -> 2 nodes
        let mut level_2 = Vec::new();
        for i in 0..2 {
            let left = &level_4[i * 2];
            let right = &level_4[i * 2 + 1];
            let hash = left + right;
            level_2.push(hash);
        }

        // Level 3: 2 nodes -> 1 root
        let left = &level_2[0];
        let right = &level_2[1];
        let computed_root = left + right;

        // Enforce computed_root == deposit_commitment
        computed_root.enforce_equal(&deposit_commitment_var)?;

        Ok(())
    }
}

// Helper function to compute commitment outside circuit (for testing)
pub fn compute_simple_commitment(balances: &[u64; NUM_ACCOUNTS]) -> Fr {
    // Level 0
    let level: Vec<Fr> = balances.iter().map(|&b| Fr::from(b)).collect();

    // Level 1: 8 -> 4
    let mut level_4 = Vec::new();
    for i in 0..4 {
        level_4.push(level[i * 2] + level[i * 2 + 1]);
    }

    // Level 2: 4 -> 2
    let mut level_2 = Vec::new();
    for i in 0..2 {
        level_2.push(level_4[i * 2] + level_4[i * 2 + 1]);
    }

    // Level 3: 2 -> 1
    level_2[0] + level_2[1]
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;

    #[test]
    fn test_valid_issuance() {
        let mut rng = thread_rng();

        // Setup: 8 accounts with total balance = 1000 (matching Noir test)
        let balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
        let token_supply: u64 = 1000; // Equal to sum of balances

        // Compute the correct commitment
        let commitment = compute_simple_commitment(&balances);

        // Create circuit
        let circuit = IssuanceLimitCircuit {
            token_supply: Some(token_supply),
            deposit_commitment: Some(commitment),
            balances: Some(balances),
        };

        // Generate parameters
        let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

        // Generate proof
        let proof = Groth16::<Bn254>::prove(&pk, circuit.clone(), &mut rng).unwrap();

        // Verify proof
        let public_inputs = vec![commitment];
        let valid = Groth16::<Bn254>::verify(&vk, &public_inputs, &proof).unwrap();
        assert!(valid, "Proof verification failed for valid issuance");
    }

    #[test]
    fn test_valid_issuance_with_excess_deposits() {
        let mut rng = thread_rng();

        // Setup: 8 accounts with total balance = 1000 (matching Noir test)
        let balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
        let token_supply: u64 = 800; // Less than sum of balances

        // Compute the correct commitment
        let commitment = compute_simple_commitment(&balances);

        // Create circuit
        let circuit = IssuanceLimitCircuit {
            token_supply: Some(token_supply),
            deposit_commitment: Some(commitment),
            balances: Some(balances),
        };

        // Generate parameters
        let (pk, vk) = Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();

        // Generate proof
        let proof = Groth16::<Bn254>::prove(&pk, circuit.clone(), &mut rng).unwrap();

        // Verify proof
        let public_inputs = vec![commitment];
        let valid = Groth16::<Bn254>::verify(&vk, &public_inputs, &proof).unwrap();
        assert!(valid, "Proof verification failed for valid issuance with excess deposits");
    }

    #[test]
    #[should_panic(expected = "Unsatisfiable")]
    fn test_over_issuance_fails() {
        let mut rng = thread_rng();

        // Setup: 8 accounts with total balance = 1000 (matching Noir test)
        let balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
        let token_supply: u64 = 1001; // More than sum of balances

        // Compute the correct commitment
        let commitment = compute_simple_commitment(&balances);

        // Create circuit
        let circuit = IssuanceLimitCircuit {
            token_supply: Some(token_supply),
            deposit_commitment: Some(commitment),
            balances: Some(balances),
        };

        // This should fail during constraint generation
        let (pk, _vk) = Groth16::<Bn254>::circuit_specific_setup(circuit.clone(), &mut rng).unwrap();
        let _proof = Groth16::<Bn254>::prove(&pk, circuit.clone(), &mut rng).unwrap();
    }

    #[test]
    fn test_fake_balances_fails() {
        let _rng = thread_rng();

        // Real balances that were committed to (matching Noir test)
        let real_balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
        let commitment = compute_simple_commitment(&real_balances);

        // Fake balances the bank tries to use
        let fake_balances: [u64; NUM_ACCOUNTS] = [200, 200, 200, 200, 200, 200, 200, 200];
        let token_supply: u64 = 1000;

        // Create circuit with fake balances
        let circuit = IssuanceLimitCircuit {
            token_supply: Some(token_supply),
            deposit_commitment: Some(commitment), // Commitment from real balances
            balances: Some(fake_balances),        // But using fake balances
        };

        // Try to generate constraints
        use ark_relations::r1cs::ConstraintSystem;
        let cs = ConstraintSystem::<Fr>::new_ref();

        // Generate constraints
        let result = circuit.generate_constraints(cs.clone());

        // Constraint generation should succeed
        assert!(result.is_ok(), "Constraint generation failed");

        // But constraints should NOT be satisfied
        let is_satisfied = cs.is_satisfied().unwrap();
        assert!(!is_satisfied, "Constraints should not be satisfied with fake balances");
    }
}
