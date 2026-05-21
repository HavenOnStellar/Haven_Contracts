//! # Killswitch Module
//!
//! The economic denial engine of Haven.
//!
//! When a device is reported stolen:
//! 1. The device state flips to `is_stolen: true`
//! 2. The asset is effectively "frozen" — it cannot be transferred
//! 3. The owner deposits a bounty (XLM or USDC) into the contract's escrow
//! 4. A recovery contact (email/phone) is stored for the finder to reach out
//!
//! The bounty sits in the contract's balance until either:
//! - The owner confirms recovery → bounty goes to finder (see `recovery.rs`)
//! - An insurance claim is filed → bounty may be returned or forfeited (see `insurance.rs`)

use soroban_sdk::{Address, BytesN, Env, String};

use crate::{DataKey, DeviceState};

/// Minimum recovery bounty in the asset's smallest unit.
///
/// For XLM this is stroops, so this value is 0.1 XLM. For USDC or any other
/// 7-decimal Stellar asset, it is 0.1 token units.
pub const MIN_BOUNTY_AMOUNT: i128 = 1_000_000;

/// Report a device as stolen and deposit a recovery bounty.
///
/// # Flow
/// 1. Verify the owner has signed the transaction
/// 2. Load the device state and verify ownership
/// 3. Flip `is_stolen` to `true`
/// 4. Store the bounty amount in escrow
/// 5. Save the recovery contact for the finder
///
/// # Arguments
/// * `owner` - Must match the device's registered owner
/// * `hashed_imei` - The SHA-256 hash identifying the device
/// * `bounty_amount` - Amount to escrow (in stroops for XLM, or smallest unit for USDC)
/// * `recovery_contact` - Email or phone number for the finder to contact
///
/// # Panics
/// - If the device doesn't exist
/// - If `owner` doesn't match the registered owner
/// - If the device is already reported as stolen
/// - If `bounty_amount` is zero, negative, or below `MIN_BOUNTY_AMOUNT`
///
/// # TODO
/// - [ ] Actually transfer XLM/USDC from the owner to the contract's balance
///       This requires a SAC (Stellar Asset Contract) token transfer call:
///       `token::Client::new(&env, &token_address).transfer(&owner, &contract_address, &bounty_amount)`
/// - [ ] Emit a `DeviceStolen` event for indexers and notification services
/// - [ ] Allow the owner to increase the bounty after initial report
/// - [ ] Add a minimum bounty threshold to ensure economic viability
pub fn report_stolen(
    env: Env,
    owner: Address,
    hashed_imei: BytesN<32>,
    bounty_amount: i128,
    recovery_contact: String,
) {
    owner.require_auth();

    if bounty_amount <= 0 {
        panic!("bounty amount must be positive");
    }
    if bounty_amount < MIN_BOUNTY_AMOUNT {
        panic!("bounty amount below minimum");
    }

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

    // Ensure not already stolen
    if device.is_stolen {
        panic!("device already reported as stolen");
    }

    // Update device state
    device.is_stolen = true;
    device.recovery_contact = recovery_contact;
    env.storage().persistent().set(&device_key, &device);

    // Store the bounty amount
    // TODO: Actually transfer tokens from owner to contract
    // For now, we just record the promised bounty amount
    let bounty_key = DataKey::Bounty(hashed_imei);
    env.storage().persistent().set(&bounty_key, &bounty_amount);

    // TODO: Emit DeviceStolen event
    // env.events().publish((symbol_short!("stolen"),), (&hashed_imei, &bounty_amount));
}

/// Get the current bounty amount for a device.
///
/// Returns 0 if no bounty has been set.
///
/// # TODO
/// - [ ] Return the token type (XLM vs USDC) alongside the amount
pub fn get_bounty(env: Env, hashed_imei: BytesN<32>) -> i128 {
    let key = DataKey::Bounty(hashed_imei);
    env.storage().persistent().get(&key).unwrap_or(0)
}
