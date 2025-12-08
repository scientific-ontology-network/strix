use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::time::SystemTime;
use horned_owl::model::{ClassExpression, ForIRI, ObjectPropertyExpression};
use horned_owl::ontology::indexed::ForIndex;
use horned_owl::ontology::set::SetOntology;
use strix_roost::dependency::base::{reduce_map, DependencyBuilder, OntologySymbol};
use strix_roost::dependency::growth::GrowthDependency;
use crate::onto::owl::class::ClassDetails;
use crate::onto::owl::visitor::AxiomVisitor;
use crate::onto::owl::hierarchy::OntologyView;
use crate::onto::serialize::OntologySymbolView;

#[derive(Default)]
pub struct StrixState<T> where T: ForIRI  {
    ontology: SetOntology<T>,
    dependencies: HashMap<T, HashSet<OntologySymbolView>>,
    pub(crate) reduced_dependencies: HashMap<T, HashSet<T>>,
    pub(crate) dependency_roots: HashSet<T>,

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
    pub fn get_hierarchy(&self) -> OntologyView {
        OntologyView::new(&self.ontology)
    }

    fn compute_dependencies(&mut self) {

        let dependency_map = GrowthDependency::build_dependencies(self.ontology.i().into_iter());
        let reduced_dependency_map = reduce_map(&dependency_map);
        println!("{:?}", reduced_dependency_map);
        let dependency_map = GrowthDependency::remove_super_expressions(dependency_map, self.ontology.i().into_iter());
        let symbol_dependency = GrowthDependency::remove_super_symbols(&reduced_dependency_map, self.ontology.i().into_iter());
        println!("{:?}", symbol_dependency);
        self.reduced_dependencies = invert_map(&symbol_dependency);
        println!("{:?}", self.reduced_dependencies);
        self.dependency_roots = find_roots(&self.reduced_dependencies);
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
}


fn find_roots<T>(m: &HashMap<T, HashSet<T>>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    let mut all_children = HashSet::new();
    for children in m.values() {
        all_children.extend(children.iter().cloned());
    }

    m.keys()
        .filter(|k| !all_children.contains(*k))
        .cloned()
        .collect()
}
pub fn invert_map<T: ForIRI>(map: &HashMap<T, HashSet<T>>) -> HashMap<T, HashSet<T>> {
    let mut new_map: HashMap<T, HashSet<T>> = HashMap::new();
    for (k,vset) in map {
        for v in vset {
            if !new_map.contains_key(&v) {
                new_map.insert(v.clone(), HashSet::new());
            }
            let l = new_map.get_mut(&v).unwrap();
            l.insert(k.clone());
        }
    }
    new_map
}