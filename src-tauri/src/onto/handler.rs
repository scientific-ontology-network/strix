use crate::util::StrixError;
use horned_owl::curie::PrefixMapping;
use horned_owl::error::HornedError;
use horned_owl::io::rdf::reader::{ConcreteRDFOntology, IncompleteParse};
use horned_owl::io::{rdf, ParserConfiguration, RDFParserConfiguration};
use horned_owl::model::{AnnotatedComponent, Annotation, AnnotationAssertion, AnnotationProperty, AnnotationSubject, AnnotationValue, AnonymousIndividual, ArcAnnotatedComponent, ArcStr, Build, Class, ClassExpression, Component, ForIRI, Literal, ObjectProperty, ObjectPropertyExpression, Ontology, IRI};
use horned_owl::model::{DeclareClass, SubClassOf};
use horned_owl::ontology::component_mapped::ComponentMappedIndex;
use horned_owl::ontology::indexed::ForIndex;
use horned_owl::ontology::iri_mapped::IRIMappedIndex;
use horned_owl::ontology::set::SetIndex;
use semantic_dependency::dependency::base::{DependencyBuilder, DependencyMap, OntologySymbol};
use semantic_dependency::dependency::growth::GrowthDependency;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use std::ops::Deref;
use std::sync::Arc;

trait OntologyHandling<T: ForIRI, O: Ontology<T>> {
    fn derive_subclass_pairs(ontology: &O) -> impl Iterator<Item = (String, String)>;
    fn derive_annotations(ontology: &O) -> HashMap<String, HashMap<String, HashSet<String>>>;
    fn derive_declared_classes(ontology: &O) -> HashSet<String>;

    fn _derive_classes_from_class_expression(ce: ClassExpression<T>) -> HashSet<T> {
        match ce {
            ClassExpression::Class(Class(iri)) => HashSet::from([iri.underlying()]),
            ClassExpression::ObjectIntersectionOf(v) => v
                .into_iter()
                .flat_map(|ce| Self::_derive_classes_from_class_expression(ce))
                .collect(),
            ClassExpression::ObjectUnionOf(v) => v
                .into_iter()
                .flat_map(|ce| Self::_derive_classes_from_class_expression(ce))
                .collect(),
            ClassExpression::ObjectComplementOf(ce) => {
                Self::_derive_classes_from_class_expression(*ce)
            }
            ClassExpression::ObjectSomeValuesFrom { ope, bce } => {
                Self::_derive_classes_from_class_expression(*bce)
            }
            ClassExpression::ObjectAllValuesFrom { ope, bce } => {
                Self::_derive_classes_from_class_expression(*bce)
            }
            ClassExpression::ObjectMinCardinality { n, ope, bce } => {
                Self::_derive_classes_from_class_expression(*bce)
            }
            ClassExpression::ObjectMaxCardinality { n, ope, bce } => {
                Self::_derive_classes_from_class_expression(*bce)
            }
            ClassExpression::ObjectExactCardinality { n, ope, bce } => {
                Self::_derive_classes_from_class_expression(*bce)
            }
            _ => HashSet::new(),
        }
    }
}

pub struct OntologyContainer {
    pub declared_classes: HashSet<String>,
    pub annotations: HashMap<String, HashMap<String, HashSet<String>>>,
    pub direct_subclasses: HashMap<String, HashSet<String>>,
    pub class_dependencies: HashMap<String, HashSet<String>>,
}

impl Default for OntologyContainer {
    fn default() -> Self {
        Self {
            declared_classes: HashSet::new(),
            annotations: HashMap::new(),
            direct_subclasses: HashMap::new(),
            class_dependencies: HashMap::new(),
        }
    }
}

impl<T: ForIRI> OntologyHandling<T, ConcreteRDFOntology<T, Arc<AnnotatedComponent<T>>>>
    for OntologyContainer
{
    fn derive_subclass_pairs(
        ontology: &ConcreteRDFOntology<T, Arc<AnnotatedComponent<T>>>,
    ) -> impl Iterator<Item = (String, String)> {
        let set_index = ontology.i();
        set_index
            .iter()
            .filter_map(|tt| match &Arc::as_ref(tt).component {
                Component::SubClassOf(SubClassOf {
                    sub: ClassExpression::Class(Class(sub_iri)),
                    sup: ClassExpression::Class(Class(sup_iri)),
                }) => Some((String::from(sup_iri.underlying().deref()), String::from(sub_iri.underlying().deref()))),
                _ => None,
            })
    }

    fn derive_annotations(
        ontology: &ConcreteRDFOntology<T, Arc<AnnotatedComponent<T>>>,
    ) -> HashMap<String, HashMap<String, HashSet<String>>> {
        let set_index = ontology.i();
        let pairs = set_index
            .iter()
            .filter_map(|tt| match &Arc::as_ref(tt).component {
                Component::AnnotationAssertion(AnnotationAssertion {
                    subject: AnnotationSubject::IRI(subj),
                    ann:
                        Annotation {
                            ap: AnnotationProperty(ap_iri),
                            av
                        },
                }) => Some((subj.underlying(), ap_iri.underlying(),
                match av {
                    AnnotationValue::Literal(l) => match l {
                        Literal::Simple {literal } => literal.clone(),
                        Literal::Language { literal, lang     } => literal.clone(), // Todo: Handle language tag
                        Literal::Datatype { literal, datatype_iri } => literal.clone(), // Todo: Handle datatype
                    }.clone(),
                    AnnotationValue::IRI(iri) => String::from(iri.underlying().deref()),
                    AnnotationValue::AnonymousIndividual(AnonymousIndividual(a)) => a.to_string(),
                    _ => format!("Unknown annotation value type: {:?}", av)
                })),
                _ => None,
            });
        let mut map = HashMap::new();
        for (subj, ap_iri, literal) in pairs {
            map.entry(String::from(subj.deref())).or_insert_with(HashMap::new).entry(String::from(ap_iri.deref())).or_insert_with(HashSet::new).insert(literal.clone());
        }
        map
    }

    fn derive_declared_classes(
        ontology: &ConcreteRDFOntology<T, Arc<AnnotatedComponent<T>>>,
    ) -> HashSet<String> {
        let set_index = ontology.i();
        HashSet::from_iter(set_index.iter().filter_map(|ac| match &ac.component {
            Component::DeclareClass(DeclareClass(Class(iri))) => Some(String::from(iri.underlying().deref())),
            _ => None,
        }))
    }
}

impl OntologyContainer {
    pub fn load(&mut self, path: &str) {
        println!("Loading ontology from {}", path);
        let ending = path.split(".").last().unwrap();
        let res = match ending {
            "owl" => self.load_rdf_ontology(&path),
            _ => Err(StrixError::InternalStrixError {
                message: format!("Unknown file ending: {}", ending),
            }),
        };
        match res {
            Ok(oc) => oc,
            Err(e) => panic!("Error loading ontology: {}", e),
        }
    }

    fn load_rdf_ontology(&mut self, path: &str) -> Result<(), StrixError> {
        let file = File::open(path)?;
        let reader = &mut BufReader::new(file);
        let build = Build::new_arc();
        let (ontology, _incomplete_parse) =
            rdf::reader::read_with_build::<ArcStr, ArcAnnotatedComponent, BufReader<File>>(
                reader,
                &build,
                ParserConfiguration {
                    rdf: RDFParserConfiguration { lax: true },
                    ..Default::default()
                },
            )?;

        let mut subclass_map = HashMap::new();
        for (k, v) in Self::derive_subclass_pairs(&ontology) {
            subclass_map.entry(k).or_insert_with(HashSet::new).insert(v);
        }
        let dependencies = GrowthDependency::dep(ontology.i().into_iter());

        let mut class_dependency_map: HashMap<String, HashSet<String>> = HashMap::new();
        let mut property_dependency_map: HashMap<String, HashSet<String>> = HashMap::new();

        let filter_dependencies = |v: &OntologySymbol<ArcStr>| match v {
            OntologySymbol::CE(ClassExpression::Class(Class(iri))) => {
                Some(String::from(iri.underlying().deref()))
            }
            OntologySymbol::Role(ObjectPropertyExpression::ObjectProperty(ObjectProperty(iri))) => {
                Some(String::from(iri.underlying().deref()))
            }
            _ => None,
        };

        for (k, vs) in dependencies {
            match k {
                OntologySymbol::CE(ClassExpression::Class(Class(kiri))) => {
                    let _ = class_dependency_map.insert(
                        String::from(kiri.underlying().deref()),
                        vs.iter().filter_map(filter_dependencies).collect(),
                    );
                }
                OntologySymbol::Role(ObjectPropertyExpression::ObjectProperty(ObjectProperty(
                    kiri,
                ))) => {
                    let _ = property_dependency_map.insert(
                        kiri.underlying().clone().parse().unwrap(),
                        vs.iter().filter_map(filter_dependencies).collect(),
                    );
                }
                _ => (),
            }
        }

        self.declared_classes = Self::derive_declared_classes(&ontology);
        self.annotations = Self::derive_annotations(&ontology);
        self.direct_subclasses = subclass_map;
        self.class_dependencies = class_dependency_map;
        Ok(())
    }

    pub fn get_roots(&self) -> HashSet<String> {
        let subclasses = self
            .direct_subclasses
            .values()
            .flatten().map(|v| v.clone())
            .collect::<HashSet<String>>();
        let superclasses = HashSet::from_iter(self.direct_subclasses.keys().map(|v| v.clone()));
        let diff = superclasses.difference(&subclasses).map(|v| v.clone()).collect::<HashSet<String>>();
        diff
    }
}
