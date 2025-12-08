use std::collections::{HashMap};
use horned_owl::model::ForIRI;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::serialize::{AnnotationValueView, ClassExpressionView, IndividualView};


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndividualDetails<T>
where
    T: ForIRI,
{
    pub annotations: HashMap<T, Vec<AnnotationValueView>>,

    pub sames: Vec<IndividualView>,
    pub different: Vec<IndividualView>,
    pub member_of: Vec<ClassExpressionView>,
}

impl<'a, T: ForIRI + Serialize> Serialize for IndividualDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("IndividualDetails", 4)?;
        state.serialize_field("annotations", &self.annotations)?;
        state.serialize_field("sames", &self.sames)?;
        state.serialize_field("different", &self.different)?;
        state.serialize_field("member_of", &self.member_of)?;
        state.end()
    }
}

impl<T: ForIRI> Default for IndividualDetails<T>{
    fn default() -> Self {
        IndividualDetails{
            annotations: Default::default(),
            sames: vec![],
            different: vec![],
            member_of: vec![],
        }
    }
}