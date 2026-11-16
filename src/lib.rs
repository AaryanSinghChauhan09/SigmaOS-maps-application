// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-maps-application: maps application component model.
//!
//! Real implementation: maps, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
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
pub enum MapsApplicationError {
    EmptyMapName,
    DuplicateMap(String),
    EmptyEventSubject,
}

impl fmt::Display for MapsApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyMapName => write!(f, "map name must not be empty"),
            Self::DuplicateMap(id) => write!(f, "duplicate map {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for MapsApplicationError {}

pub struct MapsApplication {
    pub name: String,
    maps: BTreeMap<String, Map>,
    events: BTreeMap<String, Event>,
}

impl MapsApplication {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            maps: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_map(&mut self, item: Map) -> Result<(), MapsApplicationError> {
        if item.name.trim().is_empty() {
            return Err(MapsApplicationError::EmptyMapName);
        }
        let id = item.id.clone();
        if self.maps.contains_key(&id) {
            return Err(MapsApplicationError::DuplicateMap(id.clone()));
        }
        self.maps.insert(id, item);
        Ok(())
    }

    pub fn map_count(&self) -> usize {
        self.maps.len()
    }

    pub fn get_map(&self, id: &str) -> Option<&Map> {
        self.maps.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), MapsApplicationError> {
        if event.subject.trim().is_empty() {
            return Err(MapsApplicationError::EmptyEventSubject);
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

    fn sample_map(id: &str, name: &str) -> Map {
        Map {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = MapsApplication::new("MapsApplication");
        assert_eq!(m.name, "MapsApplication");
        assert_eq!(m.map_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_map() {
        let mut m = MapsApplication::new("MapsApplication");
        m.add_map(sample_map("a1", "Alpha")).unwrap();
        assert_eq!(m.map_count(), 1);
        assert!(m.get_map("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = MapsApplication::new("MapsApplication");
        assert!(m.add_map(sample_map("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = MapsApplication::new("MapsApplication");
        m.add_map(sample_map("x", "One")).unwrap();
        assert!(m.add_map(sample_map("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = MapsApplication::new("MapsApplication");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
