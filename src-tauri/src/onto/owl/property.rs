use std::collections::{HashMap};
use horned_owl::model::{AnnotationValue, Class, ClassExpression, DataProperty, DataRange, ForIRI, ObjectProperty, ObjectPropertyExpression, SubObjectPropertyExpression, IRI};
use strix_roost::dependency::base::OntologySymbol;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::serialize::{AnnotationValueView, ClassExpressionView, DataPropertyView, DataRangeView, ObjectPropertyExpressionView, OntologySymbolView, SubObjectPropertyExpressionView};
use crate::onto::owl::class::ClassDetails;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectPropertyDetails<T>
where
    T: ForIRI,
{
    pub annotations:  HashMap<T, Vec<AnnotationValueView>>,

    pub subproperty_of: Vec<ObjectPropertyExpressionView>,
    pub superproperty_of: Vec<SubObjectPropertyExpressionView>,
    pub equivalent_to: Vec<ObjectPropertyExpressionView>,
    pub disjoint_with: Vec<ObjectPropertyExpressionView>,
    pub inverse_of: Vec<ObjectPropertyExpressionView>,
    pub domain: Vec<ClassExpressionView>,
    pub range: Vec<ClassExpressionView>,

    pub is_functional: bool,
    pub is_inverse_functional: bool,
    pub is_reflexive: bool,
    pub is_irreflexive: bool,
    pub is_symmetric: bool,
    pub is_asymmetric: bool,
    pub is_transitive: bool,

    pub depends_on: Vec<OntologySymbolView>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DataPropertyDetails<T>
where
    T: ForIRI,
{
    pub annotations: HashMap<T, Vec<AnnotationValueView>>,

    pub(crate) subproperty_of: Vec<DataPropertyView>,
    pub(crate) superproperty_of: Vec<DataPropertyView>,
    pub equivalent_to: Vec<DataPropertyView>,
    pub disjoint_with: Vec<DataPropertyView>,
    pub domain: Vec<ClassExpressionView>,
    pub range: Vec<DataRangeView>,

    pub(crate) is_functional: bool,
}

impl<T: ForIRI + Serialize> Serialize for ObjectPropertyDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ObjectPropertyDetails", 15)?;
        state.serialize_field("annotations", &self.annotations)?;
        state.serialize_field("subproperty_of", &self.subproperty_of)?;
        state.serialize_field("superproperty_of", &self.superproperty_of)?;
        state.serialize_field("equivalent_to", &self.equivalent_to)?;
        state.serialize_field("disjoint_with", &self.disjoint_with)?;
        state.serialize_field("inverse_of", &self.inverse_of)?;
        state.serialize_field("domain", &self.domain)?;
        state.serialize_field("range", &self.range)?;
        state.serialize_field("is_functional", &self.is_functional)?;
        state.serialize_field("is_inverse_functional", &self.is_inverse_functional)?;
        state.serialize_field("is_reflexive", &self.is_reflexive)?;
        state.serialize_field("is_irreflexive", &self.is_irreflexive)?;
        state.serialize_field("is_symmetric", &self.is_symmetric)?;
        state.serialize_field("is_asymmetric", &self.is_asymmetric)?;
        state.serialize_field("is_transitive", &self.is_transitive)?;
        state.serialize_field("depends_on", &self.depends_on)?;
        state.end()
    }
}

impl<T: ForIRI + Default + Serialize> Serialize for DataPropertyDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("DataPropertyDetails", 8)?;
        state.serialize_field("annotations", &self.annotations)?;
        state.serialize_field("subproperty_of", &self.subproperty_of)?;
        state.serialize_field("superproperty_of", &self.superproperty_of)?;
        state.serialize_field("equivalent_to", &self.equivalent_to)?;
        state.serialize_field("disjoint_with", &self.disjoint_with)?;
        state.serialize_field("domain", &self.domain)?;
        state.serialize_field("range", &self.range)?;
        state.serialize_field("is_functional", &self.is_functional)?;
        state.end()
    }
}

impl<'a,T: ForIRI> Default for ObjectPropertyDetails<T>{
    fn default() -> Self {
        ObjectPropertyDetails{
            annotations: Default::default(),
            subproperty_of: vec![],
            superproperty_of: vec![],
            equivalent_to: vec![],
            disjoint_with: vec![],
            inverse_of: vec![],
            domain: vec![],
            range: vec![],
            is_functional: false,
            is_inverse_functional: false,
            is_reflexive: false,
            is_irreflexive: false,
            is_symmetric: false,
            is_asymmetric: false,
            is_transitive: false,
            depends_on: vec![],
        }
    }
}

impl<'a,T: ForIRI> Default for DataPropertyDetails<T>{
    fn default() -> Self {
        DataPropertyDetails{
            annotations: Default::default(),
            subproperty_of: vec![],
            superproperty_of: vec![],
            equivalent_to: vec![],
            disjoint_with: vec![],
            domain: vec![],
            range: vec![],
            is_functional: false,
        }
    }
}
