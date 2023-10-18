use std::str::FromStr;

use iri_s::{IriS, IriSError};
use serde_derive::{Deserialize, Serialize};
use void::Void;

use super::FromStrRefError;

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(try_from = "&str", into = "String")]
pub enum Ref {
    IriRef { value: IriS },
    BNode { value: String },
}

impl Ref {
    pub fn iri_unchecked(s: &str) -> Ref {
        Ref::IriRef {
            value: IriS::new_unchecked(s),
        }
    }

    pub fn bnode_unchecked(s: &str) -> Ref {
        Ref::BNode {
            value: s.to_string(),
        }
    }
}

impl Into<String> for Ref {
    fn into(self) -> String {
        match self {
            Ref::IriRef { value } => value.to_string(),
            Ref::BNode { value } => value,
        }
    }
}

impl TryFrom<&str> for Ref {
    type Error = IriSError;

    // TODO: We should parse the bnode
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let iri_s = IriS::from_str(s)?;
        Ok(Ref::IriRef { value: iri_s })
    }
}

impl From<IriS> for Ref {
    fn from(iri: IriS) -> Ref {
        Ref::iri_unchecked(iri.as_str())
    }
}

impl FromStr for Ref {
    type Err = IriSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let iri_s = IriS::from_str(s)?;
        Ok(Ref::IriRef {
            value: iri_s,
        })
    }
}
