//! # Device Module
//!
//! Handles device registration and state queries.
//!
//! The core flow:
//! 1. A user hashes their device IMEI off-chain (SHA-256)
//! 2. The hashed IMEI is sent to `register_device`
//! 3. A `DeviceState` is created and stored on-chain
//! 4. Anyone can query `get_device` to check a device's status

use soroban_sdk::{Address, BytesN, Env, String};

use crate::{DataKey, DeviceState};

/// Register a new device on-chain.
///
/// # Flow
/// 1. Verify the owner has signed the transaction
/// 2. Ensure the device hasn't already been registered
/// 3. Create and persist the `DeviceState`
/// 4. Increment the global device counter
///
/// # Panics
/// - If `owner` has not authorized the transaction
/// - If a device with the same `hashed_imei` already exists
///
/// # TODO
/// - [ ] Emit a `DeviceRegistered` event for indexers
/// - [ ] Add optional metadata fields (color, storage capacity, etc.)
/// - [ ] Consider adding a small registration fee to prevent spam
pub fn register_device(
    env: Env,
    owner: Address,
    hashed_imei: BytesN<32>,
    device_model: String,
) -> DeviceState {
    // Require authorization from the device owner
    owner.require_auth();

    // Ensure this device hasn't already been registered
    let key = DataKey::Device(hashed_imei.clone());
    if env.storage().persistent().has(&key) {
        panic!("device already registered");
    }

    // Create the device state
    let device = DeviceState {
        owner: owner.clone(),
        hashed_imei: hashed_imei.clone(),
        device_model,
        is_stolen: false,
        registered_at: env.ledger().sequence(),
        recovery_contact: String::from_str(&env, ""),
        insurer: None,
    };

    // Persist to storage
    env.storage().persistent().set(&key, &device);

    // Increment device count
    let count: u32 = env
        .storage()
        .instance()
        .get(&DataKey::DeviceCount)
        .unwrap_or(0);
    env.storage()
        .instance()
        .set(&DataKey::DeviceCount, &(count + 1));

    // TODO: Emit a DeviceRegistered event
    // env.events().publish((symbol_short!("register"),), &device);

    device
}

/// Retrieve a device's state by its hashed IMEI.
///
/// This is the public lookup function used by:
/// - The vendor verification portal
/// - The owner's dashboard
/// - Insurance providers checking device status
///
/// # Panics
/// - If no device with the given `hashed_imei` exists
///
/// # TODO
/// - [ ] Add a rate-limiting mechanism to prevent IMEI enumeration attacks
/// - [ ] Return an Option<DeviceState> instead of panicking
pub fn get_device(env: Env, hashed_imei: BytesN<32>) -> DeviceState {
    let key = DataKey::Device(hashed_imei);
    env.storage()
        .persistent()
        .get(&key)
        .expect("device not found")
}
