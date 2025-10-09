use std::collections::{HashMap, HashSet};
use horned_owl::model::{AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationProperty, AnnotationSubject, AnnotationValue, AnonymousIndividual, AsymmetricObjectProperty, Class, ClassAssertion, ClassExpression, Component, DataProperty, DataPropertyDomain, DataPropertyRange, DataRange, Datatype, DatatypeDefinition, DeclareAnnotationProperty, DeclareClass, DeclareDataProperty, DeclareDatatype, DeclareNamedIndividual, DeclareObjectProperty, DifferentIndividuals, DisjointClasses, DisjointDataProperties, DisjointObjectProperties, DisjointUnion, EquivalentClasses, EquivalentDataProperties, EquivalentObjectProperties, ForIRI, FunctionalDataProperty, FunctionalObjectProperty, Import, Individual, InverseFunctionalObjectProperty, InverseObjectProperties, IrreflexiveObjectProperty, Literal, NamedIndividual, ObjectProperty, ObjectPropertyDomain, ObjectPropertyExpression, ObjectPropertyRange, OntologyAnnotation, OntologyID, ReflexiveObjectProperty, SameIndividual, SubClassOf, SubDataPropertyOf, SubObjectPropertyExpression, SubObjectPropertyOf, SymmetricObjectProperty, TransitiveObjectProperty, IRI};
use semantic_dependency::dependency::base::{DependencyMap, OntologySymbol};
use semantic_dependency::dependency::growth::GrowthDependency;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::owl::annotation::{wrap_annotations, AnnotationPropertyDetails};
use crate::onto::owl::class::{ClassDetails, _derive_classes_from_class_expression};
use crate::onto::owl::datatype::DatatypeDetails;
use crate::onto::owl::individual::IndividualDetails;
use crate::onto::owl::property::{DataPropertyDetails, ObjectPropertyDetails};
use crate::onto::rdf::RDFS_LABEL;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
struct OntologyDetails<T: ForIRI + Default> {
    pub iri: String,
    pub annotations: HashMap<String, Vec<AnnotationValue<T>>>,
    pub imports: Vec<String>,
}

#[derive(Default, Debug, Clone)]
pub struct OntologyContainer<T: ForIRI + Default> {
    pub class_details: HashMap<String, ClassDetails<T>>,
    pub object_property_details: HashMap<String, ObjectPropertyDetails<T>>,
    pub data_properties_details: HashMap<String, DataPropertyDetails<T>>,
    pub annotation_properties_details: HashMap<String, AnnotationPropertyDetails<T>>,
    pub individuals_details: HashMap<String, IndividualDetails<T>>,
    pub datatype_details: HashMap<String, DatatypeDetails<T>>,

    pub dependencies: HashMap<OntologySymbol<T>, HashSet<OntologySymbol<T>>>,
    pub details: OntologyDetails<T>,
}



impl<T> OntologyContainer<T>
where
    T: ForIRI + Default,
{
    fn digest_annotation(
        anno_map: &mut HashMap<String, Vec<AnnotationValue<T>>>,
        ann: Annotation<T>,
    ) {
        let ap_id = ann.ap.underlying().to_string();
        let annotations = anno_map.get_mut(&ap_id);
        let mut annotation_list = match annotations {
            Some(annotation_list) => annotation_list,
            _ => {
                let new_list = Vec::new();
                anno_map.insert(ap_id.clone(), new_list);
                anno_map.get_mut(&ap_id).unwrap()
            }
        };
        annotation_list.push(ann.av);
    }

    pub fn digest_dependencies(&mut self, dependency_map: DependencyMap<T>){
        for (k,v) in dependency_map.into_iter() {
            match k {
                OntologySymbol::CE(ClassExpression::Class(c)) => {
                    let details = self.class_details.get_mut(&c.underlying().to_string()).unwrap();
                    details.depends_on = v;
                }
                OntologySymbol::Role(ObjectPropertyExpression::ObjectProperty(o)) => {
                    let details = self.object_property_details.get_mut(&o.underlying().to_string()).unwrap();
                    details.depends_on = v;
                }
                _ => {}
            }
        }
    }

}

fn get_or_insert_default<'a, T: ForIRI, S>(map: &'a mut HashMap<T, S>, ciri: &T) -> &'a mut S
where S: 'a + Default
{
    if !map.contains_key(&ciri) {
        let new_s = S::default();
        map.insert(ciri.clone(), new_s);
    }
    map.get_mut(&ciri).unwrap()
}


impl<T: ForIRI + Default> OntologyContainer<T> {
    fn handle_ontology_id(&mut self, iri: &IRI<T>, _viri: Option<T>) {
        self.details.iri = iri.to_string();
    }

    fn handle_ontology_annotation(&mut self, ann: Annotation<T>) {
        Self::digest_annotation(&mut self.details.annotations, ann);
    }

    fn handle_import(&mut self, iri: &IRI<T>) {
        self.details.imports.push(iri.underlying().to_string());
    }

    fn handle_declare_class(&mut self, iri: &IRI<T>) {
        get_or_insert_default(&mut self.class_details, &iri.to_string());
    }

    fn handle_declare_object_property(&mut self, iri: &IRI<T>) {
        get_or_insert_default(&mut self.object_property_details, &iri.to_string());
    }

    fn handle_declare_annotation_property(&mut self, iri: &IRI<T>) {
        get_or_insert_default(&mut self.annotation_properties_details, &iri.to_string());
    }

    fn handle_declare_data_property(&mut self, iri: &IRI<T>) {
        get_or_insert_default(&mut self.data_properties_details, &iri.to_string());
    }

    fn handle_declare_named_individual(&mut self, iri: &IRI<T>) {
        get_or_insert_default(&mut self.individuals_details, &iri.to_string());
    }

    fn handle_declare_datatype(&mut self, iri: &IRI<T>) {
        get_or_insert_default(&mut self.datatype_details, &iri.to_string());
    }

    fn handle_subclass_of(&mut self, sub: &ClassExpression<T>, sup: &ClassExpression<T>) {
        if let ClassExpression::Class(Class(ref ciri)) = sub {
            let class_details =
                get_or_insert_default(&mut self.class_details, &ciri.to_string());
            class_details.subclass_of.insert(sup.clone());
        }
        if let ClassExpression::Class(Class(ref ciri)) = sup {
            let class_details =
                get_or_insert_default(&mut self.class_details, &ciri.to_string());
            class_details.superclass_of.insert(sub.clone());
        }
    }

    fn handle_equivalent_classes(&mut self, cs: &Vec<ClassExpression<T>>) {
        for ce in cs.iter() {
            if let ClassExpression::Class(Class(ref ciri)) = ce {
                let class_details =
                    get_or_insert_default(&mut self.class_details, &ciri.to_string());
                for ce2 in cs.iter() {
                    if ce2 != ce {
                        class_details.equivalent_to.insert(ce.clone());
                    }
                }
            }
        }
    }

    fn handle_disjoint_classes(&mut self, cs: &Vec<ClassExpression<T>>) {
        for ce in cs.iter() {
            if let ClassExpression::Class(Class(ciri)) = ce {
                let class_details =
                    get_or_insert_default(&mut self.class_details, &ciri.to_string());
                for ce2 in cs.iter() {
                    if ce2 != ce {
                        class_details.disjoint_with.insert(ce.clone());
                    }
                }
            }
        }
    }

    fn handle_disjoint_union(&mut self, ciri: &IRI<T>, cs: &Vec<ClassExpression<T>>) {
        get_or_insert_default(&mut self.class_details, &ciri.underlying().to_string())
            .disjoint_union_of
            .push(cs.clone());
    }

    fn handle_sub_object_property_of(
        &mut self,
        sub: &SubObjectPropertyExpression<T>,
        sup: &ObjectPropertyExpression<T>,
    ) {
        if let SubObjectPropertyExpression::ObjectPropertyExpression(
            ObjectPropertyExpression::ObjectProperty(ObjectProperty(ref ciri)),
        ) = sub
        {
            get_or_insert_default(&mut self.object_property_details, &ciri.to_string())
                .subproperty_of
                .insert(sup.clone());
        }
        if let ObjectPropertyExpression::ObjectProperty(ObjectProperty(ref ciri)) = sup {
            get_or_insert_default(&mut self.object_property_details, &ciri.to_string())
                .superproperty_of
                .insert(sub.clone());
        }
    }

    fn handle_equivalent_object_properties(&mut self, es: &Vec<ObjectPropertyExpression<T>>) {
        for ce in es.iter() {
            if let ObjectPropertyExpression::ObjectProperty(ObjectProperty(ciri)) = ce {
                let class_details =
                    get_or_insert_default(&mut self.object_property_details, &ciri.to_string());
                for ce2 in es.iter() {
                    if ce2 != ce {
                        class_details.equivalent_to.insert(ce.clone());
                    }
                }
            }
        }
    }

    fn handle_disjoint_object_properties(&mut self, es: &Vec<ObjectPropertyExpression<T>>) {
        for ce in es.iter() {
            if let ObjectPropertyExpression::ObjectProperty(ObjectProperty(ciri)) = ce {
                let class_details =
                    get_or_insert_default(&mut self.object_property_details, &ciri.to_string());
                for ce2 in es.iter() {
                    if ce2 != ce {
                        class_details.disjoint_with.insert(ce.clone());
                    }
                }
            }
        }
    }

    fn handle_inverse_object_properties(
        &mut self,
        a: &ObjectProperty<T>,
        b: &ObjectProperty<T>,
    ) {
        get_or_insert_default(&mut self.object_property_details, &a.to_string())
            .inverse_of
            .insert(b.clone());
        get_or_insert_default(&mut self.object_property_details, &b.to_string())
            .inverse_of
            .insert(a.clone());
    }

    fn handle_object_property_domain(&mut self, oiri: &IRI<T>, ce: &ClassExpression<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.underlying().to_string())
            .domain
            .insert(ce.clone());
    }

    fn handle_object_property_range(&mut self, oiri: &IRI<T>, ce: &ClassExpression<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.underlying().to_string())
            .range
            .insert(ce.clone());
    }

    fn handle_functional_object_property(&mut self, oiri: &IRI<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.to_string()).is_functional = true;
    }

    fn handle_inverse_functional_object_property(&mut self, oiri: &IRI<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.to_string())
            .is_inverse_functional = true;
    }

    fn handle_reflexive_object_property(&mut self, oiri: &IRI<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.to_string()).is_reflexive = true;
    }

    fn handle_irreflexive_object_property(&mut self, oiri: &IRI<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.to_string()).is_irreflexive = true;
    }

    fn handle_symmetric_object_property(&mut self, oiri: &IRI<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.to_string()).is_symmetric = true;
    }

    fn handle_asymmetric_object_property(&mut self, oiri: &IRI<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.to_string()).is_asymmetric = true;
    }

    fn handle_transitive_object_property(&mut self, oiri: &IRI<T>) {
        get_or_insert_default(&mut self.object_property_details, &oiri.to_string()).is_transitive = true;
    }

    fn handle_sub_data_property_of(&mut self, sub: &DataProperty<T>, sup: &DataProperty<T>) {
        get_or_insert_default(&mut self.data_properties_details, &sub.to_string())
            .subproperty_of
            .insert(sup.clone());
        get_or_insert_default(&mut self.data_properties_details, &sup.to_string())
            .superproperty_of
            .insert(sub.clone());
    }

    fn handle_equivalent_data_properties(&mut self, es: &Vec<DataProperty<T>>) {
        for a in es.iter() {
            for b in es.iter() {
                if a != b {
                    get_or_insert_default(&mut self.data_properties_details, &a.to_string())
                        .equivalent_to
                        .insert(b.clone());
                }
            }
        }
    }

    fn handle_disjoint_data_properties(&mut self, es: &Vec<DataProperty<T>>) {
        for a in es.iter() {
            for b in es.iter() {
                if a != b {
                    get_or_insert_default(&mut self.data_properties_details, &a.to_string())
                        .disjoint_with
                        .insert(b.clone());
                }
            }
        }
    }

    fn handle_data_property_domain(&mut self, dp: &DataProperty<T>, ce: &ClassExpression<T>) {
        get_or_insert_default(&mut self.data_properties_details, &dp.to_string())
            .domain
            .insert(ce.clone());
    }

    fn handle_data_property_range(&mut self, dp: &DataProperty<T>, dr: &DataRange<T>) {
        get_or_insert_default(&mut self.data_properties_details, &dp.to_string())
            .range
            .insert(dr.clone());
    }

    fn handle_functional_data_property(&mut self, dp: &T) {
        get_or_insert_default(&mut self.data_properties_details, &dp.to_string()).is_functional = true;
    }

    fn handle_datatype_definition(&mut self, kind: &T, range: &DataRange<T>) {
        get_or_insert_default(&mut self.datatype_details, &kind.to_string()).definition = Some(range.clone());
    }

    fn handle_same_individual(&mut self, es: &Vec<Individual<T>>) {
        for a in es.iter() {
            if let Individual::Named(NamedIndividual(iri)) = a {
                for b in es.iter() {
                    if a != b {
                        get_or_insert_default(&mut self.individuals_details, &iri.to_string())
                            .sames
                            .insert(b.clone());
                    }
                }
            }
        }
    }

    fn handle_different_individuals(&mut self, es: &Vec<Individual<T>>) {
        for a in es.iter() {
            if let Individual::Named(NamedIndividual(iri)) = a {
                for b in es.iter() {
                    if a != b {
                        get_or_insert_default(&mut self.individuals_details, &iri.to_string())
                            .different
                            .insert(b.clone());
                    }
                }
            }
        }
    }

    fn handle_class_assertion(&mut self, ce: &ClassExpression<T>, i: &Individual<T>) {
        if let ClassExpression::Class(ref c) = ce {
            get_or_insert_default(&mut self.class_details, &c.to_string())
                .individuals
                .insert(i.clone());
        }
        if let Individual::Named(ni) = i {
            get_or_insert_default(&mut self.individuals_details, &ni.to_string())
                .member_of
                .insert(ce.clone());
        }
    }

    fn handle_annotation_assertion(
        &mut self,
        subject: &AnnotationSubject<T>,
        ann: &Annotation<T>,
    ) {
        match subject {
            AnnotationSubject::IRI(iri) => {
                let subject_iri = iri.to_string();
                let ap_iri = ann.ap.underlying().to_string();
                // Todo: Figure out a better way to handle this without having to clone the annotation
                if (self.details.iri == subject_iri) {
                    get_or_insert_default(&mut self.details.annotations, &ap_iri).push(ann.av.clone());
                }
                if let Some(details) = self.class_details.get_mut(&subject_iri) {
                    get_or_insert_default(&mut details.annotations, &ap_iri).push(ann.av.clone());
                }
                if let Some(details) = self.object_property_details.get_mut(&subject_iri) {
                    get_or_insert_default(&mut details.annotations, &ap_iri).push(ann.av.clone());
                }
                if let Some(details) = self.data_properties_details.get_mut(&subject_iri) {
                    get_or_insert_default(&mut details.annotations, &ap_iri).push(ann.av.clone());
                }
                if let Some(details) = self.individuals_details.get_mut(&subject_iri) {
                    get_or_insert_default(&mut details.annotations, &ap_iri).push(ann.av.clone());
                }
                if let Some(details) = self.datatype_details.get_mut(&subject_iri) {
                    get_or_insert_default(&mut details.annotations, &ap_iri).push(ann.av.clone());
                }
                if let Some(details) = self.annotation_properties_details.get_mut(&subject_iri) {
                    get_or_insert_default(&mut details.annotations, &ap_iri).push(ann.av.clone());
                }
            }
            AnnotationSubject::AnonymousIndividual(_) => {}
        }
    }

    pub fn handle_component(&mut self, component: &AnnotatedComponent<T>) {
        match &component.component {
            Component::OntologyID(OntologyID {
                                      iri: Some(piri),
                                      viri,
                                  }) => self.handle_ontology_id(piri, viri.as_ref().map(|v| v.underlying())),

            Component::DocIRI(_) => {}

            Component::OntologyAnnotation(OntologyAnnotation(ref ann)) => {
                self.handle_ontology_annotation(ann.clone())
            }

            Component::Import(Import(iri)) => self.handle_import(iri),

            Component::DeclareClass(DeclareClass(Class(iri))) => self.handle_declare_class(&iri),

            Component::DeclareObjectProperty(DeclareObjectProperty(ObjectProperty(iri))) => {
                self.handle_declare_object_property(&iri)
            }

            Component::DeclareAnnotationProperty(DeclareAnnotationProperty(
                                                     AnnotationProperty(iri),
                                                 )) => self.handle_declare_annotation_property(&iri),

            Component::DeclareDataProperty(DeclareDataProperty(DataProperty(iri))) => {
                self.handle_declare_data_property(&iri)
            }

            Component::DeclareNamedIndividual(DeclareNamedIndividual(NamedIndividual(iri))) => {
                self.handle_declare_named_individual(&iri)
            }

            Component::DeclareDatatype(DeclareDatatype(Datatype(iri))) => {
                self.handle_declare_datatype(&iri)
            }

            Component::SubClassOf(SubClassOf { sub, sup }) => self.handle_subclass_of(sub, sup),

            Component::EquivalentClasses(EquivalentClasses(cs)) => {
                self.handle_equivalent_classes(cs)
            }

            Component::DisjointClasses(DisjointClasses(cs)) => self.handle_disjoint_classes(cs),

            Component::DisjointUnion(DisjointUnion(Class(ciri), cs)) => {
                self.handle_disjoint_union(&ciri, cs)
            }

            Component::SubObjectPropertyOf(SubObjectPropertyOf { sub, sup }) => {
                self.handle_sub_object_property_of(sub, sup)
            }

            Component::EquivalentObjectProperties(EquivalentObjectProperties(es)) => {
                self.handle_equivalent_object_properties(es)
            }

            Component::DisjointObjectProperties(DisjointObjectProperties(es)) => {
                self.handle_disjoint_object_properties(es)
            }

            Component::InverseObjectProperties(InverseObjectProperties(a, b)) => {
                self.handle_inverse_object_properties(a, b)
            }

            Component::ObjectPropertyDomain(ObjectPropertyDomain {
                                                ope: ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                                ce,
                                            }) => self.handle_object_property_domain(&oiri, ce),

            Component::ObjectPropertyRange(ObjectPropertyRange {
                                               ope: ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                               ce,
                                           }) => self.handle_object_property_range(&oiri, ce),

            Component::FunctionalObjectProperty(FunctionalObjectProperty(
                                                    ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                                )) => self.handle_functional_object_property(&oiri),

            Component::InverseFunctionalObjectProperty(InverseFunctionalObjectProperty(
                                                           ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                                       )) => self.handle_inverse_functional_object_property(&oiri),

            Component::ReflexiveObjectProperty(ReflexiveObjectProperty(
                                                   ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                               )) => self.handle_reflexive_object_property(&oiri),

            Component::IrreflexiveObjectProperty(IrreflexiveObjectProperty(
                                                     ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                                 )) => self.handle_irreflexive_object_property(&oiri),

            Component::SymmetricObjectProperty(SymmetricObjectProperty(
                                                   ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                               )) => self.handle_symmetric_object_property(&oiri),

            Component::AsymmetricObjectProperty(AsymmetricObjectProperty(
                                                    ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                                )) => self.handle_asymmetric_object_property(&oiri),

            Component::TransitiveObjectProperty(TransitiveObjectProperty(
                                                    ObjectPropertyExpression::ObjectProperty(ObjectProperty(oiri)),
                                                )) => self.handle_transitive_object_property(&oiri),

            Component::SubDataPropertyOf(SubDataPropertyOf { sub, sup }) => {
                self.handle_sub_data_property_of(sub, sup)
            }

            Component::EquivalentDataProperties(EquivalentDataProperties(es)) => {
                self.handle_equivalent_data_properties(es)
            }

            Component::DisjointDataProperties(DisjointDataProperties(es)) => {
                self.handle_disjoint_data_properties(es)
            }

            Component::DataPropertyDomain(DataPropertyDomain { dp, ce }) => {
                self.handle_data_property_domain(dp, ce)
            }

            Component::DataPropertyRange(DataPropertyRange { dp, dr }) => {
                self.handle_data_property_range(dp, dr)
            }

            Component::FunctionalDataProperty(FunctionalDataProperty(DataProperty(dp))) => {
                self.handle_functional_data_property(&dp.underlying())
            }

            Component::DatatypeDefinition(DatatypeDefinition { kind, range }) => {
                self.handle_datatype_definition(&kind.underlying(), range)
            }

            Component::HasKey(_) => {}

            Component::SameIndividual(SameIndividual(es)) => self.handle_same_individual(es),

            Component::DifferentIndividuals(DifferentIndividuals(es)) => {
                self.handle_different_individuals(es)
            }

            Component::ClassAssertion(ClassAssertion { ce, i }) => {
                self.handle_class_assertion(ce, i)
            }

            Component::ObjectPropertyAssertion(_) => {}
            Component::NegativeObjectPropertyAssertion(_) => {}
            Component::DataPropertyAssertion(_) => {}
            Component::NegativeDataPropertyAssertion(_) => {}

            Component::AnnotationAssertion(AnnotationAssertion { subject, ann }) => {
                self.handle_annotation_assertion(subject, ann)
            }

            Component::SubAnnotationPropertyOf(_) => {}
            Component::AnnotationPropertyDomain(_) => {}
            Component::AnnotationPropertyRange(_) => {}
            Component::Rule(_) => {}
            _ => {}
        }
    }

    fn calculate_dependencies(&mut self) {
        //self.dependencies = GrowthDependency::dep(self.ontology.i().into_iter());
    }

    pub fn calculate_roots_classes(&self) -> Vec<String>{
        let classes = self.class_details.keys().cloned().collect::<HashSet<_>>();
        let sub_classes = &self.class_details.values().flat_map(
            |cd| cd.superclass_of.iter().flat_map(|ce| _derive_classes_from_class_expression(&ce).iter().map(|c| c.to_string()).collect::<Vec<String>>())).collect();
        classes.difference(&sub_classes).cloned().collect()
    }

    pub fn calculate_class_hierarchy(&self) -> HashMap<String, HashSet<String>>{
        let mut map =HashMap::new();
        for c in  self.class_details.keys(){
            let details = self.class_details.get(c).unwrap();
            let mut superclasses = HashSet::new();
            for ce in details.superclass_of.iter(){
                for c in _derive_classes_from_class_expression(&ce).iter(){
                    superclasses.insert(c.to_string());
                }
            }
            map.insert(c.to_string(), superclasses);
        };
        map
    }

    fn extract_labels<S, F>(details: &S, get_annotations: F) -> Option<String>
    where F: FnOnce (&S,) -> &HashMap<String, Vec<AnnotationValue<T>>>{
        match get_annotations(details).get(&RDFS_LABEL.to_string()){
            Some(v) => {
                let first = v.get(0)?;
                match first {
                    AnnotationValue::Literal(ref l) => match l {
                        Literal::Simple { ref literal } => Some(literal.clone()),
                        Literal::Language { ref literal, ref lang } => Some(literal.clone()),
                        Literal::Datatype { ref literal, ref datatype_iri } => Some(literal.clone())
                    }
                    AnnotationValue::IRI(ref l) => Some(l.to_string()),
                    AnnotationValue::AnonymousIndividual(AnonymousIndividual(ref a)) => Some(a.to_string()),
                }
            },
            _ => None
        }
    }

    pub fn calculate_label_map(&self) -> HashMap<String, String>{
        let mut map =HashMap::new();
        for (iri, details) in self.class_details.iter() {
            match Self::extract_labels(details, |s| &s.annotations ) {
                None => (),
                Some(l) => {map.insert(iri.clone(), l);}
            }
        }
        for (iri, details) in self.object_property_details.iter() {
            match Self::extract_labels(details, |s| &s.annotations ) {
                None => (),
                Some(l) => {map.insert(iri.clone(), l);}
            }
        }
        for (iri, details) in self.object_property_details.iter() {
            match Self::extract_labels(details, |s| &s.annotations ) {
                None => (),
                Some(l) => {map.insert(iri.clone(), l);}
            }
        }
        for (iri, details) in self.data_properties_details.iter() {
            match Self::extract_labels(details, |s| &s.annotations ) {
                None => (),
                Some(l) => {map.insert(iri.clone(), l);}
            }
        }
        for (iri, details) in self.individuals_details.iter() {
            match Self::extract_labels(details, |s| &s.annotations ) {
                None => (),
                Some(l) => {map.insert(iri.clone(), l);}
            }
        }
        for (iri, details) in self.annotation_properties_details.iter() {
            match Self::extract_labels(details, |s| &s.annotations ) {
                None => (),
                Some(l) => {map.insert(iri.clone(), l);}
            }
        }
        for (iri, details) in self.datatype_details.iter() {
            match Self::extract_labels(details, |s| &s.annotations ) {
                None => (),
                Some(l) => {map.insert(iri.clone(), l);}
            }
        }
        map
    }
}


impl<T: ForIRI + Default + Serialize> Serialize for OntologyDetails<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("OntologyDetails", 3)?;
        state.serialize_field("iri", &self.iri)?;

        state.serialize_field("annotations", &wrap_annotations(&self.annotations))?;
        state.serialize_field("imports", &self.imports)?;
        state.end()
    }
}
