use crate::util::StrixError;
use horned_owl::io::{rdf, ParserConfiguration, RDFParserConfiguration};
use horned_owl::model::ObjectPropertyDomain;
use horned_owl::model::*;
use horned_owl::model::{
    AnnotationValue, ClassAssertion, DataPropertyDomain, DatatypeDefinition, Individual, SubClassOf, Literal
};
use horned_owl::ontology::set::SetOntology;
use semantic_dependency::dependency::base::OntologySymbol;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::hash::RandomState;
use std::io::BufReader;
use std::option::Option;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;
use crate::onto::rdf::RDFS_LABEL;
use crate::onto::serialize::*;

pub fn load(path: &str) -> SetOntology<ArcStr> {
    let ending = path.split(".").last().unwrap();
    let res = match ending {
        "owl" => load_rdf_ontology(&path),
        _ => Err(StrixError::InternalStrixError {
            message: format!("Unknown file ending: {}", ending),
        }),
    };
    match res {
        Ok(oc) => oc,
        Err(e) => panic!("Error loading ontology: {}", e),
    }
}

fn load_rdf_ontology(path: &str) -> Result<SetOntology<ArcStr>, StrixError> {
    let file = File::open(path)?;
    let reader = &mut BufReader::new(file);
    let build = Build::new_arc();
    let (ontology, _incomplete_parse) =
        rdf::reader::read_with_build::<ArcStr, ArcAnnotatedComponent, BufReader<File>>(
            reader,
            &build,
            ParserConfiguration {
                rdf: RDFParserConfiguration { lax: true },
                ..Default::default()
            },
        )?;

    Ok(ontology.into())
}







