//! # Insurance Module
//!
//! Provides cryptographic proof-of-loss for the insurance industry.
//!
//! When a device owner files an insurance claim:
//! 1. The device's authority is permanently transferred to the insurer
//! 2. The owner can no longer control or recover the device
//! 3. The on-chain record serves as immutable proof of loss
//!
//! This eliminates "double-dipping" fraud where owners claim insurance
//! while still possessing or selling their device on the black market.
//!
//! ## B2B Value Proposition
//!
//! Insurance companies can integrate Haven as their proof-of-loss layer:
//! - Verify claims against on-chain state before paying out
//! - Reduce fraud by requiring on-chain device freeze before claim approval
//! - Access immutable audit trails for regulatory compliance

use soroban_sdk::{Address, BytesN, Env};

use crate::{DataKey, DeviceState};

/// File an insurance claim, transferring device authority to an insurer.
///
/// # Flow
/// 1. Verify the owner has signed the transaction
/// 2. Load the device state and verify ownership
/// 3. Assign the insurer as the new authority
/// 4. Permanently mark the device as claimed
///
/// # Arguments
/// * `owner` - Must match the device's registered owner
/// * `hashed_imei` - The SHA-256 hash identifying the device
/// * `insurer` - The Stellar address of the insurance provider
///
/// # Panics
/// - If the device doesn't exist
/// - If `owner` doesn't match the registered owner
/// - If an insurance claim has already been filed (insurer is already set)
///
/// # TODO
/// - [ ] Add a multi-sig requirement: both owner AND insurer must sign
/// - [ ] Implement a "salvage" function for insurers to release/transfer the asset
/// - [ ] Add claim metadata (claim ID, timestamp, payout amount)
/// - [ ] Consider burning the device NFT entirely vs. transferring to insurer
/// - [ ] Emit an `InsuranceClaimed` event for indexers
/// - [ ] Handle the bounty: should it be refunded to owner or forfeited?
/// - [ ] Add a cooldown period between reporting stolen and filing insurance
///   to prevent instant insurance fraud
pub fn file_insurance_claim(env: Env, owner: Address, hashed_imei: BytesN<32>, insurer: Address) {
    owner.require_auth();

    let device_key = DataKey::Device(hashed_imei.clone());
    let mut device: DeviceState = env
        .storage()
        .persistent()
        .get(&device_key)
        .expect("device not found");

    // Verify ownership
    if device.owner != owner {
        panic!("not the device owner");
    }

    // Ensure no existing insurance claim
    if device.insurer.is_some() {
        panic!("insurance claim already filed");
    }

    // Transfer authority to the insurer
    device.insurer = Some(insurer);

    // The device remains in stolen state — the insurer now owns the salvage rights
    env.storage().persistent().set(&device_key, &device);

    // TODO: Handle the bounty escrow
    // If a bounty was deposited, it could be:
    // 1. Refunded to the owner (since insurance is paying out)
    // 2. Forfeited to the contract (penalty for insurance claim)
    // 3. Transferred to the insurer (salvage value)

    // TODO: Emit InsuranceClaimed event
    // env.events().publish((symbol_short!("insured"),), (&hashed_imei, &insurer));
}
