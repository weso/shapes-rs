use std::fs::File;
use srdf::SRDF;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use oxiri::Iri;

use oxrdf::{
    BlankNode as OxBlankNode, Literal as OxLiteral, NamedNode as OxNamedNode, Subject as OxSubject,
    Term as OxTerm, Triple as OxTriple, Graph, TripleRef,
};
use rio_api::model::{BlankNode, NamedNode, Subject, Term, Triple, Literal};
use rio_api::parser::*;
use rio_turtle::*;
use srdf::bnode::BNode;
use srdf::iri::IRI;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SRDFSPARQLError {
}

struct SRDFSPARQL {
    endpoint_iri: String
}

impl SRDF for SRDFSPARQL {
    type IRI = OxNamedNode;
    type BNode = OxBlankNode;
    type Literal = OxLiteral;
    type Subject = OxSubject;
    type Term = OxTerm;

    fn get_predicates_subject(&self, subject: &OxSubject) -> Vec<OxNamedNode> {
        todo!();
    } 
    fn get_objects_for_subject_predicate(&self, subject: &OxSubject, pred: &OxNamedNode) -> Vec<OxTerm> {
        todo!();
    }
    fn get_subjects_for_object_predicate(&self, object: &OxTerm, pred: &OxNamedNode) -> Vec<OxSubject> {
        todo!();
    }

    fn subject2iri(&self, subject:&OxSubject) -> Option<OxNamedNode> {
        match subject {
            OxSubject::NamedNode(n) => Some(n.clone()),
            _ => None
        }
    }
    fn subject2bnode(&self, subject:&OxSubject) -> Option<OxBlankNode> {
        match subject {
            OxSubject::BlankNode(b) => Some(b.clone()),
            _ => None
        }
    }
    fn subject_is_iri(&self, subject:&OxSubject) -> bool {
        match subject {
            OxSubject::NamedNode(_) => true,
            _ => false
        }
    }
    fn subject_is_bnode(&self, subject:&OxSubject) -> bool {
        match subject {
            OxSubject::BlankNode(_) => true,
            _ => false
        }
    }

    fn object2iri(&self, object:&OxTerm) -> Option<OxNamedNode> {
        match object {
            OxTerm::NamedNode(n) => Some(n.clone()),
            _ => None
        }
    }
    fn object2bnode(&self, object:&OxTerm) -> Option<OxBlankNode> {
        match object {
            OxTerm::BlankNode(b) => Some(b.clone()),
            _ => None
        }
    }
    fn object2literal(&self, object:&OxTerm) -> Option<OxLiteral> {
        match object {
            OxTerm::Literal(l) => Some(l.clone()),
            _ => None
        }
    }
    fn object_is_iri(&self, object: &OxTerm) -> bool {
        match object {
            OxTerm::NamedNode(_) => true,
            _ => false
        }
    }
    fn object_is_bnode(&self, object:&OxTerm) -> bool {
        match object {
            OxTerm::BlankNode(_) => true,
            _ => false
        }
    }

    fn object_is_literal(&self, object:&OxTerm) -> bool {
        match object {
            OxTerm::Literal(_) => true,
            _ => false
        }
    }

    fn lexical_form(&self, literal: &OxLiteral) -> String {
        literal.to_string()
    }
    fn lang(&self, literal: &OxLiteral) -> Option<String> {
        literal.language().map(|s| s.to_string())
    }
    fn datatype(&self, literal: &OxLiteral) -> OxNamedNode {
        literal.datatype().into_owned()
    }


}

fn cnv_subject(s: Subject) -> OxSubject {
    match s {
        Subject::NamedNode(n) => {
            OxSubject::NamedNode(OxNamedNode::new_unchecked(n.iri.to_string()))
        }
        Subject::BlankNode(b) => OxSubject::BlankNode(OxBlankNode::new_unchecked(b.id)),
        Subject::Triple(_) => todo!(),
    }
}

fn cnv_named_node(s: NamedNode) -> OxNamedNode {
    OxNamedNode::new_unchecked(s.iri)
}
fn cnv_literal(l: Literal) -> OxLiteral {
    match l {
        Literal::Simple { value } => OxLiteral::new_simple_literal(value.to_string()),
        Literal::LanguageTaggedString { value, language } => {
            OxLiteral::new_language_tagged_literal_unchecked(value, language)
        }
        Literal::Typed { value, datatype } => {
            OxLiteral::new_typed_literal(value, cnv_named_node(datatype))
        }
    }
}
fn cnv_object(s: Term) -> OxTerm {
    match s {
        Term::NamedNode(n) => {
            OxTerm::NamedNode(OxNamedNode::new_unchecked(n.iri.to_string()))
        }
        Term::BlankNode(b) => OxTerm::BlankNode(OxBlankNode::new_unchecked(b.id)),
        Term::Literal(l) => OxTerm::Literal(cnv_literal(l)),
        Term::Triple(_) => todo!(),
    }
}

fn cnv(t: Triple) -> OxTriple {
    OxTriple::new(
        cnv_subject(t.subject),
        cnv_named_node(t.predicate),
        cnv_object(t.object),
    )
}


#[cfg(test)]
mod tests {
}