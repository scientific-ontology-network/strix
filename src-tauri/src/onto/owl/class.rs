use std::collections::{HashMap, HashSet};
use horned_owl::model::{Annotation, AnnotationSubject, Class, ClassExpression, ForIRI, Individual, SubClassOf};
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::serialize::{AnnotationValueView, ClassExpressionView, IndividualView, OntologySymbolView};
use strix_roost::ontology::visitor::AxiomVisitor;

pub struct ClassDetails<T>
where
    T: ForIRI,
{
    pub annotations: HashMap<T, Vec<AnnotationValueView>>,

    pub subclass_of: Vec<ClassExpressionView>,
    pub equivalent_to: Vec<ClassExpressionView>,
    pub superclass_of: Vec<ClassExpressionView>,
    pub disjoint_with: Vec<ClassExpressionView>,
    pub disjoint_union_of: Vec<Vec<ClassExpressionView>>,
    pub individuals: Vec<IndividualView>,

    pub depends_on: HashSet<OntologySymbolView>,
}

impl<T: ForIRI + Serialize> Serialize for ClassDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("ClassDetails", 8)?;
        state.serialize_field("annotations", &self.annotations)?;
        state.serialize_field("subclass_of", &self.subclass_of)?;
        state.serialize_field("equivalent_to", &self.equivalent_to)?;
        state.serialize_field("superclass_of", &self.superclass_of)?;
        state.serialize_field("disjoint_with", &self.disjoint_with)?;
        state.serialize_field("disjoint_union_of", &self.disjoint_union_of)?;
        state.serialize_field("individuals", &self.individuals)?;
        state.serialize_field("depends_on", &self.depends_on)?;
        state.end()
    }
}

impl<T: ForIRI> AxiomVisitor<T> for ClassDetails<T> {
    fn visit_subclass_of(&mut self, sco: &SubClassOf<T>, target: &T) {
        if let ClassExpression::Class(ref iri) = sco.sub {
            if iri.underlying() == *target {
                self.subclass_of.push((&sco.sup).into())
            }
        }
        if let ClassExpression::Class(ref iri) = sco.sup {
            if iri.underlying() == *target {
                self.superclass_of.push((&sco.sub).into())
            }
        }
    }

    fn visit_equivalent_classes(&mut self, cs: &Vec<ClassExpression<T>>, target: &T) {
        let (is_contained, rest) = Self::match_class_list(target, cs);
        if is_contained {
            self.equivalent_to.extend(rest.into_iter().map(|c| ClassExpressionView::from(c)))
        }
    }

    fn visit_disjoint_classes(&mut self, cs: &Vec<ClassExpression<T>>, target: &T) {
        let (is_contained, rest) = Self::match_class_list(target, cs);
        if is_contained {
            self.disjoint_with.extend(rest.into_iter().map(|c| ClassExpressionView::from(c)))
        }
    }

    fn visit_disjoint_union(&mut self, c: &Class<T>, cs: &Vec<ClassExpression<T>>, target: &T) {
        let Class(iri) = c;
        if iri.underlying() == *target {
            self.disjoint_union_of.push(cs.iter().map(|c| ClassExpressionView::from(c)).collect::<Vec<_>>())
        }
        
    }

    fn visit_class_assertion(&mut self, ce: &ClassExpression<T>, i: &Individual<T>, target: &T) {
        if let ClassExpression::Class(iri) = ce{
            if iri.underlying() == *target {
                self.individuals.push(i.into())
            }
        }
    }

    fn visit_annotation_assertion(&mut self, subject: &AnnotationSubject<T>, ann: &Annotation<T>, target: &T) {
        match subject {
            AnnotationSubject::IRI(iri) => {
                if iri.underlying() == *target {
                    let ann_iri = ann.ap.underlying();
                    let annos = self.annotations.entry(ann_iri).or_insert_with(Vec::new);
                    annos.push((&ann.av).into())
                }
            }
            AnnotationSubject::AnonymousIndividual(_) => {}
        }
    }
}


pub fn _derive_classes_from_class_expression<T: ForIRI>(ce: &ClassExpression<T>) -> Vec<T> {
    match ce {
        ClassExpression::Class(Class(iri)) => Vec::from([iri.underlying()]),
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
        _ => Vec::new(),
    }
}

impl<T: ForIRI> Default for ClassDetails<T>{
    fn default() -> Self {
        ClassDetails{
            annotations: Default::default(),
            subclass_of: vec![],
            equivalent_to: vec![],
            superclass_of: vec![],
            disjoint_with: vec![],
            disjoint_union_of: vec![],
            individuals: vec![],
            depends_on: HashSet::new(),
        }
    }
}

