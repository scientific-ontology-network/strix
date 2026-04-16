use crate::onto::serialize::AnnotationValueView;
use horned_owl::model::ForIRI;
use std::collections::HashMap;

#[derive(Default)]
pub struct OntologyDetails<T: ForIRI> {
    pub annotations: HashMap<T, Vec<AnnotationValueView>>,
    pub imports: Vec<T>,
    pub ontology_iri: Option<T>,
}
