use std::collections::{HashMap, HashSet};
use std::mem::needs_drop;
use std::marker::Send;
use std::ops::Deref;
use std::sync::Arc;
use horned_owl::model::{AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationProperty, AnnotationSubject, AnnotationValue, AnonymousIndividual, ArcStr, AsymmetricObjectProperty, Build, Class, ClassAssertion, ClassExpression, Component, DataProperty, DataPropertyDomain, DataPropertyRange, DataRange, Datatype, DatatypeDefinition, DeclareAnnotationProperty, DeclareClass, DeclareDataProperty, DeclareDatatype, DeclareNamedIndividual, DeclareObjectProperty, DifferentIndividuals, DisjointClasses, DisjointDataProperties, DisjointObjectProperties, DisjointUnion, EquivalentClasses, EquivalentDataProperties, EquivalentObjectProperties, ForIRI, FunctionalDataProperty, FunctionalObjectProperty, Import, Individual, InverseFunctionalObjectProperty, InverseObjectProperties, IrreflexiveObjectProperty, Literal, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, ObjectPropertyRange, OntologyAnnotation, OntologyID, ReflexiveObjectProperty, SameIndividual, SubClassOf, SubDataPropertyOf, SubObjectPropertyExpression, SubObjectPropertyOf, SymmetricObjectProperty, TransitiveObjectProperty, IRI};
use horned_owl::ontology::set::SetOntology;
use strix_roost::dependency::base::{DependencyMap, OntologySymbol};
use strix_roost::dependency::growth::GrowthDependency;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use strix_roost::ontology::io::load_set_ontology;
use crate::onto::owl::class::{ClassDetails, _derive_classes_from_class_expression};
use crate::onto::owl::datatype::DatatypeDetails;
use crate::onto::owl::individual::IndividualDetails;
use crate::onto::owl::property::{DataPropertyDetails, ObjectPropertyDetails};
use crate::onto::rdf::RDFS_LABEL;
use crate::onto::serialize::AnnotationValueView;

#[derive(Default)]
pub struct OntologyDetails<T: ForIRI> {
    pub annotations: HashMap<T, Vec<AnnotationValueView>>,
    pub imports: Vec<T>,
    pub ontology_iri: Option<T>,
}
