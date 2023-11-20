#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("./README.md")]

#[cfg(feature = "duration")]
#[cfg_attr(docsrs, doc(cfg(feature = "month")))]
pub use timext_month::duration;

#[cfg(feature = "partial")]
#[cfg_attr(docsrs, doc(cfg(feature = "parts")))]
pub use timext_parts::partial;

pub mod error {
    //! Various error types returned by methods in the crate.

    #[cfg(feature = "partial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "parts")))]
    pub use timext_parts::error::*;

    /// A unified error type for anything returned by a method in the [`timext`] crate.
    ///
    /// [`timext`]: crate
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        /// A unified error type for the [`time`] crate.
        #[error("{0}")]
        Base(time::Error),

        /// Partial conversion error.
        #[cfg(feature = "partial")]
        #[error("{0}")]
        TryFromPartial(TryFromPartial),
    }

    /// A specialized [`Result`] type for [`timext`] operations.
    ///
    /// [`Result`]: std::result::Result
    /// [`timext`]: crate
    pub type Result<T> = std::result::Result<T, Error>;
}

pub mod ext {
    //! Extension traits.

    #[cfg(feature = "duration")]
    #[cfg_attr(docsrs, doc(cfg(feature = "month")))]
    pub use timext_month::extension::*;

    #[cfg(feature = "partial")]
    #[cfg_attr(docsrs, doc(cfg(feature = "parts")))]
    pub use timext_parts::extension::*;
}
