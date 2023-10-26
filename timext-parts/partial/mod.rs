//! Incomplete time formats.

mod date;
mod offset;
mod primitive;
mod time;

pub use date::*;
pub use offset::*;
pub use primitive::*;
pub use time::*;

use crate::error::TryFromPartial;

/// A set of methods required to implement a partial timestamp.
pub trait Partial: Sized {
    /// The complete timestamp type that corresponds to the partial timestamp.
    type Complete;

    /// Converts the complete timestamp into the partial one.
    fn from_complete(complete: Self::Complete) -> Self;

    /// Converts the partial timestamp into the complete one.
    fn into_complete(self) -> Result<Self::Complete, TryFromPartial>;

    /// Completes partial components of the partial timestamp.
    fn with_fallback(self, fallback: Self::Complete) -> Result<Self, TryFromPartial>;

    /// Completes partial components of the partial timestamp
    /// and then converts into the complete timestamp.
    fn fallback(self, fallback: Self::Complete) -> Result<Self::Complete, TryFromPartial> {
        self.with_fallback(fallback)?.into_complete()
    }
}
