// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod onto;

use horned_owl::model::{ArcStr};
use strix_roost::ontology::io::load_set_ontology;
use onto::owl::ontology::OntologyContainer;
use onto::owl::class::ClassDetails;
use std::collections::{HashMap, HashSet};
use std::fmt::Error;
use std::sync::Mutex;
use strix_roost::dependency::base::{DependencyBuilder, OntologySymbol};
use serde_json::{json, Value};
use tauri::{Manager, State};
use strix_roost::util::error::StrixError;

use strix_roost::dependency::growth::GrowthDependency;

#[tauri::command]
fn load_ontology(state: State<'_, Mutex<OntologyContainer<ArcStr>>>, path: &str) -> (
    Vec<String>,
    HashMap<String, HashSet<String>>,
    HashMap<String, String>,
) {
    let mut state = state.lock().unwrap();
    println!("Loading ontology from {} ...", path);
    let o = load_set_ontology(path);
    println!("done");
    println!("Processing components...");
    for c in o.i() {
        state.handle_component(c);
    }
    println!("done");
    println!("Calculating dependencies...");
    let dependencies = GrowthDependency::build_dependencies(o.i().into_iter());
    let cleaned_dependencies = GrowthDependency::remove_supers(dependencies, o.i().into_iter());
    state.digest_dependencies(cleaned_dependencies);
    println!("done");
    let subclass_map = state.calculate_class_hierarchy();
    let res = (
        OntologyContainer::<ArcStr>::calculate_roots_classes(subclass_map),
        state.calculate_class_hierarchy(),
        state.calculate_label_map()
    );
    println!("done");
    res
}

#[tauri::command]
fn get_class_details(state: State<'_, Mutex<OntologyContainer<ArcStr>>>, iri: &str) -> Result<ClassDetails<ArcStr>, StrixError>{
    let mut state = state.lock().unwrap();
    match state.class_details.get(iri) {
        None => Err(StrixError::InternalStrixError {message:String::from("Class not found")}),
        Some(cd) => Ok(cd.clone())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            app.manage::<Mutex<OntologyContainer<ArcStr>>>(
                Mutex::new(OntologyContainer::default()),
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
