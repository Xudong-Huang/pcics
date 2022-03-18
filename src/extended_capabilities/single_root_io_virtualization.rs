//! Single Root I/O Virtualization (SR-IOV)
//!
//! Single Root I/O Virtualization (SR-IOV) SR-IOV consists of two basic units: PF (Physical Function),
//! which supports SR-IOV PCIe extended capability and manages entire physical devices;
//! and VF (Virtual Function), a “lightweight” PCIe function which is a passthrough device for VMs.

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
    pub sriov_vf_migration_state_array_offset: u32,
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
            sriov_vf_migration_state_array_offset: bytes.read_with::<u32>(offset, endian)?,
        };
        Ok((ptm, *offset))
    }
}

#[bitfield(bits = 32)]
#[repr(u32)]
pub struct SrIovCapabilityProto {
    pub ptm_requester_capable: bool,
    pub ptm_responder_capable: bool,
    pub ptm_root_capable: bool,
    pub rsvdp: B5,
    pub local_clock_granularity: u8,
    pub rsvdp_2: B16,
}
impl From<SrIovCapability> for SrIovCapabilityProto {
    fn from(data: SrIovCapability) -> Self {
        Self::new()
            .with_ptm_requester_capable(data.ptm_requester_capable)
            .with_ptm_responder_capable(data.ptm_responder_capable)
            .with_ptm_root_capable(data.ptm_root_capable)
            .with_rsvdp(0)
            .with_local_clock_granularity(data.local_clock_granularity)
            .with_rsvdp_2(0)
    }
}

/// Describes a Function’s support for Precision Time Measurement
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrIovCapability {
    /// PTM Requester Capable
    pub ptm_requester_capable: bool,
    /// PTM Responder Capable
    pub ptm_responder_capable: bool,
    /// PTM Root Capable
    pub ptm_root_capable: bool,
    /// Local Clock Granularity
    pub local_clock_granularity: u8,
}
impl From<SrIovCapabilityProto> for SrIovCapability {
    fn from(proto: SrIovCapabilityProto) -> Self {
        let _ = proto.rsvdp();
        let _ = proto.rsvdp_2();
        Self {
            ptm_requester_capable: proto.ptm_requester_capable(),
            ptm_responder_capable: proto.ptm_responder_capable(),
            ptm_root_capable: proto.ptm_root_capable(),
            local_clock_granularity: proto.local_clock_granularity(),
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
    pub ptm_enable: bool,
    pub root_select: bool,
    pub rsvdp: B6,
    pub effective_granularity: u8,
}
impl From<SrIovControl> for SrIovControlProto {
    fn from(data: SrIovControl) -> Self {
        Self::new()
            .with_ptm_enable(data.ptm_enable)
            .with_root_select(data.root_select)
            .with_rsvdp(0)
            .with_effective_granularity(data.effective_granularity)
    }
}

/// SR-IOV control
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrIovControl {
    /// PTM Enable
    pub ptm_enable: bool,
    /// Root Select
    pub root_select: bool,
    /// Effective Granularity
    pub effective_granularity: u8,
}
impl From<SrIovControlProto> for SrIovControl {
    fn from(proto: SrIovControlProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            ptm_enable: proto.ptm_enable(),
            root_select: proto.root_select(),
            effective_granularity: proto.effective_granularity(),
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
    pub ptm_enable: bool,
    pub root_select: bool,
    pub rsvdp: B6,
    pub effective_granularity: u8,
}
impl From<SrIovStatus> for SrIovStatusProto {
    fn from(data: SrIovStatus) -> Self {
        Self::new()
            .with_ptm_enable(data.ptm_enable)
            .with_root_select(data.root_select)
            .with_rsvdp(0)
            .with_effective_granularity(data.effective_granularity)
    }
}

/// SR-IOV status
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SrIovStatus {
    /// PTM Enable
    pub ptm_enable: bool,
    /// Root Select
    pub root_select: bool,
    /// Effective Granularity
    pub effective_granularity: u8,
}
impl From<SrIovStatusProto> for SrIovStatus {
    fn from(proto: SrIovStatusProto) -> Self {
        let _ = proto.rsvdp();
        Self {
            ptm_enable: proto.ptm_enable(),
            root_select: proto.root_select(),
            effective_granularity: proto.effective_granularity(),
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
