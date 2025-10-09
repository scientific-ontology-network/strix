
#![allow(dead_code)]
/// Standard RDF vocabulary IRIs
pub const RDF_TYPE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";
pub const RDF_PROPERTY: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Property";
pub const RDF_BAG: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Bag";
pub const RDF_SEQ: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Seq";
pub const RDF_ALT: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Alt";
pub const RDF_LIST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#List";
pub const RDF_FIRST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#first";
pub const RDF_REST: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#rest";
pub const RDF_NIL: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#nil";
pub const RDF_STATEMENT: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#Statement";
pub const RDF_SUBJECT: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#subject";
pub const RDF_PREDICATE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#predicate";
pub const RDF_OBJECT: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#object";
pub const RDF_VALUE: &str = "http://www.w3.org/1999/02/22-rdf-syntax-ns#value";

/// Standard RDFS vocabulary IRIs
pub const RDFS_CLASS: &str = "http://www.w3.org/2000/01/rdf-schema#Class";
pub const RDFS_RESOURCE: &str = "http://www.w3.org/2000/01/rdf-schema#Resource";
pub const RDFS_LITERAL: &str = "http://www.w3.org/2000/01/rdf-schema#Literal";
pub const RDFS_DATATYPE: &str = "http://www.w3.org/2000/01/rdf-schema#Datatype";
pub const RDFS_SUBCLASSOF: &str = "http://www.w3.org/2000/01/rdf-schema#subClassOf";
pub const RDFS_SUBPROPERTYOF: &str = "http://www.w3.org/2000/01/rdf-schema#subPropertyOf";
pub const RDFS_DOMAIN: &str = "http://www.w3.org/2000/01/rdf-schema#domain";
pub const RDFS_RANGE: &str = "http://www.w3.org/2000/01/rdf-schema#range";
pub const RDFS_LABEL: &str = "http://www.w3.org/2000/01/rdf-schema#label";
pub const RDFS_COMMENT: &str = "http://www.w3.org/2000/01/rdf-schema#comment";
pub const RDFS_MEMBER: &str = "http://www.w3.org/2000/01/rdf-schema#member";
pub const RDFS_CONTAINER: &str = "http://www.w3.org/2000/01/rdf-schema#Container";
pub const RDFS_CONTAINERMEMBERSHIPPROPERTY: &str = "http://www.w3.org/2000/01/rdf-schema#ContainerMembershipProperty";
pub const RDFS_SEEALSO: &str = "http://www.w3.org/2000/01/rdf-schema#seeAlso";
pub const RDFS_ISDEFINEDBY: &str = "http://www.w3.org/2000/01/rdf-schema#isDefinedBy";