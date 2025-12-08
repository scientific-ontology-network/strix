use std::collections::HashMap;
use horned_owl::model::ForIRI;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::serialize::{AnnotationValueView, DataRangeView};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatatypeDetails<T>
where
    T: ForIRI,
{
    pub annotations: HashMap<T, Vec<AnnotationValueView>>,
    pub definition: Option<DataRangeView>,
}

impl<'a, T: ForIRI + Serialize> Serialize for DatatypeDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DatatypeDetails", 2)?;
        state.serialize_field("annotations",&self.annotations)?;
        state.serialize_field("definition", &self.definition)?;
        state.end()
    }
}

impl<'a,T: ForIRI> Default for DatatypeDetails<T>{
    fn default() -> Self {
        DatatypeDetails{ annotations: Default::default(), definition: None }
    }
}