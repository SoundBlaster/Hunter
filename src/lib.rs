//! Reusable filesystem primitives, independent of product classification.
#[cfg(unix)]
mod scoped_metadata;
#[cfg(unix)]
pub use scoped_metadata::{FileFact, Location, Observation, ScopedMetadata, Unavailable};
