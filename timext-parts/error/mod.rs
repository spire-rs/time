//! Various error types returned by methods in the crate.

/// An error type indicating that an expected component was not found,
/// causing a conversion failure.
#[derive(Debug, thiserror::Error)]
#[error("component `{name}` was not found")]
pub struct PartialVariant {
    /// Name of the component.
    pub(crate) name: &'static str,
}

impl PartialVariant {
    /// Creates the new error with the specified component name.
    pub(crate) fn new(component: &'static str) -> PartialVariant {
        Self { name: component }
    }

    /// Obtains the name of the component that was not found.
    pub fn name(&self) -> &'static str {
        self.name
    }
}

/// An error type indicating that an expected component was not found or
/// was out of range, causing a conversion failure.
#[derive(Debug, thiserror::Error)]
pub enum TryFromPartial {
    /// Missing part.
    #[error("partial component: {0}")]
    Partial(#[from] PartialVariant),

    /// Out of range,
    #[error("out of range: {0}")]
    ComponentRange(#[from] time::error::ComponentRange),
}
