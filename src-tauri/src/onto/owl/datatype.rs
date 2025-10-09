use std::collections::HashMap;
use horned_owl::model::{AnnotationValue, DataRange, ForIRI};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::owl::annotation::wrap_annotations;
use crate::onto::serialize::DataRangeView;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DatatypeDetails<T>
where
    T: ForIRI + Default,
{
    pub annotations: HashMap<String, Vec<AnnotationValue<T>>>,
    pub definition: Option<DataRange<T>>,
}

impl<T: ForIRI + Default + Serialize> Serialize for DatatypeDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DatatypeDetails", 2)?;
        state.serialize_field("annotations", &wrap_annotations(&self.annotations))?;
        state.serialize_field("definition", &match &self.definition {
            None => None,
            Some(def) => Some(DataRangeView::from(def))
        })?;
        state.end()
    }
}
