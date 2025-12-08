// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod onto;

use std::collections::{HashMap, HashSet};
use horned_owl::model::{ArcStr, Build, ForIRI, Ontology, IRI};
use strix_roost::ontology::io::load_set_ontology;
use onto::owl::class::ClassDetails;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use horned_owl::ontology::set::SetOntology;
use strix_roost::dependency::base::{DependencyBuilder, DependencyMap, OntologySymbol};
use serde_json::{json, Value};
use tauri::{Manager, State};
use strix_roost::util::error::StrixError;
use crate::onto::owl::hierarchy::OntologyView;
use crate::onto::state::StrixState;
use crate::onto::serialize::OntologySymbolView;


#[tauri::command]
fn load_ontology<'a>(raw_state: State<'a, Mutex<StrixState<ArcStr>>>, path: &str) -> Result<(OntologyView, HashMap<ArcStr, HashSet<ArcStr>>, HashSet<ArcStr>), StrixError>{
    let mut state = raw_state.lock().unwrap();
    let start = SystemTime::now();
    state.set_ontology(load_set_ontology(path));
    println!("Ontology loaded in {:?}", start.elapsed().unwrap());
    let hier = state.get_hierarchy();

    Ok((hier, state.reduced_dependencies.clone(), state.dependency_roots.clone()))
}

#[tauri::command]
fn get_class_details<'a>(state: State<'a, Mutex<StrixState<ArcStr>>>, s: &str) -> Result<ClassDetails<ArcStr>, StrixError>{
    let mut state = state.lock().unwrap();
    let b_arc = Build::new_arc();
    let iri = b_arc.iri(s.to_string());
    Ok(state.get_class_details(&iri.underlying()))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage::<Mutex<StrixState<ArcStr>>>(
                Mutex::new(StrixState::default()),
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_ontology,
            get_class_details,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
