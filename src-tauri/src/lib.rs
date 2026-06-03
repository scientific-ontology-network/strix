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
use strix_roost::dependency::base::DependencyBuilder;
use strix_roost::dependency::growth::GrowthDependency;
use strix_roost::ontology::io::load_set_ontology;
use strix_roost::ontology::visitor::AxiomVisitor;
use strix_roost::util::error::StrixError;
use tauri::{Manager, State};

#[tauri::command]
fn load_ontology<'a>(
    raw_state: State<'a, Mutex<StrixState<ArcStr>>>,
    path: &str,
) -> Result<
    (
        OntologyView,
        HashMap<ArcStr, HashSet<ArcStr>>,
        HashSet<ArcStr>,
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
            let dependencies = GrowthDependency::build_dependencies(state.ontology.i().iter());
            let dependencies_without_cause: HashMap<ArcStr, HashSet<ArcStr>> = dependencies
                .iter()
                .map(|(k, vm)| {
                    (
                        k.underlying().clone(),
                        vm.keys()
                            .map(|k2| k2.underlying().clone())
                            .collect::<HashSet<_>>(),
                    )
                })
                .collect();
            state.dependencies = dependencies_without_cause;
            println!("Calculate dependency roots");
            let dependency_roots: HashSet<ArcStr> = find_roots(&state.dependencies);
            println!("Done!");
            Ok((
                ontology_view,
                state.dependencies.clone(),
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
) -> Result<(HashMap<ArcStr, HashSet<ArcStr>>,HashMap<ArcStr, HashSet<ArcStr>>), StrixError> {

    let state = state.lock().unwrap();
    println!("Load ontology");
    match load_set_ontology(path) {
        Ok(ontology) => {
            let mut left_not_right = HashMap::new();
            let mut right_not_left = HashMap::new();
            let right_symbol_dependencies = GrowthDependency::build_dependencies(ontology.i().iter());
            let right_dependencies: &HashMap<ArcStr, HashSet<ArcStr>> = &right_symbol_dependencies.into_iter().map(|(k, vm)| (k.underlying().clone(), vm.into_iter().map(|(k2,vn)| k.underlying().clone()).collect())).collect();
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
                left_not_right,
                right_not_left,
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
        .invoke_handler(tauri::generate_handler![load_ontology, get_class_details])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
