use crate::onto::serialize::{
    AnnotationValueView, ClassExpressionView, ObjectPropertyExpressionView,
    SubObjectPropertyExpressionView,
};
use horned_owl::model::{
    Annotation, AnnotationSubject, Class, ClassExpression, ForIRI, ObjectProperty,
    ObjectPropertyExpression, SubClassOf, SubObjectPropertyExpression, IRI,
};
use horned_owl::ontology::set::SetOntology;
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use strix_roost::ontology::visitor::AxiomVisitor;

#[derive(Default, Serialize)]
pub struct OntologyView {
    pub is_asserted_subclass_expression_of: HashMap<String, HashSet<String>>,
    is_asserted_superclass_of: HashMap<String, HashSet<String>>,
    equivalent_classes: Vec<Vec<ClassExpressionView>>,
    class_roots: HashSet<String>,

    is_asserted_sub_object_property_expression_of: HashSet<(
        SubObjectPropertyExpressionView,
        ObjectPropertyExpressionView,
    )>,
    is_asserted_super_object_property_of: HashMap<String, HashSet<String>>,
    equivalent_object_properties: Vec<Vec<ObjectPropertyExpressionView>>,
    object_property_roots: HashSet<String>,

    labels: HashMap<String, AnnotationValueView>,
}

fn add_to_map<T: Hash + PartialEq + Eq, S: Eq + Hash>(
    map: &mut HashMap<T, HashSet<S>>,
    key: T,
    value: S,
) {
    map.entry(key).or_insert_with(HashSet::new).insert(value);
}

pub(crate) fn find_roots<T: Hash + Eq + PartialEq + Clone>(
    supers: &HashMap<T, HashSet<T>>,
) -> HashSet<T> {
    let sups = supers.iter().map(|(a, _b)| a).collect::<HashSet<_>>();
    let subs = supers.iter().flat_map(|(_a, b)| b).collect::<HashSet<_>>();
    sups.difference(&subs).map(|c| (**c).clone()).collect()
}

fn merge_maps<K: Hash + Eq, V, C: Default + Extend<V> + IntoIterator<Item = V>>(
    m1: &mut HashMap<K, C>,
    m2: HashMap<K, C>,
) {
    for (k2, vs2) in m2.into_iter() {
        m1.entry(k2).or_insert(C::default()).extend(vs2);
    }
}

impl OntologyView {
    pub(crate) fn new<T: ForIRI>(so: &SetOntology<T>) -> Self {
        let mut hier = OntologyView {
            is_asserted_subclass_expression_of: Default::default(),
            is_asserted_superclass_of: Default::default(),
            equivalent_classes: vec![],
            class_roots: Default::default(),
            is_asserted_sub_object_property_expression_of: Default::default(),
            is_asserted_super_object_property_of: Default::default(),
            equivalent_object_properties: vec![],
            object_property_roots: Default::default(),
            labels: Default::default(),
        };
        for ov in OntologyViewBuilder::visit_components(so.i().iter(), None) {
            merge_maps(
                &mut hier.is_asserted_subclass_expression_of,
                ov.is_asserted_subclass_expression_of,
            );
            merge_maps(
                &mut hier.is_asserted_superclass_of,
                ov.is_asserted_superclass_of,
            );
            hier.equivalent_classes.extend(ov.equivalent_classes);
            hier.is_asserted_sub_object_property_expression_of
                .extend(ov.is_asserted_sub_object_property_expression_of);
            merge_maps(
                &mut hier.is_asserted_super_object_property_of,
                ov.is_asserted_super_object_property_of,
            );
            hier.equivalent_object_properties
                .extend(ov.equivalent_object_properties);
            hier.labels.extend(ov.labels.into_iter());
        }
        hier.class_roots = find_roots(&hier.is_asserted_superclass_of);
        hier.object_property_roots = find_roots(&hier.is_asserted_super_object_property_of);
        hier
    }

    fn derive_superclasses_of_class_expression<T: ForIRI>(ce: &ClassExpression<T>) -> Vec<String> {
        match ce {
            ClassExpression::Class(c) => Vec::from([c.underlying().to_string()]),
            ClassExpression::ObjectIntersectionOf(cs) => {
                let c = cs
                    .iter()
                    .flat_map(|c| Self::derive_superclasses_of_class_expression(c))
                    .collect();
                c
            }
            _ => Vec::new(),
        }
    }

    fn derive_subclasses_of_class_expression<T: ForIRI>(ce: &ClassExpression<T>) -> Vec<String> {
        match ce {
            ClassExpression::Class(c) => Vec::from([c.0.underlying().to_string()]),
            ClassExpression::ObjectUnionOf(cs) => cs
                .iter()
                .flat_map(|c| Self::derive_subclasses_of_class_expression(c))
                .collect(),
            _ => Vec::new(),
        }
    }

    fn derive_subroles_of_object_property_expression<T: ForIRI>(
        sope: &SubObjectPropertyExpression<T>,
    ) -> Vec<String> {
        match sope {
            SubObjectPropertyExpression::ObjectPropertyExpression(ope) => match ope {
                ObjectPropertyExpression::ObjectProperty(op) => {
                    vec![op.0.to_string()]
                }
                ObjectPropertyExpression::InverseObjectProperty(_) => Vec::new(),
            },
            SubObjectPropertyExpression::ObjectPropertyChain(_) => Vec::new(),
        }
    }

    fn derive_superroles_of_object_property_expression<T: ForIRI>(
        ope: &ObjectPropertyExpression<T>,
    ) -> Vec<String> {
        match ope {
            ObjectPropertyExpression::ObjectProperty(op) => {
                vec![op.0.to_string()]
            }
            ObjectPropertyExpression::InverseObjectProperty(_) => Vec::new(),
        }
    }
}

fn derive_superclasses_of_class_expression<'a, T: ForIRI>(
    ce: &'a ClassExpression<T>,
) -> HashSet<&'a IRI<T>> {
    let mut hs = HashSet::new();
    match ce {
        ClassExpression::Class(Class(ref iri)) => {
            hs.insert(iri);
        }
        ClassExpression::ObjectUnionOf(cs) => {
            cs.iter()
                .flat_map(|c| derive_subclasses_of_class_expression(c))
                .for_each(|x| {
                    hs.insert(x);
                });
        }
        _ => {}
    }
    hs
}

fn derive_subclasses_of_class_expression<'a, T: ForIRI>(
    ce: &'a ClassExpression<T>,
) -> HashSet<&'a IRI<T>> {
    let mut hs = HashSet::new();
    match ce {
        ClassExpression::Class(Class(ref iri)) => {
            hs.insert(iri);
        }
        ClassExpression::ObjectIntersectionOf(cs) => {
            cs.iter()
                .flat_map(|c| derive_subclasses_of_class_expression(c))
                .for_each(|x| {
                    hs.insert(x);
                });
        }
        _ => {}
    }
    hs
}

fn derive_subroles_of_object_property_expression<'a, T: ForIRI>(
    sope: &'a SubObjectPropertyExpression<T>,
) -> HashSet<&'a IRI<T>> {
    let mut hs = HashSet::new();
    match sope {
        SubObjectPropertyExpression::ObjectPropertyExpression(ope) => match ope {
            ObjectPropertyExpression::ObjectProperty(ObjectProperty(ref iri)) => {
                hs.insert(iri);
            }
            _ => {}
        },
        _ => {}
    }
    hs
}

fn derive_superroles_of_object_property_expression<'a, T: ForIRI>(
    sope: &'a ObjectPropertyExpression<T>,
) -> HashSet<&'a IRI<T>> {
    let mut hs = HashSet::new();
    match sope {
        ObjectPropertyExpression::ObjectProperty(ObjectProperty(ref iri)) => {
            hs.insert(iri);
        }
        _ => {}
    }
    hs
}
struct OntologyViewBuilder {}

impl<'a, T: ForIRI> AxiomVisitor<'a, T, OntologyView> for OntologyViewBuilder {
    fn visit_subclass_of(sco: &SubClassOf<T>, _target: Option<&T>) -> Option<OntologyView> {
        let mut res = OntologyView::default();

        for sub in derive_subclasses_of_class_expression(&sco.sub) {
            for sup in derive_superclasses_of_class_expression(&sco.sup) {
                add_to_map(
                    &mut res.is_asserted_subclass_expression_of,
                    sub.underlying().to_string(),
                    sup.underlying().to_string(),
                );
                add_to_map(
                    &mut res.is_asserted_superclass_of,
                    sup.underlying().to_string(),
                    sub.underlying().to_string(),
                );
            }
        }
        Some(res)
    }

    fn visit_equivalent_classes(
        cs: &Vec<ClassExpression<T>>,
        _target: Option<&T>,
    ) -> Option<OntologyView> {
        let mut res = OntologyView::default();
        res.equivalent_classes
            .push(cs.iter().map(|c| c.into()).collect());
        Some(res)
    }

    fn visit_sub_object_property_of(
        sub: &SubObjectPropertyExpression<T>,
        sup: &ObjectPropertyExpression<T>,
        _target: Option<&T>,
    ) -> Option<OntologyView> {
        let mut res = OntologyView::default();
        res.is_asserted_sub_object_property_expression_of
            .insert((sub.into(), sup.into()));
        for sub in derive_subroles_of_object_property_expression(sub) {
            for sup in derive_superroles_of_object_property_expression(sup) {
                add_to_map(
                    &mut res.is_asserted_super_object_property_of,
                    sup.underlying().to_string(),
                    sub.underlying().to_string(),
                );
            }
        }
        Some(res)
    }

    fn visit_equivalent_object_properties(
        es: &Vec<ObjectPropertyExpression<T>>,
        _target: Option<&T>,
    ) -> Option<OntologyView> {
        let mut res = OntologyView::default();
        res.equivalent_object_properties
            .push(es.iter().map(|c| c.into()).collect());
        Some(res)
    }

    fn visit_annotation_assertion(
        subject: &AnnotationSubject<T>,
        ann: &Annotation<T>,
        _target: Option<&T>,
    ) -> Option<OntologyView> {
        if ann.ap.0.underlying().to_string() == "http://www.w3.org/2000/01/rdf-schema#label" {
            match subject {
                AnnotationSubject::IRI(subject_iri) => {
                    let mut res = OntologyView::default();
                    res.labels
                        .insert(subject_iri.underlying().to_string(), (&ann.av).into());
                    Some(res)
                }
                AnnotationSubject::AnonymousIndividual(_) => None,
            }
        } else {
            None
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
        _ => HashSet::new(),
    }
}

#[derive(Default, Serialize)]
pub struct RoleHierarchy {
    is_asserted_subrole_expression_of: HashSet<(ClassExpressionView, ClassExpressionView)>,
    is_asserted_superrole_of: HashMap<String, HashSet<String>>,
    equivalent_roles: Vec<Vec<ClassExpressionView>>,
    roots: HashSet<String>,
    labels: HashMap<String, AnnotationValueView>,
}
