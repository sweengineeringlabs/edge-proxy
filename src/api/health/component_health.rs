//! ComponentHealth — health snapshot for a single component.

use serde::{Deserialize, Serialize};

use super::health_status::HealthStatus;

/// Health snapshot for a single component (handler, subsystem, backend).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentHealth {
    /// Stable identifier matching `Handler::id` for handler components.
    pub id: String,
    /// Classification at the moment the probe ran.
    pub status: HealthStatus,
    /// Optional human-readable reason when status is not `Healthy`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl ComponentHealth {
    /// Construct a `Healthy` component entry with no message.
    pub fn healthy(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            status: HealthStatus::Healthy,
            message: None,
        }
    }

    /// Construct a non-healthy component entry with a reason.
    pub fn with_status(
        id: impl Into<String>,
        status: HealthStatus,
        message: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            status,
            message: Some(message.into()),
        }
    }
}
