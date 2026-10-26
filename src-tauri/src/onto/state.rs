use crate::onto::serialize::OntologySymbolView;
use horned_owl::model::ForIRI;
use horned_owl::ontology::set::SetOntology;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

#[derive(Default)]
pub struct StrixState<T>
where
    T: ForIRI,
{
    pub(crate) ontology: SetOntology<T>,
    pub(crate) dependencies: HashMap<OntologySymbolView, HashSet<OntologySymbolView>>,
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
