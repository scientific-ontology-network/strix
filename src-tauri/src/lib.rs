// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod onto;

use crate::onto::owl::hierarchy::{find_roots, OntologyView};
use crate::onto::state::StrixState;
use horned_owl::io::ofn::writer::AsFunctional;
use horned_owl::model::{ArcStr, Build, IRI};
use onto::owl::class::ClassDetails;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::SystemTime;
use strix_roost::dependency::base::{DependencyBuilder, remove_super_symbols};
use strix_roost::dependency::empty::SyntacticEmptinessDependency;
use strix_roost::ontology::io::load_set_ontology;
use strix_roost::ontology::visitor::AxiomVisitor;
use strix_roost::util::error::StrixError;
use strix_roost::util::graph::{transitive_closure, transitive_closure_with_data};
use tauri::{Manager, State};
use crate::onto::serialize::OntologySymbolView;

#[tauri::command]
fn load_ontology<'a>(
    raw_state: State<'a, Mutex<StrixState<ArcStr>>>,
    path: &str,
) -> Result<
    (
        OntologyView,
        HashSet<(OntologySymbolView, Vec<OntologySymbolView>)>,
        HashSet<OntologySymbolView>,
    ),
    StrixError,
> {
    let mut state = raw_state.lock().unwrap();
    let start = SystemTime::now();
    match load_set_ontology(path) {
        Ok(ontology) => {
            state.ontology = ontology;
            println!("Load ontology");
            let ontology_view = OntologyView::new(&state.ontology);
            println!("Calculate dependencies");
            let dependencies = SyntacticEmptinessDependency::build_dependencies(state.ontology.i().iter());
            let reduced_dependencies = remove_super_symbols(&dependencies, state.ontology.i().iter());
            let dependencies_views: HashMap<OntologySymbolView, HashSet<OntologySymbolView>> = reduced_dependencies
                .iter()
                .map(|(k, vm)| {
                    (
                        OntologySymbolView::from(k),
                        vm.keys().map(|k2|OntologySymbolView::from(k2)).collect()
                    )
                })
                .collect();

            state.dependencies = dependencies_views;
            println!("Calculate dependency roots");
            let dependency_roots: HashSet<OntologySymbolView> = find_roots(&state.dependencies);
            let deps = state.dependencies.iter().map(|(k,v)|(k.clone(), v.clone().into_iter().collect())).collect();
            println!("Done!");
            Ok((
                ontology_view,
                deps,
                dependency_roots,
            ))
        }
        Err(err) => Err(err),
    }
}

#[tauri::command]
fn get_class_details<'a>(
    state: State<'a, Mutex<StrixState<ArcStr>>>,
    s: &str,
) -> Result<ClassDetails<ArcStr>, StrixError> {
    let state = state.lock().unwrap();
    let b_arc = Build::new_arc();
    let iri = b_arc.iri(s.to_string());
    let mut class_details = ClassDetails::default();
    for cd in ClassDetails::visit_components(state.ontology.i().iter(), Some(&iri.underlying())) {
        class_details = class_details.merge(cd);
    }
    Ok(class_details)
}

#[tauri::command]
fn dependency_diff<'a>(
    state: State<'a, Mutex<StrixState<ArcStr>>>,
    path: &str,
) -> Result<(Vec<(OntologySymbolView, HashSet<OntologySymbolView>)>,Vec<(OntologySymbolView, HashSet<OntologySymbolView>)>), StrixError> {

    let state = state.lock().unwrap();
    println!("Load ontology");
    match load_set_ontology(path) {
        Ok(ontology) => {
            let mut left_not_right = HashMap::new();
            let mut right_not_left = HashMap::new();
            let right_symbol_dependencies = SyntacticEmptinessDependency::build_dependencies(ontology.i().iter());
            let reduced_dependencies = remove_super_symbols(&right_symbol_dependencies, state.ontology.i().iter());
            let right_dependencies: &HashMap<_, _> = &reduced_dependencies.into_iter().map(|(k, vm)| (OntologySymbolView::from(&k), vm.into_iter().map(|(k2,vn)| OntologySymbolView::from(&k2)).collect())).collect();

            let left_dependencies = &state.dependencies;
            let all_symbols: HashSet<_> = left_dependencies.keys().chain(right_dependencies.keys()).collect();
            for a in all_symbols {
                let empty = HashSet::new();
                let in_right: HashSet<_> = right_dependencies.get(&a.clone()).unwrap_or(&empty).iter().collect();
                let in_left: HashSet<_> = left_dependencies.get(&a.clone()).unwrap_or(&empty).iter().collect();
                for &b in in_left.difference(&in_right) {
                    left_not_right.entry(a.clone()).or_insert_with(HashSet::new).insert(b.clone());
                }
                for &b in in_right.difference(&in_left) {
                    right_not_left.entry(a.clone()).or_insert_with(HashSet::new).insert(b.clone());
                }
            }
            Ok((
                left_not_right.iter().map(|(k,v)|(k.clone(), v.clone().into_iter().collect())).collect(),
                right_not_left.iter().map(|(k,v)|(k.clone(), v.clone().into_iter().collect())).collect(),
            ))
        }
        Err(err) => Err(err),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage::<Mutex<StrixState<ArcStr>>>(Mutex::new(StrixState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_ontology, get_class_details, dependency_diff])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
