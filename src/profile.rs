use std::fmt;

/// Explicit normal-runtime database capability selected before entering the ORM boundary.
///
/// `Migrator` is represented for configuration and audit layers, but this crate
/// deliberately refuses to open a normal application pool with migration authority.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityProfile {
    WebReadOnly,
    ApiReadWrite,
    WorkerReadOnly,
    WorkerReadWrite,
    Migrator,
}

impl CapabilityProfile {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WebReadOnly => "web_ro",
            Self::ApiReadWrite => "api_rw",
            Self::WorkerReadOnly => "worker_ro",
            Self::WorkerReadWrite => "worker_rw",
            Self::Migrator => "migrator",
        }
    }

    #[must_use]
    pub const fn allows_read(self) -> bool {
        matches!(
            self,
            Self::WebReadOnly
                | Self::ApiReadWrite
                | Self::WorkerReadOnly
                | Self::WorkerReadWrite
        )
    }

    #[must_use]
    pub const fn allows_write(self) -> bool {
        matches!(self, Self::ApiReadWrite | Self::WorkerReadWrite)
    }

    #[must_use]
    pub const fn is_migrator(self) -> bool {
        matches!(self, Self::Migrator)
    }
}

impl fmt::Display for CapabilityProfile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::CapabilityProfile;

    #[test]
    fn capability_matrix_is_fail_closed() {
        assert!(CapabilityProfile::WebReadOnly.allows_read());
        assert!(!CapabilityProfile::WebReadOnly.allows_write());
        assert!(CapabilityProfile::ApiReadWrite.allows_read());
        assert!(CapabilityProfile::ApiReadWrite.allows_write());
        assert!(CapabilityProfile::WorkerReadOnly.allows_read());
        assert!(!CapabilityProfile::WorkerReadOnly.allows_write());
        assert!(CapabilityProfile::WorkerReadWrite.allows_write());
        assert!(CapabilityProfile::Migrator.is_migrator());
        assert!(!CapabilityProfile::Migrator.allows_read());
        assert!(!CapabilityProfile::Migrator.allows_write());
    }

    #[test]
    fn profile_names_are_stable_machine_values() {
        assert_eq!(CapabilityProfile::WebReadOnly.as_str(), "web_ro");
        assert_eq!(CapabilityProfile::ApiReadWrite.as_str(), "api_rw");
        assert_eq!(CapabilityProfile::WorkerReadOnly.as_str(), "worker_ro");
        assert_eq!(CapabilityProfile::WorkerReadWrite.as_str(), "worker_rw");
        assert_eq!(CapabilityProfile::Migrator.as_str(), "migrator");
    }
}
