//! # Introduction
//!
//! The **_BasiliqStore_** crate is a helper crate for the [**_Basiliq_**](https://github.com/basiliqio/basiliq)
//! project.
//!
//! It allows to build the [BasiliqStore](BasiliqStore) necessary to run the [**_Basiliq_**](https://github.com/basiliqio/basiliq)
//! server.
//!
//! The [BasiliqStore](BasiliqStore) is built by scanning the database for metadatas and then merging configuration
//! provided by the user.

#![warn(clippy::all)]

mod postgres_metadata;
mod store;
use arcstr::ArcStr;

#[cfg(test)]
mod tests;

pub use postgres_metadata::parsed::BasiliqDbScannedTable;
pub use store::*;
