use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use horned_owl::model::{Annotation, AnnotationSubject, Class, ClassExpression, ForIRI, ObjectPropertyExpression, SubClassOf, SubObjectPropertyExpression};
use horned_owl::ontology::set::SetOntology;
use serde::Serialize;
use strix_roost::ontology::visitor::AxiomVisitor;
use crate::onto::serialize::{AnnotationValueView, ClassExpressionView, ObjectPropertyExpressionView, SubObjectPropertyExpressionView};

#[derive(Default, Serialize)]
pub struct OntologyView {
    is_asserted_subclass_expression_of: HashSet<(ClassExpressionView, ClassExpressionView)>,
    is_asserted_superclass_of: HashMap<String, HashSet<String>>,
    equivalent_classes: Vec<Vec<ClassExpressionView>>,
    class_roots: HashSet<String>,

    is_asserted_sub_object_property_expression_of: HashSet<(SubObjectPropertyExpressionView, ObjectPropertyExpressionView)>,
    is_asserted_super_object_property_of: HashMap<String, HashSet<String>>,
    equivalent_object_properties: Vec<Vec<ObjectPropertyExpressionView>>,
    object_property_roots: HashSet<String>,

    labels: HashMap<String, AnnotationValueView>
}

fn add_to_map<T: Hash + PartialEq + Eq, S: Eq + Hash>(map: &mut HashMap<T, HashSet<S>>, key: T, value: S)  {
    map.entry(key).or_insert_with(HashSet::new).insert(value);
}

impl OntologyView {
    fn find_roots<T: Hash + Eq + PartialEq + Clone>(supers: &HashMap<T, HashSet<T>>) -> HashSet<T> {
        let sups = supers.iter().map(|(a,_b)| a).collect::<HashSet<_>>();
        let subs = supers.iter().flat_map(|(_a,b)| b).collect::<HashSet<_>>();
        sups.difference(&subs).map(|c| (**c).clone()).collect()
    }

    pub(crate) fn new<T: ForIRI>(so: &SetOntology<T>) -> Self {
        let mut hier = OntologyView::default();
        hier.visit_components(so.i().iter(), &T::from("".to_string()));
        hier.class_roots = Self::find_roots(&hier.is_asserted_superclass_of);
        hier.object_property_roots = Self::find_roots(&hier.is_asserted_super_object_property_of);
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

    fn derive_subroles_of_object_property_expression<T: ForIRI>(sope: &SubObjectPropertyExpression<T>) -> Vec<String> {
        match sope {
            SubObjectPropertyExpression::ObjectPropertyExpression(ope) =>
                match ope {
                    ObjectPropertyExpression::ObjectProperty(op) => { vec![op.0.to_string()] }
                    ObjectPropertyExpression::InverseObjectProperty(_) => { Vec::new() }
                }
            SubObjectPropertyExpression::ObjectPropertyChain(_) => { Vec::new() },
        }
    }

    fn derive_superroles_of_object_property_expression<T: ForIRI>(ope: &ObjectPropertyExpression<T>) -> Vec<String> {
        match ope {
            ObjectPropertyExpression::ObjectProperty(op) => { vec![op.0.to_string()] }
            ObjectPropertyExpression::InverseObjectProperty(_) => { Vec::new() }
        }

    }
}

impl<T: ForIRI>  AxiomVisitor<T> for OntologyView {
    fn visit_subclass_of(&mut self, sco: &SubClassOf<T>, _target: &T) {
        self.is_asserted_subclass_expression_of.insert(((&sco.sub).into(), (&sco.sup).into()));
        for sub in Self::derive_subclasses_of_class_expression(&sco.sub) {
            for sup in Self::derive_superclasses_of_class_expression(&sco.sup){
                add_to_map(&mut self.is_asserted_superclass_of, sup, sub.clone());
            }
        }

    }

    fn visit_equivalent_classes(&mut self, cs: &Vec<ClassExpression<T>>, _target: &T) {
        self.equivalent_classes.push(cs.iter().map(|c| c.into()).collect())
    }

    fn visit_annotation_assertion(&mut self, subject: &AnnotationSubject<T>, ann: &Annotation<T>, _target: &T) {
        if ann.ap.0.underlying().to_string() == "http://www.w3.org/2000/01/rdf-schema#label" {
            match subject {
                AnnotationSubject::IRI(subject_iri) => {
                    self.labels.insert(subject_iri.underlying().to_string(), (&ann.av).into());
                }
                AnnotationSubject::AnonymousIndividual(_) => {}
            }
        }
    }

    fn visit_sub_object_property_of(&mut self, sub: &SubObjectPropertyExpression<T>, sup: &ObjectPropertyExpression<T>, _target: &T) {
        self.is_asserted_sub_object_property_expression_of.insert((sub.into(), sup.into()));
        for sub in Self::derive_subroles_of_object_property_expression(sub) {
            for sup in Self::derive_superroles_of_object_property_expression(sup){
                add_to_map(&mut self.is_asserted_super_object_property_of, sup, sub.clone());
            }
        }
    }

    fn visit_equivalent_object_properties(&mut self, es: &Vec<ObjectPropertyExpression<T>>, _target: &T) {
        self.equivalent_object_properties.push(es.iter().map(|c| c.into()).collect())
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

#[derive(Default, Serialize)]
pub struct RoleHierarchy {
    is_asserted_subrole_expression_of: HashSet<(ClassExpressionView, ClassExpressionView)>,
    is_asserted_superrole_of: HashMap<String, HashSet<String>>,
    equivalent_roles: Vec<Vec<ClassExpressionView>>,
    roots: HashSet<String>,
    labels: HashMap<String, AnnotationValueView>
}