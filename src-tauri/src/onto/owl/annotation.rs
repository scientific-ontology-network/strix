use std::collections::HashMap;
use horned_owl::model::{AnnotationValue, ForIRI};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::serialize::AnnotationValueView;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AnnotationPropertyDetails<T>
where
    T: ForIRI + Default,
{
    pub annotations: HashMap<String, Vec<AnnotationValue<T>>>,
}

impl<T: ForIRI + Default + Serialize> Serialize for AnnotationPropertyDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AnnotationPropertyDetails", 1)?;
        state.serialize_field("annotations", &wrap_annotations(&self.annotations))?;
        state.end()
    }
}

pub(crate) fn wrap_annotations<T: ForIRI>(annotations: &HashMap<String, Vec<AnnotationValue<T>>>) -> HashMap<String, Vec<AnnotationValueView>> {
    HashMap::from_iter(annotations.iter().map(|(iri, values)| (iri.clone(), values.iter().map(|v|AnnotationValueView::from(v)).collect())))
}