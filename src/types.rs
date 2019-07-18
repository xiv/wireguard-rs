use std::fmt;
use std::error::Error;

use x25519_dalek::PublicKey;
use x25519_dalek::SharedSecret;

use crate::timestamp;

// config error

#[derive(Debug)]
pub struct ConfigError(String);

impl ConfigError {
    pub fn new(s : &str) -> Self {
        ConfigError(s.to_string())
    }
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ConfigError({})", self.0)
    }
}

impl Error for ConfigError {
    fn description(&self) -> &str {
        &self.0
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// handshake error

#[derive(Debug)]
pub enum HandshakeError {
    DecryptionFailure,
    UnknownPublicKey,
    InvalidMessageFormat,
    OldTimestamp
}

impl fmt::Display for HandshakeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HandshakeError")
    }
}

impl Error for HandshakeError {
    fn description(&self) -> &str {
        "Generic Handshake Error"
    }

    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

// types for resulting key-material

struct Key {
    key : [u8; 32],
    id  : u32
}

pub struct KeyPair {
    confimed : bool, // has the key-pair been confirmed?
    send     : Key,  // key for outbound messages
    recv     : Key   // key for inbound messages
}

pub struct Output (
    pub Option<KeyPair>, // resulting key-pair of successful handshake
    pub Option<Vec<u8>>  // message to send
);

// per-peer state machine

pub type Psk = [u8; 32];

