//! # Recovery Module
//!
//! Handles the trustless payout when a stolen device is recovered.
//!
//! The flow:
//! 1. A finder locates the stolen device and contacts the owner via the recovery contact
//! 2. The owner verifies recovery and calls `confirm_recovery` with the finder's address
//! 3. The contract flips `is_stolen` back to `false`
//! 4. The escrowed bounty is released to the finder's address
//!
//! This creates a trustless incentive loop:
//! - Vendors/finders are economically motivated to return devices (bounty > black market price)
//! - Owners get their devices back without relying on law enforcement
//! - The entire flow is on-chain and verifiable

use soroban_sdk::{Address, BytesN, Env, String};

use crate::{DataKey, DeviceState};

/// Confirm device recovery and release the bounty to the finder.
///
/// # Flow
/// 1. Verify the owner has signed the transaction
/// 2. Load the device state and confirm it's currently stolen
/// 3. Flip `is_stolen` back to `false`
/// 4. Transfer the escrowed bounty to the finder's address
/// 5. Clear the bounty record
///
/// # Arguments
/// * `owner` - Must match the device's registered owner
/// * `hashed_imei` - The SHA-256 hash identifying the device
/// * `finder` - The Stellar address of the person who found/returned the device
///
/// # Panics
/// - If the device doesn't exist
/// - If `owner` doesn't match the registered owner
/// - If the device is not currently marked as stolen
///
/// # TODO
/// - [ ] Actually transfer the escrowed tokens to the finder:
///   `token::Client::new(&env, &token_address).transfer(&contract_address, &finder, &bounty)`
/// - [ ] Emit a `DeviceRecovered` event for indexers
/// - [ ] Consider a time-lock: prevent instant recovery to avoid bounty gaming
/// - [ ] Add a "recovery proof" mechanism (e.g., both parties sign off)
/// - [ ] Record recovery history for insurance audit trails
pub fn confirm_recovery(env: Env, owner: Address, hashed_imei: BytesN<32>, finder: Address) {
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

    // Ensure the device is actually stolen
    if !device.is_stolen {
        panic!("device is not reported as stolen");
    }

    // Flip the stolen state back
    device.is_stolen = false;
    device.recovery_contact = String::from_str(&env, "");
    env.storage().persistent().set(&device_key, &device);

    // Release the bounty to the finder
    let bounty_key = DataKey::Bounty(hashed_imei);
    let _bounty: i128 = env.storage().persistent().get(&bounty_key).unwrap_or(0);

    // TODO: Actually transfer the bounty tokens to the finder
    // token::Client::new(&env, &usdc_token_address).transfer(
    //     &env.current_contract_address(),
    //     &finder,
    //     &bounty,
    // );

    // Clear the bounty record
    env.storage().persistent().remove(&bounty_key);

    // Suppress unused variable warning in skeleton
    let _ = finder;

    // TODO: Emit DeviceRecovered event
    // env.events().publish((symbol_short!("recovered"),), (&hashed_imei, &finder));
}
