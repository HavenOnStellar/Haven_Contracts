# Haven Registry Events

This document describes the events emitted by the Haven Registry smart contract.

## DeviceRegistered Event

Emitted when a new device is successfully registered on-chain.

### Event Structure

**Topics:**
- `dev_reg` (symbol_short) - Event category identifier
- `register` (symbol_short) - Event action identifier

**Data Payload:**
A tuple containing:
1. `hashed_imei: BytesN<32>` - SHA-256 hash of the device IMEI
2. `owner: Address` - Stellar address of the device owner
3. `device_model: String` - Human-readable device model (e.g., "iPhone 15 Pro")

### Usage

This event is useful for:
- **Indexers**: Building off-chain databases of registered devices
- **Analytics**: Tracking device registration trends and statistics
- **Notifications**: Alerting users when their device registration is confirmed
- **Audit trails**: Maintaining immutable records of device ownership

### Example

When a user registers an iPhone 15 Pro, the event will contain:
```
Topics: ("dev_reg", "register")
Data: (
  0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20,  // hashed IMEI
  GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX,              // owner address
  "iPhone 15 Pro"                                                         // device model
)
```

### Integration Notes

- The raw IMEI is **never** included in the event - only its SHA-256 hash
- Events are emitted after the device state is persisted to storage
- The device count is incremented before the event is emitted
- Event emission occurs in the same transaction as device registration

## Future Events

The following events are planned for future implementation:

- **DeviceStolen**: Emitted when a device is reported as stolen
- **DeviceRecovered**: Emitted when a stolen device is recovered
- **InsuranceClaimed**: Emitted when an insurance claim is filed

See the TODO comments in the respective module files for more details.
