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
