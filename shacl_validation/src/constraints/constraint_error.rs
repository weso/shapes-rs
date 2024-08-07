use thiserror::Error;

use crate::helper::helper_error::SPARQLError;

#[allow(clippy::upper_case_acronyms)]
#[derive(Error, Debug)]
pub enum ConstraintError {
    #[error("Error during the SPARQL operation")]
    SPARQL(#[from] SPARQLError),
    #[error("Not yet implemented Constraint")]
    NotImplemented,
    #[error("Error creating the constriant")]
    Create,
    #[error("Error during some of the query operations")]
    Query,
}
