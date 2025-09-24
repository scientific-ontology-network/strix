// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod onto;
mod util;

use horned_owl::model::{ArcStr, Build, Class, ClassExpression, IRI};
use onto::handler::OntologyContainer;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use tauri::{Manager, State};

#[tauri::command]
fn load_ontology(state: State<'_, Mutex<OntologyContainer>>, path: &str) {
    let mut state = state.lock().unwrap();
    state.load(path);
}

#[tauri::command]
fn get_ontology_structure(
    state: State<'_, Mutex<OntologyContainer>>,
) -> (
    HashMap<String, HashMap<String, HashSet<String>>>,
    HashSet<String>,
    HashSet<String>,
    HashMap<String, HashSet<String>>,
    HashMap<String, HashSet<String>>,
) {
    let state = state.lock().unwrap();
    (
        state.annotations.clone(),
        state.declared_classes.clone(),
        state.get_roots(),
        state.direct_subclasses.clone(),
        state.class_dependencies.clone(),
    )
}

#[tauri::command]
fn get_direct_subclasses(
    state: State<'_, Mutex<OntologyContainer>>,
    iri_rf: String,
) -> HashSet<String> {
    let state = state.lock().unwrap();
    state.direct_subclasses[&iri_rf].clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage::<Mutex<OntologyContainer>>(
                Mutex::new(OntologyContainer::default()),
            );
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_ontology,
            get_direct_subclasses,
            get_ontology_structure
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
