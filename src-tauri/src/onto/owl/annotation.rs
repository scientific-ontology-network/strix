use std::collections::HashMap;
use horned_owl::model::ForIRI;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::serialize::AnnotationValueView;

pub struct AnnotationPropertyDetails< T>
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

impl<T: ForIRI> Default for AnnotationPropertyDetails<T>{
    fn default() -> Self {
        AnnotationPropertyDetails{
            annotations: Default::default(),
        }
    }
}