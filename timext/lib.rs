#![forbid(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

#[cfg(feature = "month")]
#[cfg_attr(docsrs, doc(cfg(feature = "month")))]
pub use timext_month::duration;

#[cfg(feature = "parts")]
#[cfg_attr(docsrs, doc(cfg(feature = "parts")))]
pub use timext_parts::partial;

pub mod error {
    #[cfg(feature = "parts")]
    #[cfg_attr(docsrs, doc(cfg(feature = "parts")))]
    pub use timext_parts::error::*;
}

pub mod ext {
    #[cfg(feature = "month")]
    #[cfg_attr(docsrs, doc(cfg(feature = "month")))]
    pub use timext_month::extension::*;

    #[cfg(feature = "parts")]
    #[cfg_attr(docsrs, doc(cfg(feature = "parts")))]
    pub use timext_parts::extension::*;
}