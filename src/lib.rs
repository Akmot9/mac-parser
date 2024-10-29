use core::fmt::{Debug, Display, Formatter};
use core::convert::TryFrom;
use thiserror::Error;

/// Erreurs de parsing pour les adresses MAC
#[derive(Error, Debug, PartialEq, Eq, Clone, Copy)]
pub enum MacParseError {
    #[error("Invalid MAC address length: expected {expected} bytes, but got {actual} bytes")]
    InvalidLength { expected: usize, actual: usize },
}

/// Adresse MAC EUI-48 (6 octets)
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct MacAddress([u8; 6]);

impl MacAddress {
    /// Constante d'adresse broadcast
    pub const fn broadcast() -> Self {
        Self([0xff; 6])
    }

    /// Constante d'adresse zéro
    pub const fn zero() -> Self {
        Self([0x00; 6])
    }

    /// Vérifie si c'est une adresse multicast
    pub const fn is_multicast(&self) -> bool {
        self.0[0] & 0x1 != 0
    }

    /// Vérifie si c'est une adresse locale
    pub const fn is_local(&self) -> bool {
        self.0[0] & 0x2 != 0
    }

    /// Retourne un slice immuable
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl TryFrom<&[u8]> for MacAddress {
    type Error = MacParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() != 6 {
            return Err(MacParseError::InvalidLength { expected: 6, actual: bytes.len() });
        }
        let mut addr = [0u8; 6];
        addr.copy_from_slice(bytes);
        Ok(Self(addr))
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}", 
               self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}

impl Debug for MacAddress {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        write!(f, "MacAddress({:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x})",
               self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5])
    }
}
