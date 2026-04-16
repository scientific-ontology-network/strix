use crate::onto::serialize::AnnotationValueView;
use horned_owl::model::ForIRI;
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::HashMap;

pub struct AnnotationPropertyDetails<T>
where
    T: ForIRI,
{
    pub annotations: HashMap<T, Vec<AnnotationValueView>>,
}

impl<'a, T: ForIRI + Serialize> Serialize for AnnotationPropertyDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("AnnotationPropertyDetails", 1)?;
        state.serialize_field("annotations", &self.annotations)?;
        state.end()
    }
}

impl<T: ForIRI> Default for AnnotationPropertyDetails<T> {
    fn default() -> Self {
        AnnotationPropertyDetails {
            annotations: Default::default(),
        }
    }
}
