// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-boot-performance-analyzer: boot performance analyzer component model.
//!
//! Real implementation: analyzers, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Analyzer {
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
pub enum BootPerformanceAnalyzerError {
    EmptyAnalyzerName,
    DuplicateAnalyzer(String),
    EmptyEventSubject,
}

impl fmt::Display for BootPerformanceAnalyzerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAnalyzerName => write!(f, "analyzer name must not be empty"),
            Self::DuplicateAnalyzer(id) => write!(f, "duplicate analyzer {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for BootPerformanceAnalyzerError {}

pub struct BootPerformanceAnalyzer {
    pub name: String,
    analyzers: BTreeMap<String, Analyzer>,
    events: BTreeMap<String, Event>,
}

impl BootPerformanceAnalyzer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            analyzers: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_analyzer(&mut self, item: Analyzer) -> Result<(), BootPerformanceAnalyzerError> {
        if item.name.trim().is_empty() {
            return Err(BootPerformanceAnalyzerError::EmptyAnalyzerName);
        }
        let id = item.id.clone();
        if self.analyzers.contains_key(&id) {
            return Err(BootPerformanceAnalyzerError::DuplicateAnalyzer(id.clone()));
        }
        self.analyzers.insert(id, item);
        Ok(())
    }

    pub fn analyzer_count(&self) -> usize {
        self.analyzers.len()
    }

    pub fn get_analyzer(&self, id: &str) -> Option<&Analyzer> {
        self.analyzers.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), BootPerformanceAnalyzerError> {
        if event.subject.trim().is_empty() {
            return Err(BootPerformanceAnalyzerError::EmptyEventSubject);
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

    fn sample_analyzer(id: &str, name: &str) -> Analyzer {
        Analyzer {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = BootPerformanceAnalyzer::new("BootPerformanceAnalyzer");
        assert_eq!(m.name, "BootPerformanceAnalyzer");
        assert_eq!(m.analyzer_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_analyzer() {
        let mut m = BootPerformanceAnalyzer::new("BootPerformanceAnalyzer");
        m.add_analyzer(sample_analyzer("a1", "Alpha")).unwrap();
        assert_eq!(m.analyzer_count(), 1);
        assert!(m.get_analyzer("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = BootPerformanceAnalyzer::new("BootPerformanceAnalyzer");
        assert!(m.add_analyzer(sample_analyzer("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = BootPerformanceAnalyzer::new("BootPerformanceAnalyzer");
        m.add_analyzer(sample_analyzer("x", "One")).unwrap();
        assert!(m.add_analyzer(sample_analyzer("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = BootPerformanceAnalyzer::new("BootPerformanceAnalyzer");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}

/// Module extracted from the SigmaOS-system monolith.
pub mod boot_analyzer;
