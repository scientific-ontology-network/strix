use std::collections::{HashMap, HashSet};
use horned_owl::model::{AnnotationValue, ClassExpression, ForIRI, Individual};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::owl::annotation::wrap_annotations;
use crate::onto::serialize::{ClassExpressionView, IndividualView};


#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct IndividualDetails<T>
where
    T: ForIRI + Default,
{
    pub annotations: HashMap<String, Vec<AnnotationValue<T>>>,

    pub sames: HashSet<Individual<T>>,
    pub different: HashSet<Individual<T>>,
    pub member_of: HashSet<ClassExpression<T>>,
}

impl<T: ForIRI + Default + Serialize> Serialize for IndividualDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("IndividualDetails", 4)?;
        state.serialize_field("annotations", &wrap_annotations(&self.annotations))?;
        state.serialize_field("sames", &self.sames.iter().map(IndividualView::from).collect::<Vec<_>>())?;
        state.serialize_field("different", &self.different.iter().map(IndividualView::from).collect::<Vec<_>>())?;
        state.serialize_field("member_of", &self.member_of.iter().map(ClassExpressionView::from).collect::<Vec<_>>())?;
        state.end()
    }
}

