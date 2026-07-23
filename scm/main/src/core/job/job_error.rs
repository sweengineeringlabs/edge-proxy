//! Conversion from the foreign [`edge_application_handler::HandlerError`] to
//! [`JobError`] — the one place this crate references the upstream error's
//! exact shape. `JobError::Handler` carries only its formatted message, so
//! `api/` stays decoupled from upstream's variant set.

use crate::api::JobError;

impl From<edge_application_handler::HandlerError> for JobError {
    fn from(inner: edge_application_handler::HandlerError) -> Self {
        JobError::Handler(inner.to_string())
    }
}
