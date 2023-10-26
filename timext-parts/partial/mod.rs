//! Incomplete time formats.

mod date;
mod offset;
mod primitive;
mod time;

pub use date::*;
pub use offset::*;
pub use primitive::*;
pub use time::*;

use crate::error::PartRange;

/// A set of methods required to implement an incomplete timestamp.
pub trait Partial: Sized {
    type Complete;

    /// Converts the complete timestamp into incomplete one.
    fn from_complete(complete: Self::Complete) -> Self;

    /// Converts the incomplete timestamp into complete one.
    fn into_complete(self) -> Result<Self::Complete, PartRange>;

    /// Completes incomplete components of the incomplete timestamp.
    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, PartRange>;

    /// Completes incomplete components of the incomplete timestamp
    /// and then converts into the complete timestamp.
    fn fallback(self, fallback: Self::Complete) -> Result<Self::Complete, PartRange> {
        self.with_fallback(fallback)?.into_complete()
    }
}
