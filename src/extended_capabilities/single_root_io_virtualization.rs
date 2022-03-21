//! Single Root I/O Virtualization (SR-IOV)
//!
//! Single Root I/O Virtualization (SR-IOV) SR-IOV consists of two basic units: PF (Physical Function),
//! which supports SR-IOV PCIe extended capability and manages entire physical devices;
//! and VF (Virtual Function), a “lightweight” PCIe function which is a passthrough device for VMs.

use crate::capabilities::msi_x::Table;
use crate::header::BaseAddressesNormal;
use byte::{
    self,
    ctx::*,
    // TryWrite,
    BytesExt,
    TryRead,
};
use modular_bitfield::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleRootIoVirtualization {
    /// SR-IOV Capabilities
    pub sriov_capability: SrIovCapability,
    /// SR-IOV Control
    pub sriov_control: SrIovControl,
    /// SR-IOV Status
    pub sriov_status: SrIovStatus,
    /// InitialVFs (RO)
    pub sriov_initial_vfs: u16,
    /// TotalVFs (RO)
    pub sriov_total_vfs: u16,
    /// NumVFs (RW)
    pub sriov_num_vfs: u16,
    /// Function Dependency Link (RO)
    pub sriov_function_denpendency_link: SrIovFunctionDepLink,
    /// First VF Offset (RO)
    pub sriov_first_vf_offset: u16,
    /// VF Stride (RO)
    pub sriov_vf_stride: u16,
    /// VF Device ID (RO)
    pub sriov_vf_device_id: SrIovVfDeviceId,
    /// Supported Pages Sizes (RO)
    pub sriov_supported_page_sizes: u32,
    /// System Page Size (RW)
    pub sriov_system_page_size: u32,
    /// VF BAR0 ~ BAR 5
    pub sriov_vf_bar: BaseAddressesNormal,
    /// VF Migration State Array Offset (RO)
    pub sriov_vf_migration_state_array_offset: Table,
}
impl<'a> TryRead<'a, Endian> for SingleRootIoVirtualization {
    fn try_read(bytes: &'a [u8], endian: Endian) -> byte::Result<(Self, usize)> {
        let offset = &mut 0;
        let ptm = SingleRootIoVirtualization {
            sriov_capability: bytes.read_with::<u32>(offset, endian)?.into(),
            sriov_control: bytes.read_with::<u16>(offset, endian)?.into(),
            sriov_status: bytes.read_with::<u16>(offset, endian)?.into(),
            sriov_initial_vfs: bytes.read_with::<u16>(offset, endian)?,
            sriov_total_vfs: bytes.read_with::<u16>(offset, endian)?,
            sriov_num_vfs: bytes.read_with::<u16>(offset, endian)?,
            sriov_function_denpendency_link: bytes.read_with::<u16>(offset, endian)?.into(),
            sriov_first_vf_offset: bytes.read_with::<u16>(offset, endian)?,
            sriov_vf_stride: bytes.read_with::<u16>(offset, endian)?,
            sriov_vf_device_id: bytes.read_with::<u32>(offset, endian)?.into(),
            sriov_supported_page_sizes: bytes.read_with::<u32>(offset, endian)?,
            sriov_system_page_size: bytes.read_with::<u32>(offset, endian)?,
            sriov_vf_bar: bytes.read_with::<BaseAddressesNormal>(offset, endian)?,
            sriov_vf_migration_state_array_offset: bytes.read_with::<u32>(offset, endian)?.into(),
        };
        Ok((ptm, *offset))
    }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct SrIovCapabilityProto {
    /// VF Migration Capable
    pub vf_migration: bool,
    /// ARI Capable Hierarchy Preserved
    pub ari_preserved: bool,
    #[skip]
    __: B19,
    /// VF Migration Interrupt Message Number
    pub vf_mig_int: B11,
}
impl From<SrIovCapability> for SrIovCapabilityProto {
    fn from(data: SrIovCapability) -> Self {
        Self::new()
            .with_vf_migration(data.vf_migration)
            .with_ari_preserved(data.ari_preserved)
            .with_vf_mig_int(data.vf_mig_int)
    }
}

/// Describes a Function’s support for Precision Time Measurement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrIovCapability {
    /// VF Migration Capable
    pub vf_migration: bool,
    /// ARI Capable Hierarchy Preserved
    pub ari_preserved: bool,
    /// VF Migration Interrupt Message Number
    pub vf_mig_int: u16,
}
impl From<SrIovCapabilityProto> for SrIovCapability {
    fn from(proto: SrIovCapabilityProto) -> Self {
        Self {
            vf_migration: proto.vf_migration(),
            ari_preserved: proto.ari_preserved(),
            vf_mig_int: proto.vf_mig_int(),
        }
    }
}
impl From<u32> for SrIovCapability {
    fn from(dword: u32) -> Self {
        SrIovCapabilityProto::from(dword).into()
    }
}
impl From<SrIovCapability> for u32 {
    fn from(data: SrIovCapability) -> Self {
        SrIovCapabilityProto::from(data).into()
    }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct SrIovControlProto {
    pub vf_enable: bool,
    pub vf_mig_enable: bool,
    pub vf_mig_int_enable: bool,
    pub vf_mse: bool,
    pub ari_capable: bool,
    #[skip]
    __: B11,
}
impl From<SrIovControl> for SrIovControlProto {
    fn from(data: SrIovControl) -> Self {
        Self::new()
            .with_vf_enable(data.vf_enable)
            .with_vf_mig_enable(data.vf_mig_enable)
            .with_vf_mig_int_enable(data.vf_mig_int_enable)
            .with_vf_mse(data.vf_mse)
            .with_ari_capable(data.ari_capable)
    }
}

/// SR-IOV control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrIovControl {
    /// VF Enable
    pub vf_enable: bool,
    /// VF Migration Enable
    pub vf_mig_enable: bool,
    /// VF Migration Interrupt Enable
    pub vf_mig_int_enable: bool,
    /// VF MSE
    pub vf_mse: bool,
    /// ARI Capable Hierarchy
    pub ari_capable: bool,
}
impl From<SrIovControlProto> for SrIovControl {
    fn from(proto: SrIovControlProto) -> Self {
        Self {
            vf_enable: proto.vf_enable(),
            vf_mig_enable: proto.vf_mig_enable(),
            vf_mig_int_enable: proto.vf_mig_int_enable(),
            vf_mse: proto.vf_mse(),
            ari_capable: proto.ari_capable(),
        }
    }
}
impl From<u16> for SrIovControl {
    fn from(word: u16) -> Self {
        SrIovControlProto::from(word).into()
    }
}
impl From<SrIovControl> for u16 {
    fn from(data: SrIovControl) -> Self {
        SrIovControlProto::from(data).into()
    }
}

#[bitfield(bits = 16)]
#[repr(u16)]
pub struct SrIovStatusProto {
    pub vf_migration: bool,
    #[skip]
    __: B15,
}
impl From<SrIovStatus> for SrIovStatusProto {
    fn from(data: SrIovStatus) -> Self {
        Self::new().with_vf_migration(data.vf_migration)
    }
}

/// SR-IOV status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrIovStatus {
    /// VF Migration Status
    pub vf_migration: bool,
}
impl From<SrIovStatusProto> for SrIovStatus {
    fn from(proto: SrIovStatusProto) -> Self {
        Self {
            vf_migration: proto.vf_migration(),
        }
    }
}
impl From<u16> for SrIovStatus {
    fn from(word: u16) -> Self {
        SrIovStatusProto::from(word).into()
    }
}
impl From<SrIovStatus> for u16 {
    fn from(data: SrIovStatus) -> Self {
        SrIovStatusProto::from(data).into()
    }
}

/// Function Dependency Link prototype
#[bitfield(bits = 16)]
#[repr(u16)]
pub struct SrIovFunctionDepLinkProto {
    function_dependency_link: u8,
    rsvdp: B8,
}
impl From<SrIovFunctionDepLink> for SrIovFunctionDepLinkProto {
    fn from(data: SrIovFunctionDepLink) -> Self {
        Self::new()
            .with_function_dependency_link(data.function_dependency_link)
            .with_rsvdp(0)
    }
}

/// Function Dependency Link
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrIovFunctionDepLink {
    /// Function Dependency Link
    pub function_dependency_link: u8,
}
impl From<SrIovFunctionDepLinkProto> for SrIovFunctionDepLink {
    fn from(proto: SrIovFunctionDepLinkProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            function_dependency_link: proto.function_dependency_link(),
        }
    }
}
impl From<u16> for SrIovFunctionDepLink {
    fn from(word: u16) -> Self {
        SrIovFunctionDepLinkProto::from(word).into()
    }
}
impl From<SrIovFunctionDepLink> for u16 {
    fn from(data: SrIovFunctionDepLink) -> Self {
        SrIovFunctionDepLinkProto::from(data).into()
    }
}

/// vf device id proto
#[bitfield(bits = 32)]
#[repr(u32)]
pub struct SrIovVfDeviceIdProto {
    pub rsvdp: u16,
    pub vf_device_id: u16,
}
impl From<SrIovVfDeviceId> for SrIovVfDeviceIdProto {
    fn from(data: SrIovVfDeviceId) -> Self {
        Self::new()
            .with_rsvdp(0)
            .with_vf_device_id(data.vf_device_id)
    }
}

/// vf device id
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrIovVfDeviceId {
    /// vf device id
    pub vf_device_id: u16,
}
impl From<SrIovVfDeviceIdProto> for SrIovVfDeviceId {
    fn from(proto: SrIovVfDeviceIdProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            vf_device_id: proto.vf_device_id(),
        }
    }
}
impl From<u32> for SrIovVfDeviceId {
    fn from(dword: u32) -> Self {
        SrIovVfDeviceIdProto::from(dword).into()
    }
}
impl From<SrIovVfDeviceId> for u32 {
    fn from(data: SrIovVfDeviceId) -> Self {
        SrIovVfDeviceIdProto::from(data).into()
    }
}
