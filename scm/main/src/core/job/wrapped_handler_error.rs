//! Conversions and accessor for [`WrappedHandlerError`].

use crate::api::{JobError, WrappedHandlerError};

impl WrappedHandlerError {
    /// The wrapped [`edge_application_handler::HandlerError`].
    pub fn inner(&self) -> &edge_application_handler::HandlerError {
        &self.inner
    }
}

impl From<edge_application_handler::HandlerError> for WrappedHandlerError {
    fn from(inner: edge_application_handler::HandlerError) -> Self {
        Self { inner }
    }
}

impl From<edge_application_handler::HandlerError> for JobError {
    fn from(inner: edge_application_handler::HandlerError) -> Self {
        JobError::Handler(WrappedHandlerError::from(inner))
    }
}
