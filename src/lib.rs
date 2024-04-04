#![deny(missing_docs)]
//! A collection of random Yeq function components
//!
//! The components are:
//!
//! * Table
//! * Tabs
//!
//! The collection can be expanded as the need arises
//! Underlying CSS is [Bootstrap](https://getbootstrap.com/docs/5.3/getting-started/introduction/), but we are trying to make this customizable if at all possible

#[cfg(feature="table")]
pub mod table;
#[cfg(feature="tabs")]
pub mod tabs;
#[cfg(feature="pagination")]
pub mod pagination;
