mod rdf_parser;
mod rdf_parser_error;
mod rdf_node_parser;
mod focus_rdf;

pub use focus_rdf::*;
pub use rdf_parser::*;
pub use rdf_node_parser::*;
pub use rdf_parser_error::*;

type PResult<A> = Result<A, RDFParseError>;