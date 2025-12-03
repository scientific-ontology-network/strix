use std::collections::HashMap;
use std::hash::Hash;
use horned_owl::model::{AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationProperty, AnnotationSubject, AnnotationValue, ArcStr, AsymmetricObjectProperty, Class, ClassAssertion, ClassExpression, Component, DataProperty, DataPropertyDomain, DataPropertyRange, DataRange, Datatype, DatatypeDefinition, DeclareAnnotationProperty, DeclareClass, DeclareDataProperty, DeclareDatatype, DeclareNamedIndividual, DeclareObjectProperty, DifferentIndividuals, DisjointClasses, DisjointDataProperties, DisjointObjectProperties, DisjointUnion, EquivalentClasses, EquivalentDataProperties, EquivalentObjectProperties, ForIRI, FunctionalDataProperty, FunctionalObjectProperty, Import, Individual, InverseFunctionalObjectProperty, InverseObjectProperties, IrreflexiveObjectProperty, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, ObjectPropertyRange, Ontology, OntologyAnnotation, OntologyID, ReflexiveObjectProperty, SameIndividual, SubClassOf, SubDataPropertyOf, SubObjectPropertyExpression, SubObjectPropertyOf, SymmetricObjectProperty, TransitiveObjectProperty, IRI};
use horned_owl::ontology::set::SetOntology;
use strix_roost::dependency::base::{DependencyMap, OntologySymbol};
use strix_roost::ontology::io::load_set_ontology;
use crate::onto::owl::annotation::AnnotationPropertyDetails;
use crate::onto::owl::class::ClassDetails;
use crate::onto::owl::datatype::DatatypeDetails;
use crate::onto::owl::individual::IndividualDetails;
use crate::onto::owl::ontology;
use crate::onto::owl::ontology::OntologyDetails;
use crate::onto::owl::property::{DataPropertyDetails, ObjectPropertyDetails};

pub(crate) trait AxiomVisitor<T: ForIRI> {

    fn match_class_list<'a>(target: &T, list: &'a Vec<ClassExpression<T>>) -> (bool, Vec<&'a ClassExpression<T>>) {
        let mut is_contained = false;
        let mut rest = Vec::new();
        for c in list.into_iter() {
            if let ClassExpression::Class(ref iri) = c {
                if iri.underlying() == *target {
                    is_contained = true;
                } else {
                    rest.push(c)
                }
            } else {
                rest.push(c)
            }
        }
        (is_contained, rest)
    }
    fn visit_ontology_id(&mut self, oid: &OntologyID<T>, target: &T) {}

    fn visit_ontology_annotation(&mut self, ann: &Annotation<T>, target: &T) {}

    fn visit_import(&mut self, iri: &IRI<T>, target: &T) {}

    fn visit_declare_class(&mut self, cls: &Class<T>, target: &T) {}

    fn visit_declare_object_property(&mut self, op: &ObjectProperty<T>, target: &T) {}

    fn visit_declare_annotation_property(&mut self, ap: &AnnotationProperty<T>, target: &T) {}

    fn visit_declare_data_property(&mut self, dp: &DataProperty<T>, target: &T) {}


    fn visit_declare_named_individual(&mut self, ni: &NamedIndividual<T>, target: &T) {}

    fn visit_declare_datatype(&mut self, dt: &Datatype<T>, target: &T) {}

    fn visit_subclass_of(&mut self, sco: &SubClassOf<T>, target: &T) {}

    fn visit_equivalent_classes(&mut self, cs: &Vec<ClassExpression<T>>, target: &T) {}

    fn visit_disjoint_classes(&mut self, cs: &Vec<ClassExpression<T>>, target: &T) {}

    fn visit_disjoint_union(&mut self, c: &Class<T>, cs: &Vec<ClassExpression<T>>, target: &T) {}

    fn visit_sub_object_property_of(
        &mut self,
        sub: &SubObjectPropertyExpression<T>,
        sup: &ObjectPropertyExpression<T>,
        target: &T,
    ) {}

    fn visit_equivalent_object_properties(&mut self, es: &Vec<ObjectPropertyExpression<T>>, target: &T) {}

    fn visit_disjoint_object_properties(&mut self, es: &Vec<ObjectPropertyExpression<T>>, target: &T) {}

    fn visit_inverse_object_properties(
        &mut self,
        a: &ObjectProperty<T>,
        b: &ObjectProperty<T>,
        target: &T,
    ) {}

    fn visit_object_property_domain(&mut self, op: &ObjectProperty<T>, ce: &ClassExpression<T>, target: &T) {}

    fn visit_object_property_range(&mut self, op: &ObjectProperty<T>, ce: &ClassExpression<T>, target: &T) {}

    fn visit_functional_object_property(&mut self, op: &ObjectProperty<T>, target: &T) {}

    fn visit_inverse_functional_object_property(&mut self, op: &ObjectProperty<T>, target: &T) {}

    fn visit_reflexive_object_property(&mut self, op: &ObjectProperty<T>, target: &T) {}

    fn visit_irreflexive_object_property(&mut self, op: &ObjectProperty<T>, target: &T) {}

    fn visit_symmetric_object_property(&mut self, op: &ObjectProperty<T>, target: &T) {}
    fn visit_asymmetric_object_property(&mut self, op: &ObjectProperty<T>, target: &T) {}

    fn visit_transitive_object_property(&mut self, op: &ObjectProperty<T>, target: &T) {}

    fn visit_sub_data_property_of(&mut self, sub: &DataProperty<T>, sup: &DataProperty<T>, target: &T) {}

    fn visit_equivalent_data_properties(&mut self, es: &Vec<DataProperty<T>>, target: &T) {}

    fn visit_disjoint_data_properties(&mut self, es: &Vec<DataProperty<T>>, target: &T) {}

    fn visit_data_property_domain(&mut self, dp: &DataProperty<T>, ce: &ClassExpression<T>, target: &T) {}

    fn visit_data_property_range(&mut self, dp: &DataProperty<T>, dr: &DataRange<T>, target: &T) {}

    fn visit_functional_data_property(&mut self, dp: &DataProperty<T>, target: &T) {}

    fn visit_datatype_definition(&mut self, kind: &Datatype<T>, range: &DataRange<T>, target: &T) {}

    fn visit_same_individual(&mut self, es: &Vec<Individual<T>>, target: &T) {}

    fn visit_different_individuals(&mut self, es: &Vec<Individual<T>>, target: &T) {}

    fn visit_class_assertion(&mut self, ce: &ClassExpression<T>, i: &Individual<T>, target: &T) {}

    fn visit_annotation_assertion(
        &mut self,
        subject: &AnnotationSubject<T>,
        ann: &Annotation<T>,
        target: &T,
    ) {}

    fn visit_components<'a, S>(&mut self, components: S, target: &T)
    where
        T: 'a,
        S: Iterator<Item=&'a AnnotatedComponent<T>>,
    {
        for component in components {
            match &component.component {
                Component::OntologyID(oid) => { self.visit_ontology_id(oid, target); }

                Component::DocIRI(_) => {}

                Component::OntologyAnnotation(OntologyAnnotation(ref ann)) => {
                    self.visit_ontology_annotation(ann, target)
                }

                Component::Import(Import(iri)) => self.visit_import(iri, target),

                Component::DeclareClass(DeclareClass(c)) => self.visit_declare_class(c, target),

                Component::DeclareObjectProperty(DeclareObjectProperty(op)) => {
                    self.visit_declare_object_property(op, target)
                }

                Component::DeclareAnnotationProperty(DeclareAnnotationProperty(ap,
                                                     )) => self.visit_declare_annotation_property(ap, target),

                Component::DeclareDataProperty(DeclareDataProperty(dp)) => {
                    self.visit_declare_data_property(dp, target)
                }

                Component::DeclareNamedIndividual(DeclareNamedIndividual(ni)) => {
                    self.visit_declare_named_individual(ni, target)
                }

                Component::DeclareDatatype(DeclareDatatype(dt)) => {
                    self.visit_declare_datatype(dt, target)
                }

                Component::SubClassOf(sco) => self.visit_subclass_of(sco, target),

                Component::EquivalentClasses(EquivalentClasses(cs)) => {
                    self.visit_equivalent_classes(cs, target)
                }

                Component::DisjointClasses(DisjointClasses(cs)) => self.visit_disjoint_classes(cs, target),

                Component::DisjointUnion(DisjointUnion(c, cs)) => {
                    self.visit_disjoint_union(c, cs, target)
                }

                Component::SubObjectPropertyOf(SubObjectPropertyOf { sub, sup }) => {
                    self.visit_sub_object_property_of(sub, sup, target)
                }

                Component::EquivalentObjectProperties(EquivalentObjectProperties(es)) => {
                    self.visit_equivalent_object_properties(es, target)
                }

                Component::DisjointObjectProperties(DisjointObjectProperties(es)) => {
                    self.visit_disjoint_object_properties(es, target)
                }

                Component::InverseObjectProperties(InverseObjectProperties(a, b)) => {
                    self.visit_inverse_object_properties(a, b, target)
                }

                Component::ObjectPropertyDomain(ObjectPropertyDomain {
                                                    ope: ObjectPropertyExpression::ObjectProperty(op),
                                                    ce,
                                                }) => self.visit_object_property_domain(op, ce, target),

                Component::ObjectPropertyRange(ObjectPropertyRange {
                                                   ope: ObjectPropertyExpression::ObjectProperty(op),
                                                   ce,
                                               }) => self.visit_object_property_range(op, ce, target),

                Component::FunctionalObjectProperty(FunctionalObjectProperty(
                                                        ObjectPropertyExpression::ObjectProperty(op),
                                                    )) => self.visit_functional_object_property(op, target),

                Component::InverseFunctionalObjectProperty(InverseFunctionalObjectProperty(
                                                               ObjectPropertyExpression::ObjectProperty(op),
                                                           )) => self.visit_inverse_functional_object_property(op, target),

                Component::ReflexiveObjectProperty(ReflexiveObjectProperty(
                                                       ObjectPropertyExpression::ObjectProperty(op),
                                                   )) => self.visit_reflexive_object_property(op, target),

                Component::IrreflexiveObjectProperty(IrreflexiveObjectProperty(
                                                         ObjectPropertyExpression::ObjectProperty(op),
                                                     )) => self.visit_irreflexive_object_property(op, target),

                Component::SymmetricObjectProperty(SymmetricObjectProperty(
                                                       ObjectPropertyExpression::ObjectProperty(op),
                                                   )) => self.visit_symmetric_object_property(op, target),

                Component::AsymmetricObjectProperty(AsymmetricObjectProperty(
                                                        ObjectPropertyExpression::ObjectProperty(op),
                                                    )) => self.visit_asymmetric_object_property(op, target),

                Component::TransitiveObjectProperty(TransitiveObjectProperty(
                                                        ObjectPropertyExpression::ObjectProperty(op),
                                                    )) => self.visit_transitive_object_property(op, target),

                Component::SubDataPropertyOf(SubDataPropertyOf { sub, sup }) => {
                    self.visit_sub_data_property_of(sub, sup, target)
                }

                Component::EquivalentDataProperties(EquivalentDataProperties(es)) => {
                    self.visit_equivalent_data_properties(es, target)
                }

                Component::DisjointDataProperties(DisjointDataProperties(es)) => {
                    self.visit_disjoint_data_properties(es, target)
                }

                Component::DataPropertyDomain(DataPropertyDomain { dp, ce }) => {
                    self.visit_data_property_domain(dp, ce, target)
                }

                Component::DataPropertyRange(DataPropertyRange { dp, dr }) => {
                    self.visit_data_property_range(dp, dr, target)
                }

                Component::FunctionalDataProperty(FunctionalDataProperty(dp)) => {
                    self.visit_functional_data_property(dp, target)
                }

                Component::DatatypeDefinition(DatatypeDefinition { kind, range }) => {
                    self.visit_datatype_definition(kind, range, target)
                }

                Component::HasKey(_) => {}

                Component::SameIndividual(SameIndividual(es)) => self.visit_same_individual(es, target),

                Component::DifferentIndividuals(DifferentIndividuals(es)) => {
                    self.visit_different_individuals(es, target)
                }

                Component::ClassAssertion(ClassAssertion { ce, i }) => {
                    self.visit_class_assertion(ce, i, target)
                }

                Component::ObjectPropertyAssertion(_) => {}
                Component::NegativeObjectPropertyAssertion(_) => {}
                Component::DataPropertyAssertion(_) => {}
                Component::NegativeDataPropertyAssertion(_) => {}

                Component::AnnotationAssertion(aa) => {
                    self.visit_annotation_assertion(&aa.subject, &aa.ann, target);
                }

                Component::SubAnnotationPropertyOf(_) => {}
                Component::AnnotationPropertyDomain(_) => {}
                Component::AnnotationPropertyRange(_) => {}
                Component::Rule(_) => {}
                _ => {}
            }
        }
    }
}