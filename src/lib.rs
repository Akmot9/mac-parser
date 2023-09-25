#![no_std]
#![feature(iter_next_chunk)]

use core::ops::{Deref, DerefMut, Index, IndexMut};

use bin_utils::*;

/// The broadcast address.
pub const BROADCAST: MACAddress = MACAddress::new([0xff; 6]);
/// An empty address.
pub const ZERO: MACAddress = MACAddress::new([0x00; 6]);

#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
/// A MAC address.
pub struct MACAddress(pub [u8; 6]);
impl MACAddress {
    /// Checks the unicast/multicast bit of the first octet.
    pub const fn is_multicast(&self) -> bool {
        self.0[0] & 0x1 != 0x0
    }
    /// Checks the global/local bit of the first octet.
    pub const fn is_local(&self) -> bool {
        self.0[0] & 0x2 != 0x0
    }
    pub const fn new(address: [u8; 6]) -> Self {
        Self(address)
    }
}
impl From<[u8; 6]> for MACAddress {
    fn from(value: [u8; 6]) -> Self {
        Self(value)
    }
}
impl From<MACAddress> for [u8; 6] {
    fn from(value: MACAddress) -> Self {
        *value.deref()
    }
}
impl ReadFixed<6> for MACAddress {
    fn from_bytes(data: &[u8; 6]) -> Result<Self, ParserError> {
        Ok(Self(*data))
    }
}
impl WriteFixed<6> for MACAddress {
    fn to_bytes(&self) -> [u8; 6] {
        *self.deref()
    }
}
impl Deref for MACAddress {
    type Target = [u8; 6];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for MACAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Index<usize> for MACAddress {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for MACAddress {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for MACAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Display>::fmt(&self, f)
    }
}
impl core::fmt::Display for MACAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{:2x}:{:2x}:{:2x}:{:2x}:{:2x}:{:2x}",
            self[0], self[1], self[2], self[3], self[4], self[5]
        ))
    }
}
