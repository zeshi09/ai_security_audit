use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Finding {
    pub id: String,
    pub title: String,
    pub severity: Severity,
    pub description: String,
    pub target: String,
    pub evidence: Option<String>,
    pub created_at: u64,
}

impl Finding {
    pub fn with_evidence(&self) -> bool {
        self.evidence.is_some()
    }

    pub fn is_critical(&self) -> bool {
        self.severity == Severity::Critical
    }

    pub fn new(
        title: impl Into<String>,
        severity: Severity,
        description: impl Into<String>,
        target: impl Into<String>,
    ) -> Self {
        Self {
            id: generate_id(),
            title: title.into(),
            severity,
            description: description.into(),
            target: target.into(),
            evidence: None,
            created_at: current_timestamp(),
        }
    }
}

fn current_timestamp() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn generate_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    format!("finding-{:x}", nanos)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ScanTarget {
    OpenApi { url: String },
    Endpoint { url: String, method: String },
    File { path: String },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn severity_ordering() {
        assert!(Severity::Info < Severity::Critical);
    }

    #[test]
    fn finding_has_evidence() {
        // создай Finding с evidence и проверь что поле Some
        let finding = Finding {
            id: String::from("test-1"),
            title: String::from("Missing auth"),
            severity: Severity::High,
            description: String::from("No authentication"),
            target: String::from("https://api.example.com"),
            evidence: Some(String::from("GET /users returned 200")),
            created_at: 0,
        };
        assert!(finding.evidence.is_some());
        assert_eq!(finding.severity, Severity::High);
    }
}
