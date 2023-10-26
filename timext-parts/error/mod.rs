//! Various error types returned by methods in the crate.

use thiserror::Error;
use time::error::ComponentRange;

// TODO: Unified error type.

/// An error type indicating that an expected component was not found,
/// causing a failure.
#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, Hash)]
#[error("component `{name}` does not exist")]
pub struct NoComponent {
    /// Name of the component.
    pub(crate) name: &'static str,
}

impl NoComponent {
    /// Create the new error with the specified component name.
    pub(crate) fn new(component: &'static str) -> NoComponent {
        Self { name: component }
    }

    /// Obtain the name of the component that was not found.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// An error type indicating that an expected component was not found or
/// was out of range, causing a failure.
#[derive(Debug, Error)]
pub enum PartRange {
    /// Missing part.
    #[error("incomplete component: {0}")]
    InComplete(#[from] NoComponent),

    /// Out of range,
    #[error("incomplete component: {0}")]
    ComponentRange(#[from] ComponentRange),
}
