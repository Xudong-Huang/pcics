## PCI configuration space

PCI configuration space is the underlying way that the Conventional PCI, PCI-X and PCI Express perform auto configuration of the cards inserted into their bus.

This library implements access to PCI configuration space and PCI Express extended configuration space.

## Design

The library is divided into three parts:
- PCI 3.0 Compatible Configuration Space Header
- PCI Configuration Space Capabilities
- Extended Configuration Space Capabilities

## Usage

```rust
use pcics::{
    DDR_OFFSET, ECS_OFFSET, Header, Capabilities, ExtendedCapabilities,
    capabilities::{
        CapabilityKind,
        bridge_subsystem_vendor_id::BridgeSubsystemVendorId
    },
    extended_capabilities::{
        ExtendedCapabilityKind,
        vendor_specific_extended_capability::VendorSpecificExtendedCapability
    },
};

let conf_space =
    include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/device/8086_2030/config"
    ));

let header_data = &conf_space[..DDR_OFFSET];
let header = Header::try_from(header_data).unwrap();
assert_eq!((0x8086, 0x2030), (header.vendor_id, header.device_id));

let device_depended_region_data = &conf_space[DDR_OFFSET..ECS_OFFSET];
let mut caps = Capabilities::new(device_depended_region_data, header.capabilities_pointer);
let BridgeSubsystemVendorId { subsystem_vendor_id, .. } =
    caps.find_map(|c| {
        if let CapabilityKind::BridgeSubsystemVendorId(ssvid) = c.kind {
            Some(ssvid)
        } else {
            None
        }
    })
    .unwrap();
assert_eq!(0x8086, subsystem_vendor_id);

let ecs_data = &conf_space[ECS_OFFSET..];
let mut ecaps = ExtendedCapabilities::new(ecs_data);
let VendorSpecificExtendedCapability { header, .. } = ecaps
    .find_map(|c| {
        if let ExtendedCapabilityKind::VendorSpecificExtendedCapability(vsec) = c.kind {
            Some(vsec)
        } else {
            None
        }
    })
    .unwrap();
assert_eq!(0x0c, header.vsec_length);
```
More detailed usage in modules descriptions
