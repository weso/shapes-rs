use std::collections::HashSet;

use srdf::{QuerySRDF, SRDFBasic, SRDF};

use crate::constraints::constraint_error::ConstraintError;
use crate::constraints::ConstraintComponent;
use crate::constraints::DefaultConstraintComponent;
use crate::constraints::SparqlConstraintComponent;
use crate::validation_report::report::ValidationReport;

/// The property sh:uniqueLang can be set to true to specify that no pair of
///  value nodes may use the same language tag.
///
/// https://www.w3.org/TR/shacl/#UniqueLangConstraintComponent
pub(crate) struct UniqueLang {
    unique_lang: bool,
}

impl UniqueLang {
    pub fn new(unique_lang: bool) -> Self {
        UniqueLang { unique_lang }
    }
}

impl<S: SRDFBasic> ConstraintComponent<S> for UniqueLang {
    fn evaluate(
        &self,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        if !self.unique_lang {
            return Ok(());
        }
        let mut langs = HashSet::new();
        for node in &value_nodes {
            if let Some(literal) = S::term_as_literal(node) {
                if let Some(lang) = S::lang(&literal) {
                    if langs.contains(&lang) {
                        report.make_validation_result(Some(node));
                    }
                    langs.insert(lang);
                }
            }
        }
        Ok(())
    }
}

impl<S: SRDF> DefaultConstraintComponent<S> for UniqueLang {
    fn evaluate_default(
        &self,
        _: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}

impl<S: QuerySRDF> SparqlConstraintComponent<S> for UniqueLang {
    fn evaluate_sparql(
        &self,
        _: &S,
        value_nodes: HashSet<S::Term>,
        report: &mut ValidationReport<S>,
    ) -> Result<(), ConstraintError> {
        self.evaluate(value_nodes, report)
    }
}
