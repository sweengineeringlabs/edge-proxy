//! SPI extension point for the `job` domain.
//!
//! No external-lib-backed `Job` variant exists yet — every implementation in
//! this crate lives in `core/job/`. This module marks the extension point for
//! a future external-lib-backed strategy without providing one prematurely.
