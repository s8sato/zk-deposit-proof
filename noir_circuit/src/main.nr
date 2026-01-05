use std::hash::pedersen_hash;

// Number of deposit accounts (must be power of 2 for simple Merkle tree)
global NUM_ACCOUNTS: u32 = 8;

fn main(
    // Public inputs
    token_supply: pub u64,
    deposit_commitment: pub Field,

    // Private inputs
    balances: [u64; NUM_ACCOUNTS],
) {
    // Constraint 1: Non-negativity
    // (Implicit in u64 type - Noir enforces this)

    // Constraint 2: Solvency - sum(balances) >= token_supply
    let mut sum: u64 = 0;
    for i in 0..NUM_ACCOUNTS as u32 {
        sum += balances[i];
    }
    assert(sum >= token_supply, "Solvency constraint violated: total deposits < token supply");

    // Constraint 3: Commitment binding - balances correspond to deposit_commitment
    // Build Merkle tree from balances and verify root matches deposit_commitment
    let computed_root = compute_merkle_root(balances);
    assert(computed_root == deposit_commitment, "Commitment binding violated: balances do not match deposit_commitment");
}

// Compute Merkle root from array of balances
// Uses a simple binary Merkle tree with Pedersen hash
fn compute_merkle_root(balances: [u64; NUM_ACCOUNTS]) -> Field {
    // Level 0: Hash each balance to create leaf nodes
    let mut level: [Field; NUM_ACCOUNTS] = [0; NUM_ACCOUNTS];
    for i in 0..NUM_ACCOUNTS as u32 {
        // Hash balance as a field element
        level[i] = pedersen_hash([balances[i] as Field]);
    }

    // Level 1: 8 leaves -> 4 nodes
    let mut level_4: [Field; 4] = [0; 4];
    for i in 0..4 {
        level_4[i] = pedersen_hash([level[i * 2], level[i * 2 + 1]]);
    }

    // Level 2: 4 nodes -> 2 nodes
    let mut level_2: [Field; 2] = [0; 2];
    for i in 0..2 {
        level_2[i] = pedersen_hash([level_4[i * 2], level_4[i * 2 + 1]]);
    }

    // Level 3: 2 nodes -> 1 root
    let root = pedersen_hash([level_2[0], level_2[1]]);

    root
}

#[test]
fn test_valid_issuance() {
    // Setup: 8 accounts with total balance = 1000
    let balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
    let token_supply: u64 = 1000; // Equal to sum of balances

    // Compute the correct commitment
    let commitment = compute_merkle_root(balances);

    // Should succeed: token_supply = sum(balances)
    main(token_supply, commitment, balances);
}

#[test]
fn test_valid_issuance_with_excess_deposits() {
    // Setup: 8 accounts with total balance = 1000
    let balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
    let token_supply: u64 = 800; // Less than sum of balances

    // Compute the correct commitment
    let commitment = compute_merkle_root(balances);

    // Should succeed: token_supply < sum(balances) (bank has excess reserves)
    main(token_supply, commitment, balances);
}

#[test(should_fail_with = "Solvency constraint violated: total deposits < token supply")]
fn test_over_issuance_fails() {
    // Setup: 8 accounts with total balance = 1000
    let balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
    let token_supply: u64 = 1001; // More than sum of balances

    // Compute the correct commitment
    let commitment = compute_merkle_root(balances);

    // Should fail: token_supply > sum(balances) (over-issuance)
    main(token_supply, commitment, balances);
}

#[test(should_fail_with = "Commitment binding violated: balances do not match deposit_commitment")]
fn test_fake_balances_fails() {
    // Real balances that were committed to
    let real_balances: [u64; NUM_ACCOUNTS] = [100, 150, 200, 50, 250, 100, 75, 75];
    let commitment = compute_merkle_root(real_balances);

    // Fake balances the bank tries to use
    let fake_balances: [u64; NUM_ACCOUNTS] = [200, 200, 200, 200, 200, 200, 200, 200];
    let token_supply: u64 = 1000;

    // Should fail: fake_balances don't match the commitment
    main(token_supply, commitment, fake_balances);
}
