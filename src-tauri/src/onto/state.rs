use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use horned_owl::model::{ClassExpression, ForIRI, ObjectPropertyExpression};
use horned_owl::ontology::set::SetOntology;
use strix_roost::dependency::base::{reduce_map, DependencyBuilder};
use strix_roost::dependency::symbol::{DependencyMap, DependencySymbol, DependencySymbolWithAxioms, OntologySymbol, SymbolContainer};
use strix_roost::dependency::growth::{GrowthDependency, remove_super_expressions, remove_super_symbols, invert_map};
use crate::onto::owl::class::ClassDetails;
use strix_roost::ontology::visitor::AxiomVisitor;
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
        let mut cd = ClassDetails::default();
        cd.visit_components(self.ontology.i().iter(), iri);
        cd.depends_on = self.dependencies.get(iri).unwrap_or(&HashSet::new()).clone();
        cd
    }

    pub fn get_hierarchy(&self) -> OntologyView {
        OntologyView::new(&self.ontology)
    }

    fn compute_dependencies(&mut self) {

        let dependency_map: DependencyMap<OntologySymbol<T>, DependencySymbol<OntologySymbol<T>>> = GrowthDependency::build_dependencies(self.ontology.i().into_iter());
        let reduced_dependency_map = reduce_map(&dependency_map);
        let dependency_map = remove_super_expressions(dependency_map, self.ontology.i().into_iter(), |v|v.clone());
        let symbol_dependency = remove_super_symbols(&reduced_dependency_map, self.ontology.i().into_iter(), |v|v.clone());
        let reduced_dependencies = invert_map::<OntologySymbol<'_, T>, (), DependencySymbol<OntologySymbol<'_, T>>>(&symbol_dependency).iter().map(|(k,vs)| (k.get_iri().unwrap(),vs.iter().map(|v | <DependencySymbol<OntologySymbol<'_, T>> as SymbolContainer<OntologySymbol<'_, T>, ()>>::get_symbol(v).get_iri().unwrap()).collect())).collect();
        self.reduced_dependencies = reduced_dependencies;
        self.dependency_roots = find_roots(&self.reduced_dependencies);
        for (k,v) in dependency_map.iter() {
            match k {
                OntologySymbol::CE(ClassExpression::Class(iri)) => {
                    self.dependencies.insert(iri.underlying(), v.iter().map(|x|<DependencySymbol<OntologySymbol<'_, T>> as SymbolContainer<OntologySymbol<'_, T>, ()>>::get_symbol(x).into()).collect());
                }
                OntologySymbol::Role(ObjectPropertyExpression::ObjectProperty(iri)) => {
                    self.dependencies.insert(iri.underlying(), v.iter().map(|x|<DependencySymbol<OntologySymbol<'_, T>> as SymbolContainer<OntologySymbol<'_, T>, ()>>::get_symbol(x).into()).collect());
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
        all_children.extend(children.iter().map(|sc| sc).cloned());
    }

    m.keys()
        .filter(|&k| !all_children.contains(k))
        .cloned()
        .collect()
}
