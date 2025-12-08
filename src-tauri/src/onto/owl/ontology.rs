use std::collections::HashMap;
use horned_owl::model::ForIRI;
use crate::onto::serialize::AnnotationValueView;

#[derive(Default)]
pub struct OntologyDetails<T: ForIRI> {
    pub annotations: HashMap<T, Vec<AnnotationValueView>>,
    pub imports: Vec<T>,
    pub ontology_iri: Option<T>,
}
