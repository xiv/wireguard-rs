mod constants;
mod timers;
mod wireguard;

mod endpoint;
mod handshake;
mod peer;
mod router;
mod types;

#[cfg(test)]
mod tests;

pub use peer::Peer;
pub use wireguard::Wireguard;

#[cfg(test)]
pub use types::dummy_keypair;

#[cfg(test)]
use super::platform::dummy;

use super::platform::{bind, tun, Endpoint};
use peer::PeerInner;
use types::KeyPair;
use wireguard::HandshakeJob;