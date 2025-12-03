use std::collections::{HashMap, HashSet};
use std::time::SystemTime;
use horned_owl::model::{ArcStr, Class, ClassExpression, ForIRI, ObjectPropertyExpression, Ontology, IRI};
use horned_owl::model::Component::SubClassOf;
use horned_owl::model::HigherKind::Axiom;
use horned_owl::ontology::indexed::{ForIndex, OneIndexedOntology, OntologyIndex};
use horned_owl::ontology::set::SetOntology;
use strix_roost::dependency::base::{DependencyBuilder, DependencyMap, OntologySymbol};
use strix_roost::dependency::growth::GrowthDependency;
use crate::onto::owl::class::ClassDetails;
use crate::onto::owl::visitor::AxiomVisitor;
use crate::onto::owl::hierarchy::ClassHierarchy;
use crate::onto::serialize::OntologySymbolView;

#[derive(Default)]
pub struct StrixState<T> where T: ForIRI  {
    ontology: SetOntology<T>,
    dependencies: HashMap<T, HashSet<OntologySymbolView>>,
}

impl<T> StrixState<T> where T: ForIRI {

    pub fn set_ontology(&mut self, ontology: SetOntology<T>) {
        self.ontology = ontology;
        self.compute_dependencies();
    }

    pub fn get_class_details(
        &self,
        iri: &T
    ) -> ClassDetails<T> {

        let start = SystemTime::now();
        let mut cd = ClassDetails::default();
        cd.visit_components(self.ontology.i().iter(), iri);
        cd.depends_on = self.dependencies.get(iri).unwrap_or(&HashSet::new()).clone();
        cd
    }

    pub fn calculate_roots(&self){

    }
    pub fn get_hierarchy(&self) -> ClassHierarchy {
        ClassHierarchy::new(&self.ontology)
    }

    fn compute_dependencies(&mut self) {
        let dependency_map = GrowthDependency::build_dependencies(self.ontology.i().into_iter());
        let dependency_map = GrowthDependency::remove_supers(dependency_map, self.ontology.i().into_iter());
        for (k,v) in dependency_map.iter() {
            match k {
                OntologySymbol::CE(ClassExpression::Class(iri)) => {
                    self.dependencies.insert(iri.underlying(), v.iter().map(|x|x.into()).collect());
                }
                OntologySymbol::Role(ObjectPropertyExpression::ObjectProperty(iri)) => {
                    self.dependencies.insert(iri.underlying(), v.iter().map(|x|x.into()).collect());
                }
                _ => {}
            }
        }
    }

    pub fn digest_dependencies(&mut self, dependency_map: DependencyMap<T>){

    }

}