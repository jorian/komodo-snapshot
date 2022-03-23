#![feature(drain_filter)]

extern crate derive_more;

mod error;
pub mod snapshot;

pub use error::SnapshotError;
pub use snapshot::Snapshot;
