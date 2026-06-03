use crate::onto::serialize::{
    AnnotationValueView, ClassExpressionView, IndividualView, OntologySymbolView,
};
use horned_owl::model::{
    Annotation, AnnotationSubject, Class, ClassExpression, ForIRI, Individual, SubClassOf,
};
use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::collections::{HashMap, HashSet};
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
}

impl<T: ForIRI> ClassDetails<T> {
    pub(crate) fn merge(self, other: ClassDetails<T>) -> ClassDetails<T> {
        ClassDetails {
            annotations: self
                .annotations
                .into_iter()
                .chain(other.annotations.into_iter())
                .collect(),
            subclass_of: self
                .subclass_of
                .into_iter()
                .chain(other.subclass_of.into_iter())
                .collect(),
            equivalent_to: self
                .equivalent_to
                .into_iter()
                .chain(other.equivalent_to.into_iter())
                .collect(),
            superclass_of: self
                .superclass_of
                .into_iter()
                .chain(other.superclass_of.into_iter())
                .collect(),
            disjoint_with: self
                .disjoint_with
                .into_iter()
                .chain(other.disjoint_with.into_iter())
                .collect(),
            disjoint_union_of: self
                .disjoint_union_of
                .into_iter()
                .chain(other.disjoint_union_of.into_iter())
                .collect(),
            individuals: self
                .individuals
                .into_iter()
                .chain(other.individuals.into_iter())
                .collect(),
        }
    }
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
        state.end()
    }
}

impl<'a, T: ForIRI> AxiomVisitor<'a, T, ClassDetails<T>> for ClassDetails<T> {
    fn visit_subclass_of(sco: &'a SubClassOf<T>, target: Option<&T>) -> Option<ClassDetails<T>> {
        let mut res = ClassDetails::default();
        let mut changed = false;
        if let ClassExpression::Class(ref iri) = sco.sub {
            if Some(&iri.underlying()) == target {
                res.subclass_of.push((&sco.sup).into());
                changed = true;
            }
        }
        if let ClassExpression::Class(ref iri) = sco.sup {
            if Some(&iri.underlying()) == target {
                res.superclass_of.push((&sco.sub).into());
                changed = true;
            }
        }
        if changed {
            Some(res)
        } else {
            None
        }
    }

    fn visit_equivalent_classes(
        cs: &'a Vec<ClassExpression<T>>,
        target: Option<&'a T>,
    ) -> Option<ClassDetails<T>> {
        let (is_contained, rest) = Self::match_class_list(target, cs);

        if is_contained {
            let mut res = ClassDetails::default();
            res.equivalent_to
                .extend(rest.into_iter().map(|c| ClassExpressionView::from(c)));
            Some(res)
        } else {
            None
        }
    }

    fn visit_disjoint_classes(
        cs: &'a Vec<ClassExpression<T>>,
        target: Option<&'a T>,
    ) -> Option<ClassDetails<T>> {
        let (is_contained, rest) = Self::match_class_list(target, cs);
        if is_contained {
            let mut res = ClassDetails::default();
            res.disjoint_with
                .extend(rest.into_iter().map(|c| ClassExpressionView::from(c)));
            Some(res)
        } else {
            None
        }
    }

    fn visit_disjoint_union(
        c: &'a Class<T>,
        cs: &'a Vec<ClassExpression<T>>,
        target: Option<&'a T>,
    ) -> Option<ClassDetails<T>> {
        let Class(iri) = c;
        if Some(&iri.underlying()) == target {
            let mut res = ClassDetails::default();
            res.disjoint_union_of.push(
                cs.iter()
                    .map(|c| ClassExpressionView::from(c))
                    .collect::<Vec<_>>(),
            );
            Some(res)
        } else {
            None
        }
    }

    fn visit_class_assertion(
        ce: &'a ClassExpression<T>,
        i: &'a Individual<T>,
        target: Option<&'a T>,
    ) -> Option<ClassDetails<T>> {
        if let ClassExpression::Class(iri) = ce {
            match Some(&iri.underlying()) == target {
                true => {
                    let mut res = ClassDetails::default();
                    res.individuals.push(i.into());
                    Some(res)
                }
                false => None,
            }
        } else {
            None
        }
    }

    fn visit_annotation_assertion(
        subject: &'a AnnotationSubject<T>,
        ann: &'a Annotation<T>,
        target: Option<&'a T>,
    ) -> Option<ClassDetails<T>> {
        match subject {
            AnnotationSubject::IRI(iri) => {
                if target == Some(&iri.underlying()) {
                    let mut res = ClassDetails::default();
                    let ann_iri = ann.ap.underlying();
                    let annos = res.annotations.entry(ann_iri).or_insert_with(Vec::new);
                    annos.push(AnnotationValueView::from(&ann.av));
                    Some(res)
                } else {
                    None
                }
            }
            AnnotationSubject::AnonymousIndividual(_) => None,
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
        ClassExpression::ObjectComplementOf(ce) => _derive_classes_from_class_expression(ce),
        ClassExpression::ObjectSomeValuesFrom { ope: _, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        ClassExpression::ObjectAllValuesFrom { ope: _, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        ClassExpression::ObjectMinCardinality { n: _, ope: _, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        ClassExpression::ObjectMaxCardinality { n: _, ope: _, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        ClassExpression::ObjectExactCardinality { n: _, ope: _, bce } => {
            _derive_classes_from_class_expression(bce)
        }
        _ => Vec::new(),
    }
}

impl<T: ForIRI> Default for ClassDetails<T> {
    fn default() -> Self {
        ClassDetails {
            annotations: Default::default(),
            subclass_of: vec![],
            equivalent_to: vec![],
            superclass_of: vec![],
            disjoint_with: vec![],
            disjoint_union_of: vec![],
            individuals: vec![],
        }
    }
}
