use std::collections::{HashMap, HashSet};
use horned_owl::model::{AnnotationValue, ClassExpression, DataProperty, DataRange, ForIRI, ObjectProperty, ObjectPropertyExpression, SubObjectPropertyExpression};
use semantic_dependency::dependency::base::OntologySymbol;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::serialize::{ClassExpressionView, DataRangeView, ObjectPropertyExpressionView, OntologySymbolView, SubObjectPropertyExpressionView};
use crate::onto::owl::annotation::wrap_annotations;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ObjectPropertyDetails<T>
where
    T: ForIRI + Default,
{
    pub annotations: HashMap<String, Vec<AnnotationValue<T>>>,

    pub subproperty_of: HashSet<ObjectPropertyExpression<T>>,
    pub superproperty_of: HashSet<SubObjectPropertyExpression<T>>,
    pub equivalent_to: HashSet<ObjectPropertyExpression<T>>,
    pub disjoint_with: HashSet<ObjectPropertyExpression<T>>,
    pub inverse_of: HashSet<ObjectProperty<T>>,
    pub domain: HashSet<ClassExpression<T>>,
    pub range: HashSet<ClassExpression<T>>,

    pub is_functional: bool,
    pub is_inverse_functional: bool,
    pub is_reflexive: bool,
    pub is_irreflexive: bool,
    pub is_symmetric: bool,
    pub is_asymmetric: bool,
    pub is_transitive: bool,

    pub depends_on: HashSet<OntologySymbol<T>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct DataPropertyDetails<T>
where
    T: ForIRI + Default,
{
    pub annotations: HashMap<String, Vec<AnnotationValue<T>>>,

    pub(crate) subproperty_of: HashSet<DataProperty<T>>,
    pub(crate) superproperty_of: HashSet<DataProperty<T>>,
    pub equivalent_to: HashSet<DataProperty<T>>,
    pub disjoint_with: HashSet<DataProperty<T>>,
    pub domain: HashSet<ClassExpression<T>>,
    pub range: HashSet<DataRange<T>>,

    pub(crate) is_functional: bool,
}

impl<T: ForIRI + Default + Serialize> Serialize for ObjectPropertyDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ObjectPropertyDetails", 15)?;
        state.serialize_field("annotations", &wrap_annotations(&self.annotations))?;
        state.serialize_field("subproperty_of", &self.subproperty_of.iter().map(ObjectPropertyExpressionView::from).collect::<Vec<_>>())?;
        state.serialize_field("superproperty_of", &self.superproperty_of.iter().map(SubObjectPropertyExpressionView::from).collect::<Vec<_>>())?;
        state.serialize_field("equivalent_to", &self.equivalent_to.iter().map(ObjectPropertyExpressionView::from).collect::<Vec<_>>())?;
        state.serialize_field("disjoint_with", &self.disjoint_with.iter().map(ObjectPropertyExpressionView::from).collect::<Vec<_>>())?;
        state.serialize_field("inverse_of", &self.inverse_of.iter().map(|c|c.to_string()).collect::<Vec<_>>())?;
        state.serialize_field("domain", &self.domain.iter().map(ClassExpressionView::from).collect::<Vec<_>>())?;
        state.serialize_field("range", &self.range.iter().map(ClassExpressionView::from).collect::<Vec<_>>())?;
        state.serialize_field("is_functional", &self.is_functional)?;
        state.serialize_field("is_inverse_functional", &self.is_inverse_functional)?;
        state.serialize_field("is_reflexive", &self.is_reflexive)?;
        state.serialize_field("is_irreflexive", &self.is_irreflexive)?;
        state.serialize_field("is_symmetric", &self.is_symmetric)?;
        state.serialize_field("is_asymmetric", &self.is_asymmetric)?;
        state.serialize_field("is_transitive", &self.is_transitive)?;
        state.serialize_field("depends_on", &self.depends_on.iter().map(|c| OntologySymbolView::from(c)).collect::<Vec<_>>())?;
        state.end()
    }
}

impl<T: ForIRI + Default + Serialize> Serialize for DataPropertyDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DataPropertyDetails", 8)?;
        state.serialize_field("annotations", &wrap_annotations(&self.annotations))?;
        state.serialize_field("subproperty_of", &self.subproperty_of.iter().map(|p| p.to_string()).collect::<Vec<String>>())?;
        state.serialize_field("superproperty_of", &self.superproperty_of.iter().map(|p| p.to_string()).collect::<Vec<String>>())?;
        state.serialize_field("equivalent_to", &self.equivalent_to.iter().map(|p| p.to_string()).collect::<Vec<_>>())?;
        state.serialize_field("disjoint_with", &self.disjoint_with.iter().map(|p| p.to_string()).collect::<Vec<_>>())?;
        state.serialize_field("domain", &self.domain.iter().map(ClassExpressionView::from).collect::<Vec<_>>())?;
        state.serialize_field("range", &self.range.iter().map(DataRangeView::from).collect::<Vec<_>>())?;
        state.serialize_field("is_functional", &self.is_functional)?;
        state.end()
    }
}


