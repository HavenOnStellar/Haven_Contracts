//! # Haven Registry
//!
//! The core Soroban smart contract for the Haven Decentralized Device Registry.
//!
//! Haven makes smartphone theft economically unviable by turning physical hardware
//! into stateful on-chain assets. It provides trustless recovery bounties to outbid
//! the secondary black market and acts as a cryptographic proof-of-loss layer for
//! the insurance industry.
//!
//! ## Architecture
//!
//! The contract is organized into four modules:
//!
//! - **device**: Asset minting — ingests SHA-256 hashed IMEIs and issues Haven Device Assets
//! - **killswitch**: Stolen state management — freezes assets and accepts bounty deposits
//! - **recovery**: Trustless payout — releases escrowed funds to the finder's address
//! - **insurance**: Salvage logic — burns or transfers asset authority upon insurance claims

#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, BytesN, Env, String};

mod device;
mod insurance;
mod killswitch;
mod recovery;

#[cfg(test)]
mod test;

// ---------------------------------------------------------------------------
// Storage Keys
// ---------------------------------------------------------------------------

/// Top-level storage key enum for the Haven Registry contract.
///
/// All persistent state is keyed through this enum to keep the storage
/// namespace clean and collision-free.
#[contracttype]
#[derive(Clone, Debug)]
pub enum DataKey {
    /// The contract administrator address.
    Admin,
    /// Maps a hashed IMEI (SHA-256, 32 bytes) to its `DeviceState`.
    Device(BytesN<32>),
    /// Maps a hashed IMEI to the escrowed bounty amount (in stroops / USDC units).
    Bounty(BytesN<32>),
    /// Tracks the total number of registered devices.
    DeviceCount,
}

// ---------------------------------------------------------------------------
// Data Structures
// ---------------------------------------------------------------------------

/// On-chain representation of a registered device.
///
/// This struct is stored persistently under `DataKey::Device(hashed_imei)`.
/// The raw IMEI **never** touches the chain — only its SHA-256 hash is stored.
#[contracttype]
#[derive(Clone, Debug)]
pub struct DeviceState {
    /// The Stellar address of the device owner.
    pub owner: Address,
    /// SHA-256 hash of the device's IMEI number (32 bytes).
    pub hashed_imei: BytesN<32>,
    /// Human-readable device model (e.g., "iPhone 15 Pro").
    pub device_model: String,
    /// Whether the device has been reported as stolen.
    pub is_stolen: bool,
    /// Ledger sequence number when the device was registered.
    pub registered_at: u32,
    /// Optional email or phone for device recovery contact.
    pub recovery_contact: String,
    /// Optional insurer address (set when an insurance claim is filed).
    pub insurer: Option<Address>,
}

// ---------------------------------------------------------------------------
// Contract Definition
// ---------------------------------------------------------------------------

#[contract]
pub struct HavenRegistry;

#[contractimpl]
impl HavenRegistry {
    /// Initialize the contract with an administrator address.
    ///
    /// This should be called once after deployment. The admin can perform
    /// privileged operations like upgrading the contract.
    ///
    /// # Arguments
    /// * `env` - The contract environment
    /// * `admin` - The Stellar address to set as administrator
    pub fn initialize(env: Env, admin: Address) {
        // Ensure the contract hasn't already been initialized
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }

        admin.require_auth();
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::DeviceCount, &0u32);
    }

    // -----------------------------------------------------------------------
    // Device Registration — see `device.rs`
    // -----------------------------------------------------------------------

    /// Register a new device on-chain by storing its hashed IMEI.
    ///
    /// The IMEI must be hashed (SHA-256) **off-chain** before calling this
    /// function. The raw IMEI should never be sent to the contract.
    ///
    /// # Arguments
    /// * `owner` - The device owner's Stellar address
    /// * `hashed_imei` - SHA-256 hash of the IMEI (32 bytes)
    /// * `device_model` - Human-readable device model string
    pub fn register_device(
        env: Env,
        owner: Address,
        hashed_imei: BytesN<32>,
        device_model: String,
    ) -> DeviceState {
        device::register_device(env, owner, hashed_imei, device_model)
    }

    /// Look up a device by its hashed IMEI.
    ///
    /// Returns the full `DeviceState` if found. This is the function that
    /// vendors and the public portal use to verify a device's status.
    pub fn get_device(env: Env, hashed_imei: BytesN<32>) -> DeviceState {
        device::get_device(env, hashed_imei)
    }

    // -----------------------------------------------------------------------
    // Killswitch — see `killswitch.rs`
    // -----------------------------------------------------------------------

    /// Report a device as stolen, freeze it, and deposit a recovery bounty.
    ///
    /// This function:
    /// 1. Marks the device as `is_stolen: true`
    /// 2. Stores the bounty amount in escrow
    /// 3. Saves the recovery contact information
    ///
    /// Only the device owner can call this function.
    pub fn report_stolen(
        env: Env,
        owner: Address,
        hashed_imei: BytesN<32>,
        bounty_amount: i128,
        recovery_contact: String,
    ) {
        killswitch::report_stolen(env, owner, hashed_imei, bounty_amount, recovery_contact)
    }

    /// Get the current bounty amount escrowed for a stolen device.
    pub fn get_bounty(env: Env, hashed_imei: BytesN<32>) -> i128 {
        killswitch::get_bounty(env, hashed_imei)
    }

    // -----------------------------------------------------------------------
    // Recovery — see `recovery.rs`
    // -----------------------------------------------------------------------

    /// Confirm that a stolen device has been recovered.
    ///
    /// The owner calls this to flip `is_stolen` back to `false` and release
    /// the escrowed bounty to the finder's address.
    ///
    /// Only the device owner can call this function.
    pub fn confirm_recovery(
        env: Env,
        owner: Address,
        hashed_imei: BytesN<32>,
        finder: Address,
    ) {
        recovery::confirm_recovery(env, owner, hashed_imei, finder)
    }

    // -----------------------------------------------------------------------
    // Insurance — see `insurance.rs`
    // -----------------------------------------------------------------------

    /// File an insurance claim, transferring device authority to the insurer.
    ///
    /// This permanently marks the device as claimed and assigns the insurer
    /// as the new authority. The original owner loses control of the asset.
    ///
    /// Only the device owner can initiate a claim.
    pub fn file_insurance_claim(
        env: Env,
        owner: Address,
        hashed_imei: BytesN<32>,
        insurer: Address,
    ) {
        insurance::file_insurance_claim(env, owner, hashed_imei, insurer)
    }
}
