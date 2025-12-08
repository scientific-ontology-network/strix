use horned_owl::model::*;
use strix_roost::dependency::base::OntologySymbol;
use serde::Serialize;
use ts_rs::TS;

#[derive(Serialize, Eq, PartialEq, Clone, Hash, Debug)]
#[derive(TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ClassExpressionView {
    Class {
        iri: String,
    },
    ObjectSomeValuesFrom {
        property: ObjectPropertyExpressionView,
        class_expression: Box<ClassExpressionView>,
    },
    ObjectAllValuesFrom {
        property: ObjectPropertyExpressionView,
        class_expression: Box<ClassExpressionView>,
    },
    ObjectIntersectionOf {
        operands: Vec<ClassExpressionView>,
    },
    ObjectUnionOf {
        operands: Vec<ClassExpressionView>,
    },
    ObjectComplementOf {
        class_expression: Box<ClassExpressionView>,
    },
    ObjectOneOf {
        individuals: Vec<IndividualView>,
    },
    ObjectHasValue {
        property: ObjectPropertyExpressionView,
        individual: IndividualView,
    },
    ObjectHasSelf {
        property: ObjectPropertyExpressionView,
    },
    ObjectMinCardinality {
        cardinality: u32,
        property: ObjectPropertyExpressionView,
        class_expression: Box<ClassExpressionView>,
    },
    ObjectMaxCardinality {
        cardinality: u32,
        property: ObjectPropertyExpressionView,
        class_expression: Box<ClassExpressionView>,
    },
    ObjectExactCardinality {
        cardinality: u32,
        property: ObjectPropertyExpressionView,
        class_expression: Box<ClassExpressionView>,
    },
    DataSomeValuesFrom {
        property: String,
        data_range: DataRangeView,
    },
    DataAllValuesFrom {
        property: String,
        data_range: DataRangeView,
    },
    DataHasValue {
        property: String,
        value: LiteralView,
    },
    DataMinCardinality {
        cardinality: u32,
        property: String,
        data_range: DataRangeView,
    },
    DataMaxCardinality {
        cardinality: u32,
        property: String,
        data_range: DataRangeView,
    },
    DataExactCardinality {
        cardinality: u32,
        property: String,
        data_range: DataRangeView,
    },
}
#[derive(Serialize, Eq, PartialEq, Clone, Hash, Debug)]
#[derive(TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ObjectPropertyExpressionView {
    ObjectProperty {
        iri: String,
    },
    InverseObjectProperty {
        property: String,
    },
}

#[derive(Serialize, Eq, PartialEq, Clone, Hash, Debug)]
#[derive(TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum DataPropertyView {
    DataProperty {
        iri: String,
    },
}

#[derive(Serialize, Debug, Eq, PartialEq, Clone, Hash)]
#[derive(TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum LiteralView {
    Simple {
        value: String,
    },
    Language {
        value: String,
        language: String,
    },
    Datatype {
        value: String,
        datatype: String,
    },
}

#[derive(Serialize)]
#[derive(TS)]
#[ts(export)]
pub struct AnnotationView {
    property: String,
    value: AnnotationValueView,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Hash)]
#[derive(TS)]
#[ts(export)]
#[serde(untagged)]
pub enum AnnotationValueView {
    Literal(LiteralView),
    IRI(String),
    AnonymousIndividual(String),
}

// Conversion implementations
impl<T: ForIRI> From<&ClassExpression<T>> for ClassExpressionView {
    fn from(expr: &ClassExpression<T>) -> Self {
        match expr {
            ClassExpression::Class(c) => ClassExpressionView::Class {
                iri: c.0.to_string(),
            },
            ClassExpression::ObjectSomeValuesFrom { ope, bce } => ClassExpressionView::ObjectSomeValuesFrom {
                property: ope.into(),
                class_expression: Box::new((&**bce).into()),
            },
            ClassExpression::ObjectAllValuesFrom { ope, bce } => ClassExpressionView::ObjectAllValuesFrom {
                property: ope.into(),
                class_expression: Box::new((&**bce).into()),
            },
            ClassExpression::ObjectIntersectionOf(classes) => ClassExpressionView::ObjectIntersectionOf {
                operands: classes.iter().map(Into::into).collect(),
            },
            ClassExpression::ObjectUnionOf(classes) => ClassExpressionView::ObjectUnionOf {
                operands: classes.iter().map(Into::into).collect(),
            },
            ClassExpression::ObjectComplementOf(class) => ClassExpressionView::ObjectComplementOf {
                class_expression: Box::new((&**class).into()),
            },
            ClassExpression::ObjectOneOf(individuals) => ClassExpressionView::ObjectOneOf {
                individuals: individuals.iter().map(Into::into).collect(),
            },
            ClassExpression::ObjectHasValue { ope, i } => ClassExpressionView::ObjectHasValue {
                property: ope.into(),
                individual: i.into(),
            },
            ClassExpression::ObjectHasSelf(ope) => ClassExpressionView::ObjectHasSelf {
                property: ope.into(),
            },
            ClassExpression::ObjectMinCardinality { n, ope, bce } => ClassExpressionView::ObjectMinCardinality {
                cardinality: *n,
                property: ope.into(),
                class_expression: Box::new((&**bce).into()),
            },
            ClassExpression::ObjectMaxCardinality { n, ope, bce } => ClassExpressionView::ObjectMaxCardinality {
                cardinality: *n,
                property: ope.into(),
                class_expression: Box::new((&**bce).into()),
            },
            ClassExpression::ObjectExactCardinality { n, ope, bce } => ClassExpressionView::ObjectExactCardinality {
                cardinality: *n,
                property: ope.into(),
                class_expression: Box::new((&**bce).into()),
            },
            ClassExpression::DataSomeValuesFrom { dp, dr } => ClassExpressionView::DataSomeValuesFrom {
                property: dp.0.to_string(),
                data_range: dr.into(),
            },
            ClassExpression::DataAllValuesFrom { dp, dr } => ClassExpressionView::DataAllValuesFrom {
                property: dp.0.to_string(),
                data_range: dr.into(),
            },
            ClassExpression::DataHasValue { dp, l } => ClassExpressionView::DataHasValue {
                property: dp.0.to_string(),
                value: l.into(),
            },
            ClassExpression::DataMinCardinality { n, dp, dr } => ClassExpressionView::DataMinCardinality {
                cardinality: *n,
                property: dp.0.to_string(),
                data_range: dr.into(),
            },
            ClassExpression::DataMaxCardinality { n, dp, dr } => ClassExpressionView::DataMaxCardinality {
                cardinality: *n,
                property: dp.0.to_string(),
                data_range: dr.into(),
            },
            ClassExpression::DataExactCardinality { n, dp, dr } => ClassExpressionView::DataExactCardinality {
                cardinality: *n,
                property: dp.0.to_string(),
                data_range: dr.into(),
            },
        }
    }
}

impl<T: ForIRI> From<&ObjectPropertyExpression<T>> for ObjectPropertyExpressionView {
    fn from(expr: &ObjectPropertyExpression<T>) -> Self {
        match expr {
            ObjectPropertyExpression::ObjectProperty(p) => ObjectPropertyExpressionView::ObjectProperty {
                iri: p.0.to_string(),
            },
            ObjectPropertyExpression::InverseObjectProperty(p) => ObjectPropertyExpressionView::InverseObjectProperty {
                property: p.0.to_string(),
            },
        }
    }
}

impl<'a, T: ForIRI> From<&'a Literal<T>> for LiteralView {
    fn from(literal: &'a Literal<T>) -> Self {
        match literal {
            Literal::Simple { literal } => LiteralView::Simple {
                value: literal.to_string(),
            },
            Literal::Language { literal, lang } => LiteralView::Language {
                value: literal.to_string(),
                language: lang.to_string(),
            },
            Literal::Datatype { literal, datatype_iri } => LiteralView::Datatype {
                value: literal.to_string(),
                datatype: datatype_iri.to_string(),
            },
        }
    }
}

impl<'a, T: ForIRI> From<&'a Annotation<T>> for AnnotationView {
    fn from(annotation: &'a Annotation<T>) -> Self {
        AnnotationView {
            property: annotation.ap.0.to_string(),
            value: AnnotationValueView::from(&annotation.av),
        }
    }
}

impl<'a, T: ForIRI> From<&'a AnnotationValue<T>> for AnnotationValueView {
    fn from(value: &'a AnnotationValue<T>) -> Self {
        match value {
            AnnotationValue::Literal(literal) => AnnotationValueView::Literal(literal.into()),
            AnnotationValue::IRI(iri) => AnnotationValueView::IRI(iri.to_string()),
            AnnotationValue::AnonymousIndividual(id) => AnnotationValueView::AnonymousIndividual(id.to_string()),
        }
    }
}


#[derive(Serialize, Eq, PartialEq, Clone, Hash, Debug)]
#[derive(TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum IndividualView {
    Named {
        iri: String,
    },
    Anonymous {
        id: String,
    }
}

impl<T: ForIRI> From<&Individual<T>> for IndividualView {
    fn from(individual: &Individual<T>) -> Self {
        match individual {
            Individual::Named(iri) => IndividualView::Named {
                iri: iri.0.to_string(),
            },
            Individual::Anonymous(id) => IndividualView::Anonymous {
                id: id.to_string(),
            },
        }
    }
}

#[derive(Serialize, Hash, Eq, PartialEq, Clone, Debug)]
#[derive(TS)]
#[ts(export)]
#[serde(tag = "symbol_type")]
pub enum OntologySymbolView {
    CE (ClassExpressionView),
    Role(ObjectPropertyExpressionView)
}

impl<'a, T: ForIRI> From<&OntologySymbol<'a, T>> for OntologySymbolView {
    fn from(symbol: &OntologySymbol<T>) -> Self {
        match symbol {
            OntologySymbol::CE(c) => OntologySymbolView::CE(ClassExpressionView::from(*c)),
            OntologySymbol::Role(p) => OntologySymbolView::Role(ObjectPropertyExpressionView::from(*p))
        }
    }
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq, Hash)]
#[derive(TS)]
#[ts(export)]
#[serde(tag = "sop_type", content = "content")]
pub enum SubObjectPropertyExpressionView {
    ObjectPropertyExpression(ObjectPropertyExpressionView),
    ObjectPropertyChain (Vec<ObjectPropertyExpressionView>),
}

impl<T: ForIRI> From<&SubObjectPropertyExpression<T>> for SubObjectPropertyExpressionView {
    fn from(expr: &SubObjectPropertyExpression<T>) -> Self {
        match expr {
            SubObjectPropertyExpression::ObjectPropertyExpression(ope)=> SubObjectPropertyExpressionView::ObjectPropertyExpression(ObjectPropertyExpressionView::from(ope)),
            SubObjectPropertyExpression::ObjectPropertyChain(chain) =>
                SubObjectPropertyExpressionView::ObjectPropertyChain(chain.iter().map(Into::into).collect())
        }
    }
}

#[derive(Serialize, Eq, PartialEq, Clone, Hash, Debug)]
#[derive(TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum DataRangeView {
    Datatype {
        iri: String,
    },
    DataIntersectionOf {
        operands: Vec<DataRangeView>,
    },
    DataUnionOf {
        operands: Vec<DataRangeView>,
    },
    DataComplementOf {
        operand: Box<DataRangeView>,
    },
    Unsupported,
}

impl<T: ForIRI> From<&DataRange<T>> for DataRangeView {
    fn from(range: &DataRange<T>) -> Self {
        match range {
            DataRange::Datatype(dt) => DataRangeView::Datatype {
                iri: dt.0.to_string(),
            },
            DataRange::DataIntersectionOf(ranges) => DataRangeView::DataIntersectionOf {
                operands: ranges.iter().map(Into::into).collect(),
            },
            DataRange::DataUnionOf(ranges) => DataRangeView::DataUnionOf {
                operands: ranges.iter().map(Into::into).collect(),
            },
            DataRange::DataComplementOf(range) => DataRangeView::DataComplementOf {
                operand: Box::new((&**range).into()),
            },
            _ => DataRangeView::Unsupported,
        }
    }
}



