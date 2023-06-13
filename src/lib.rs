#![no_std]
#![feature(iter_next_chunk)]

use core::ops::{Deref, DerefMut, Index, IndexMut};

use bin_utils::*;

#[derive(Clone, Copy, PartialEq, Eq)]
/// A MAC address.
pub struct MACAddress {
    mac_address: [u8; 6],
}
impl MACAddress {
    /// Checks the unicast/multicast bit of the first octet.
    pub fn is_unicast(&self) -> bool {
        self.mac_address[0] & 0x1 != 0x1
    }
    /// Checks the global/local bit of the first octet.
    pub fn is_unique(&self) -> bool {
        self.mac_address[0] & 0x2 != 0x2
    }
}
impl From<[u8; 6]> for MACAddress {
    fn from(value: [u8; 6]) -> Self {
        Self { mac_address: value }
    }
}
impl From<MACAddress> for [u8; 6] {
    fn from(value: MACAddress) -> Self {
        value.mac_address
    }
}
impl ReadFixed<6> for MACAddress {
    fn from_bytes(data: &[u8; 6]) -> Result<Self, ParserError> {
        Ok(Self { mac_address: *data })
    }
}
impl WriteFixed<6> for MACAddress {
    fn to_bytes(&self) -> [u8; 6] {
        self.mac_address
    }
}
impl Deref for MACAddress {
    type Target = [u8; 6];
    fn deref(&self) -> &Self::Target {
        &self.mac_address
    }
}
impl DerefMut for MACAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mac_address
    }
}
impl Index<usize> for MACAddress {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.mac_address[index]
    }
}
impl IndexMut<usize> for MACAddress {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.mac_address[index]
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for MACAddress {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!(
            "{:2x}:{:2x}:{:2x}:{:2x}:{:2x}:{:2x}",
            self[0], self[1], self[2], self[3], self[4], self[5]
        ))
    }
}
