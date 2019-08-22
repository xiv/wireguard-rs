use std::time::Instant;

/* This file holds types passed between components.
 * Whenever a type cannot be held local to a single module.
 */

#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub key: [u8; 32],
    pub id: u32,
}

#[cfg(test)]
impl PartialEq for Key {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.key[..] == other.key[..]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KeyPair {
    pub birth: Instant,  // when was the key-pair created
    pub confirmed: bool, // has the key-pair been confirmed?
    pub send: Key,       // key for outbound messages
    pub recv: Key,       // key for inbound messages
}