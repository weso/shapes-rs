//! DCTAP Processor
//!
//! This module contains a simple [DCTAP](https://www.dublincore.org/specifications/dctap/) processor
//!
//!
//! DCTAP (Dublin Core Tabular Application Profiles) is a simple model that can be used to specify data models
//!
pub mod datatype_id;
pub mod dctap;
pub mod node_type;
pub mod prefix_cc;
pub mod property_id;
pub mod shape_id;
pub mod tap_config;
pub mod tap_error;
pub mod tap_headers;
pub mod tap_reader;
pub mod tap_reader_builder;
pub mod tap_reader_state;
pub mod tap_shape;
pub mod tap_statement;
pub mod value_constraint;

pub use crate::datatype_id::*;
pub use crate::node_type::*;
pub use crate::prefix_cc::*;
pub use crate::property_id::*;
pub use crate::shape_id::*;
pub use crate::tap_config::*;
pub use crate::tap_error::*;
pub use crate::tap_reader::*;
pub use crate::tap_reader_builder::*;
pub use crate::tap_reader_state::*;
pub use crate::tap_shape::*;
pub use crate::tap_statement::*;
pub use crate::value_constraint::*;
pub use dctap::*;
