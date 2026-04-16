use crate::onto::owl::class::ClassDetails;
use crate::onto::owl::hierarchy::OntologyView;
use crate::onto::serialize::OntologySymbolView;
use horned_owl::model::{ClassExpression, Component, ForIRI, ObjectPropertyExpression};
use horned_owl::ontology::set::SetOntology;
use serde::de::Unexpected::Str;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use strix_roost::dependency::base::{invert_map, remove_super_symbols, DependencyBuilder};
use strix_roost::dependency::growth::GrowthDependency;
use strix_roost::dependency::symbol::{Symbol, Term};
use strix_roost::ontology::visitor::AxiomVisitor;

#[derive(Default)]
pub struct StrixState<T>
where
    T: ForIRI,
{
    pub(crate) ontology: SetOntology<T>,
}

fn find_roots<T, S>(m: &HashMap<T, HashMap<T, S>>) -> HashSet<T>
where
    T: Eq + Hash + Clone,
{
    let mut all_children = HashSet::new();
    for children in m.values() {
        all_children.extend(children.iter().map(|(t, _)| t).cloned());
    }

    m.keys()
        .filter(|&k| !all_children.contains(k))
        .cloned()
        .collect()
}
