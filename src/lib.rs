// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-dhcp-relay: dhcp relay component model.
//!
//! Real implementation: relays, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Relay {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DhcpRelayError {
    EmptyRelayName,
    DuplicateRelay(String),
    EmptyEventSubject,
}

impl fmt::Display for DhcpRelayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyRelayName => write!(f, "relay name must not be empty"),
            Self::DuplicateRelay(id) => write!(f, "duplicate relay {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for DhcpRelayError {}

pub struct DhcpRelay {
    pub name: String,
    relays: BTreeMap<String, Relay>,
    events: BTreeMap<String, Event>,
}

impl DhcpRelay {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            relays: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_relay(&mut self, item: Relay) -> Result<(), DhcpRelayError> {
        if item.name.trim().is_empty() {
            return Err(DhcpRelayError::EmptyRelayName);
        }
        let id = item.id.clone();
        if self.relays.contains_key(&id) {
            return Err(DhcpRelayError::DuplicateRelay(id.clone()));
        }
        self.relays.insert(id, item);
        Ok(())
    }

    pub fn relay_count(&self) -> usize {
        self.relays.len()
    }

    pub fn get_relay(&self, id: &str) -> Option<&Relay> {
        self.relays.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), DhcpRelayError> {
        if event.subject.trim().is_empty() {
            return Err(DhcpRelayError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_relay(id: &str, name: &str) -> Relay {
        Relay {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = DhcpRelay::new("DhcpRelay");
        assert_eq!(m.name, "DhcpRelay");
        assert_eq!(m.relay_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_relay() {
        let mut m = DhcpRelay::new("DhcpRelay");
        m.add_relay(sample_relay("a1", "Alpha")).unwrap();
        assert_eq!(m.relay_count(), 1);
        assert!(m.get_relay("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = DhcpRelay::new("DhcpRelay");
        assert!(m.add_relay(sample_relay("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = DhcpRelay::new("DhcpRelay");
        m.add_relay(sample_relay("x", "One")).unwrap();
        assert!(m.add_relay(sample_relay("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = DhcpRelay::new("DhcpRelay");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
