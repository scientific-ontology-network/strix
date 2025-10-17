use std::collections::{HashMap, HashSet};
use horned_owl::model::{AnnotationValue, Class, ClassExpression, ForIRI, Individual};
use strix_roost::dependency::base::OntologySymbol;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use ts_rs::TS;
use crate::onto::owl::annotation::wrap_annotations;
use crate::onto::serialize::{ClassExpressionView, IndividualView, OntologySymbolView};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct ClassDetails<T>
where
    T: ForIRI + Default,
{
    pub annotations: HashMap<String, Vec<AnnotationValue<T>>>,

    pub subclass_of: HashSet<ClassExpression<T>>,
    pub equivalent_to: HashSet<ClassExpression<T>>,
    pub superclass_of: HashSet<ClassExpression<T>>,
    pub disjoint_with: HashSet<ClassExpression<T>>,
    pub disjoint_union_of: Vec<Vec<ClassExpression<T>>>,
    pub individuals: HashSet<Individual<T>>,

    pub depends_on: HashSet<OntologySymbol<T>>,
}

impl<T: ForIRI + Default + Serialize> Serialize for ClassDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ClassDetails", 8)?;
        state.serialize_field("annotations", &wrap_annotations(&self.annotations))?;
        state.serialize_field("subclass_of", &self.subclass_of.iter().map(|c| ClassExpressionView::from(c)).collect::<Vec<_>>())?;
        state.serialize_field("equivalent_to", &self.equivalent_to.iter().map(|c| ClassExpressionView::from(c)).collect::<Vec<_>>())?;
        state.serialize_field("superclass_of", &self.superclass_of.iter().map(|c| ClassExpressionView::from(c)).collect::<Vec<_>>())?;
        state.serialize_field("disjoint_with", &self.disjoint_with.iter().map(|c| ClassExpressionView::from(c)).collect::<Vec<_>>())?;
        state.serialize_field("disjoint_union_of", &self.disjoint_union_of.iter().map(|cs| cs.iter().map(|c| ClassExpressionView::from(c)).collect::<Vec<_>>()).collect::<Vec<_>>())?;
        state.serialize_field("individuals", &self.individuals.iter().map(|i| IndividualView::from(i)).collect::<Vec<_>>())?;
        state.serialize_field("depends_on", &self.depends_on.iter().map(|c| OntologySymbolView::from(c)).collect::<Vec<_>>())?;
        state.end()
    }
}


pub fn _derive_classes_from_class_expression<T: ForIRI>(ce: &ClassExpression<T>) -> HashSet<T> {
    match ce {
        ClassExpression::Class(Class(iri)) => HashSet::from([iri.underlying()]),
        ClassExpression::ObjectIntersectionOf(v) => v
            .into_iter()
            .flat_map(|ce| _derive_classes_from_class_expression(ce))
            .collect(),
        ClassExpression::ObjectUnionOf(v) => v
            .into_iter()
            .flat_map(|ce| _derive_classes_from_class_expression(ce))
            .collect(),
        ClassExpression::ObjectComplementOf(ce) => {
            _derive_classes_from_class_expression(ce)
        }
        ClassExpression::ObjectSomeValuesFrom {ope:_, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        ClassExpression::ObjectAllValuesFrom { ope:_, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        ClassExpression::ObjectMinCardinality { n:_, ope:_, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        ClassExpression::ObjectMaxCardinality { n:_, ope:_, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        ClassExpression::ObjectExactCardinality { n:_, ope:_, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        _ => HashSet::new(),
    }
}