use crate::msg;
use crate::rdf::{expand_uri, Property, Subject, Value};
use cosmwasm_std::StdError;
use std::collections::HashMap;

impl TryFrom<(msg::Value, &HashMap<String, String>)> for Subject {
    type Error = StdError;

    fn try_from(
        (value, prefixes): (msg::Value, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: msg::IRI::Full(uri),
            } => Ok(Subject::NamedNode(uri)),
            msg::Value::URI {
                value: msg::IRI::Prefixed(curie),
            } => Ok(Subject::NamedNode(expand_uri(&curie, prefixes)?)),
            msg::Value::BlankNode { value: id } => Ok(Subject::BlankNode(id)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported subject value: {value:?}. Expected URI or BlankNode",
            ))),
        }
    }
}

impl TryFrom<(msg::Value, &HashMap<String, String>)> for Property {
    type Error = StdError;

    fn try_from(
        (value, prefixes): (msg::Value, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: msg::IRI::Full(uri),
            } => Ok(Property(uri)),
            msg::Value::URI {
                value: msg::IRI::Prefixed(curie),
            } => Ok(Property(expand_uri(&curie, prefixes)?)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported predicate value: {value:?}. Expected URI"
            ))),
        }
    }
}

impl TryFrom<(msg::Value, &HashMap<String, String>)> for Value {
    type Error = StdError;

    fn try_from(
        (value, prefixes): (msg::Value, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match value {
            msg::Value::URI {
                value: msg::IRI::Full(uri),
            } => Ok(Value::NamedNode(uri)),
            msg::Value::URI {
                value: msg::IRI::Prefixed(curie),
            } => Ok(Value::NamedNode(expand_uri(&curie, prefixes)?)),
            msg::Value::Literal {
                value,
                lang: None,
                datatype: None,
            } => Ok(Value::LiteralSimple(value)),
            msg::Value::Literal {
                value,
                lang: Some(lang),
                datatype: None,
            } => Ok(Value::LiteralLang(value, lang)),
            msg::Value::Literal {
                value,
                lang: None,
                datatype: Some(msg::IRI::Full(uri)),
            } => Ok(Value::LiteralDatatype(value, uri)),
            msg::Value::Literal {
                value,
                lang: None,
                datatype: Some(msg::IRI::Prefixed(curie)),
            } => Ok(Value::LiteralDatatype(value, expand_uri(&curie, prefixes)?)),
            msg::Value::BlankNode { value } => Ok(Value::BlankNode(value)),
            _ => Err(StdError::generic_err(format!(
                "Unsupported object value: {value:?}. Expected URI, BlankNode or Literal"
            )))?,
        }
    }
}

impl TryFrom<(msg::Node, &HashMap<String, String>)> for Subject {
    type Error = StdError;

    fn try_from(
        (node, prefixes): (msg::Node, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match node {
            msg::Node::BlankNode(id) => Ok(Subject::BlankNode(id)),
            msg::Node::NamedNode(msg::IRI::Full(uri)) => Ok(Subject::NamedNode(uri)),
            msg::Node::NamedNode(msg::IRI::Prefixed(curie)) => {
                Ok(Subject::NamedNode(expand_uri(&curie, prefixes)?))
            }
        }
    }
}

impl TryFrom<(msg::Node, &HashMap<String, String>)> for Property {
    type Error = StdError;

    fn try_from(
        (node, prefixes): (msg::Node, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match node {
            msg::Node::NamedNode(msg::IRI::Full(uri)) => Ok(Property(uri)),
            msg::Node::NamedNode(msg::IRI::Prefixed(curie)) => {
                Ok(Property(expand_uri(&curie, prefixes)?))
            }
            _ => Err(StdError::generic_err(format!(
                "Unsupported predicate node: {node:?}. Expected URI"
            ))),
        }
    }
}

impl TryFrom<(msg::Node, &HashMap<String, String>)> for Value {
    type Error = StdError;

    fn try_from(
        (node, prefixes): (msg::Node, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match node {
            msg::Node::NamedNode(msg::IRI::Full(uri)) => Ok(Value::NamedNode(uri)),
            msg::Node::NamedNode(msg::IRI::Prefixed(curie)) => {
                Ok(Value::NamedNode(expand_uri(&curie, prefixes)?))
            }
            msg::Node::BlankNode(id) => Ok(Value::BlankNode(id)),
        }
    }
}

impl TryFrom<(msg::Literal, &HashMap<String, String>)> for Value {
    type Error = StdError;

    fn try_from(
        (literal, prefixes): (msg::Literal, &HashMap<String, String>),
    ) -> Result<Self, Self::Error> {
        match literal {
            msg::Literal::Simple(value) => Ok(Value::LiteralSimple(value)),
            msg::Literal::LanguageTaggedString { value, language } => {
                Ok(Value::LiteralLang(value, language))
            }
            msg::Literal::TypedValue {
                value,
                datatype: msg::IRI::Full(uri),
            } => Ok(Value::LiteralDatatype(value, uri)),
            msg::Literal::TypedValue {
                value,
                datatype: msg::IRI::Prefixed(prefix),
            } => Ok(Value::LiteralDatatype(
                value,
                expand_uri(&prefix, prefixes)?,
            )),
        }
    }
}

#[derive(Default)]
pub struct PrefixMap(HashMap<String, String>);
impl PrefixMap {
    pub fn into_inner(self) -> HashMap<String, String> {
        self.0
    }
}

impl From<Vec<msg::Prefix>> for PrefixMap {
    fn from(as_list: Vec<msg::Prefix>) -> Self {
        PrefixMap(
            as_list
                .into_iter()
                .map(|prefix| (prefix.prefix, prefix.namespace))
                .collect(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_subject() {
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    ),
                },
                &PrefixMap::default().into_inner(),
            )
                .try_into(),
            Ok(Subject::NamedNode(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ))
        );
        assert_eq!(
            (
                msg::Value::BlankNode {
                    value: "blank".to_string(),
                },
                &PrefixMap::default().into_inner(),
            )
                .try_into(),
            Ok(Subject::BlankNode("blank".to_string()))
        );
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Prefixed("rdf:".to_string()),
                },
                &<PrefixMap>::from(vec![msg::Prefix {
                    prefix: "rdf".to_string(),
                    namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                }])
                .into_inner(),
            )
                .try_into(),
            Ok(Subject::NamedNode(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ))
        );
        assert_eq!(
            Subject::try_from((
                msg::Value::Literal {
                    value: "rdf".to_string(),
                    lang: None,
                    datatype: None,
                },
                 &PrefixMap::default().into_inner(),
            )),
            Err(StdError::generic_err(
                "Unsupported subject value: Literal { value: \"rdf\", lang: None, datatype: None }. Expected URI or BlankNode"
            ))
        );
    }

    #[test]
    fn try_from_property() {
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    ),
                },
                &PrefixMap::default().into_inner(),
            )
                .try_into(),
            Ok(Property(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
            ))
        );
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Prefixed("rdf:".to_string()),
                },
                &<PrefixMap>::from(vec![msg::Prefix {
                    prefix: "rdf".to_string(),
                    namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                }])
                .into_inner(),
            )
                .try_into(),
            Ok(Property(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ))
        );
        assert_eq!(
            Property::try_from((
                msg::Value::BlankNode {
                    value: "blank".to_string(),
                },
                &PrefixMap::default().into_inner(),
            )),
            Err(StdError::generic_err(
                "Unsupported predicate value: BlankNode { value: \"blank\" }. Expected URI"
            ))
        );
    }

    #[test]
    fn try_from_value() {
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    ),
                },
                &PrefixMap::default().into_inner()
            )
                .try_into(),
            Ok(Value::NamedNode(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
            ))
        );
        assert_eq!(
            (
                msg::Value::URI {
                    value: msg::IRI::Prefixed("rdf:".to_string()),
                },
                &<PrefixMap>::from(vec![msg::Prefix {
                    prefix: "rdf".to_string(),
                    namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                }])
                .into_inner(),
            )
                .try_into(),
            Ok(Value::NamedNode(
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
            ))
        );
        assert_eq!(
            (
                msg::Value::Literal {
                    value: "foo".to_string(),
                    lang: None,
                    datatype: None,
                },
                &PrefixMap::default().into_inner(),
            )
                .try_into(),
            Ok(Value::LiteralSimple("foo".to_string()))
        );
        assert_eq!(
            (
                msg::Value::Literal {
                    value: "foo".to_string(),
                    lang: Some("en".to_string()),
                    datatype: None,
                },
                &PrefixMap::default().into_inner()
            )
                .try_into(),
            Ok(Value::LiteralLang("foo".to_string(), "en".to_string()))
        );
        assert_eq!(
            (
                msg::Value::Literal {
                    value: "foo".to_string(),
                    lang: None,
                    datatype: Some(msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    )),
                },
                &PrefixMap::default().into_inner(),
            )
                .try_into(),
            Ok(Value::LiteralDatatype(
                "foo".to_string(),
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
            ))
        );
        assert_eq!(
            (
                msg::Value::Literal {
                    value: "foo".to_string(),
                    lang: None,
                    datatype: Some(msg::IRI::Prefixed("rdf:".to_string())),
                },
                &<PrefixMap>::from(vec![msg::Prefix {
                    prefix: "rdf".to_string(),
                    namespace: "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string(),
                }])
                .into_inner(),
            )
                .try_into(),
            Ok(Value::LiteralDatatype(
                "foo".to_string(),
                "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
            ))
        );
        assert_eq!(
            (
                msg::Value::BlankNode {
                    value: "foo".to_string()
                },
                &PrefixMap::default().into_inner(),
            )
                .try_into(),
            Ok(Value::BlankNode("foo".to_string()))
        );
        assert_eq!(
            Value::try_from((
                msg::Value::Literal {
                    value: "blank".to_string(),
                    lang: Some("en".to_string()),
                    datatype: Some(msg::IRI::Full(
                        "http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string()
                    )),
                },
                &PrefixMap::default().into_inner(),
            )),
            Err(StdError::generic_err(
                "Unsupported object value: Literal { value: \"blank\", lang: Some(\"en\"), datatype: Some(Full(\"http://www.w3.org/1999/02/22-rdf-syntax-ns#\")) }. Expected URI, BlankNode or Literal"
            ))
        );
    }
}
