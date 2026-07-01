//! Constructors for [`ComponentHealth`].

use crate::api::{ComponentHealth, HealthStatus};

impl ComponentHealth {
    /// Construct a `Healthy` component entry with no message.
    pub fn healthy(id: impl Into<String>) -> Self {
        Self::with_fields(id, HealthStatus::Healthy, None)
    }

    /// Construct a non-healthy component entry with a reason.
    pub fn with_status(
        id: impl Into<String>,
        status: HealthStatus,
        message: impl Into<String>,
    ) -> Self {
        Self::with_fields(id, status, Some(message.into()))
    }

    /// Shared field assembly for the public constructors above.
    fn with_fields(id: impl Into<String>, status: HealthStatus, message: Option<String>) -> Self {
        Self {
            id: id.into(),
            status,
            message,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// @covers: healthy
    #[test]
    fn test_healthy_sets_healthy_status() {
        let c = ComponentHealth::healthy("svc");
        assert_eq!(c.status, HealthStatus::Healthy);
        assert!(c.message.is_none());
    }

    /// @covers: with_status
    #[test]
    fn test_with_status_sets_given_status() {
        let c = ComponentHealth::with_status("svc", HealthStatus::Degraded, "slow");
        assert_eq!(c.status, HealthStatus::Degraded);
    }

    /// @covers: with_fields
    #[test]
    fn test_with_fields_assembles_given_values() {
        let c = ComponentHealth::with_fields("svc", HealthStatus::Degraded, Some("slow".into()));
        assert_eq!(c.id, "svc");
        assert_eq!(c.status, HealthStatus::Degraded);
        assert_eq!(c.message.as_deref(), Some("slow"));
    }
}
