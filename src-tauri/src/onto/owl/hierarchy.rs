use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::sync::Arc;
use horned_owl::model::{Annotation, AnnotationSubject, Class, ClassExpression, ForIRI, SubClassOf};
use horned_owl::ontology::set::SetOntology;
use petgraph::graph::DiGraph;
use serde::Serialize;
use crate::onto::owl::class::ClassDetails;
use crate::onto::owl::visitor::AxiomVisitor;
use crate::onto::serialize::{AnnotationValueView, ClassExpressionView};

#[derive(Default, Serialize)]
pub struct ClassHierarchy {
    is_asserted_subclass_expression_of: HashSet<(ClassExpressionView, ClassExpressionView)>,
    is_asserted_superclass_of: HashMap<String, HashSet<String>>,
    equivalent_classes: Vec<Vec<ClassExpressionView>>,
    roots: HashSet<String>,
    labels: HashMap<String, AnnotationValueView>
}

fn add_to_map<T: Hash + PartialEq + Eq, S: Eq + Hash>(map: &mut HashMap<T, HashSet<S>>, key: T, value: S)  {
    map.entry(key).or_insert_with(HashSet::new).insert(value);
}

impl ClassHierarchy {
    fn find_roots(&mut self) {
        let sups = self.is_asserted_superclass_of.iter().map(|(a,b)| a).collect::<HashSet<_>>();
        let subs = self.is_asserted_superclass_of.iter().flat_map(|(a,b)| b).collect::<HashSet<_>>();
        self.roots = sups.difference(&subs).map(|c| (*c).clone()).collect()
    }

    pub(crate) fn new<T: ForIRI>(so: &SetOntology<T>) -> Self {
        let mut hier = ClassHierarchy::default();
        hier.visit_components(so.i().iter(), &T::from("".to_string()));
        hier.find_roots();
        hier
    }

    fn derive_superclasses_of_class_expression<T: ForIRI>(ce: &ClassExpression<T>) -> Vec<String> {
        match ce {
            ClassExpression::Class(c) => Vec::from([c.underlying().to_string()]),
            ClassExpression::ObjectIntersectionOf(cs) => {
                let c = cs.iter().flat_map(|c| Self::derive_superclasses_of_class_expression(c)).collect();
                c
            },
            _ => Vec::new()
        }
    }

    fn derive_subclasses_of_class_expression<T: ForIRI>(ce: &ClassExpression<T>) -> Vec<String> {
        match ce {
            ClassExpression::Class(c) => Vec::from([c.0.underlying().to_string()]),
            ClassExpression::ObjectUnionOf(cs) => cs.iter().flat_map(|c| Self::derive_subclasses_of_class_expression(c)).collect(),
            _ => Vec::new()
        }
    }
}

impl<T: ForIRI>  AxiomVisitor<T> for ClassHierarchy {
    fn visit_subclass_of(&mut self, sco: &SubClassOf<T>, target: &T) {
        self.is_asserted_subclass_expression_of.insert(((&sco.sub).into(), (&sco.sup).into()));
        for sub in Self::derive_subclasses_of_class_expression(&sco.sub) {
            for sup in Self::derive_superclasses_of_class_expression(&sco.sup){
                add_to_map(&mut self.is_asserted_superclass_of, sup, sub.clone());
            }
        }

    }

    fn visit_equivalent_classes(&mut self, cs: &Vec<ClassExpression<T>>, target: &T) {
        self.equivalent_classes.push(cs.iter().map(|c| c.into()).collect())
    }

    fn visit_annotation_assertion(&mut self, subject: &AnnotationSubject<T>, ann: &Annotation<T>, target: &T) {
        if ann.ap.0.underlying().to_string() == "http://www.w3.org/2000/01/rdf-schema#label" {
            match subject {
                AnnotationSubject::IRI(subject_iri) => {
                    self.labels.insert(subject_iri.underlying().to_string(), (&ann.av).into());
                }
                AnnotationSubject::AnonymousIndividual(_) => {}
            }
        }
    }

}

fn _derive_classes_from_class_expression<T: ForIRI>(ce: &ClassExpression<T>) -> HashSet<T> {
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
