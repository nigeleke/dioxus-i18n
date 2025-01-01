//! # dioxus-i18n 🌍
//!
//! i18n integration for Dioxus apps based on the [Project Fluent](https://github.com/projectfluent/fluent-rs).
//!
//! ## Example:
//!
//! ```ftl
//! # en-US.ftl
//! #
//! hello = Hello, {$name}!
//! ```
//!
//! ```rust
//! # use dioxus::prelude::*;
//! # use dioxus_i18n::{t, prelude::*};
//! # use unic_langid::langid;
//! # use std::path::PathBuf;
//! #
//! fn app() -> Element {
//!     let i18 = use_init_i18n(|| {
//!         I18nConfig::new(langid!("en-US"))
//!             .with_locale(( // Embed
//!                 langid!("en-US"),
//!                 include_str!("../examples/en-US.ftl"),
//!             ))
//!             .with_locale(( // Load at launch
//!                 langid!("es-ES"),
//!                 PathBuf::from("../examples/es-ES.ftl"),
//!             ))
//!     });
//!
//!     rsx!(
//!         label { { t!("hello", name: "World") } }
//!     )
//! }
//! ```
//!
pub mod i18n_macro;
pub mod use_i18n;

pub use fluent;
pub use unic_langid;

pub mod prelude {
    pub use crate::use_i18n::*;
}
