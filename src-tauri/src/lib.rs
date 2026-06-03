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
            println!("Calculate dependency roots");
            let dependency_roots: HashSet<ArcStr> = find_roots(&dependencies_without_cause);
            println!("Done!");
            Ok((
                ontology_view,
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage::<Mutex<StrixState<ArcStr>>>(Mutex::new(StrixState::default()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_ontology, get_class_details,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
