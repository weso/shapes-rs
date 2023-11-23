use std::str::FromStr;
use std::{fmt, result};

use serde::de::{MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{de, Deserialize, Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};
use srdf::lang::Lang;

use crate::IriRef;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LiteralExclusion {
    Literal(String),
    LiteralStem(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum IriExclusion {
    Iri(IriRef),
    IriStem(String),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum LanguageExclusion {
    Language(Lang),
    LanguageStem(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Exclusion {
    LiteralExclusion(LiteralExclusion),
    LanguageExclusion(LanguageExclusion),
    IriExclusion(IriExclusion),
}

impl Serialize for Exclusion {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Exclusion::IriExclusion(iri) => todo!(),
            Exclusion::LiteralExclusion(LiteralExclusion::Literal(lit)) => {
                todo!()
            }
            Exclusion::LiteralExclusion(LiteralExclusion::LiteralStem(stem)) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "LiteralStem")?;
                map.serialize_entry("stem", stem)?;
                map.end()
            }
            Exclusion::LanguageExclusion(_) => todo!(),
        }
    }
}

impl<'de> Deserialize<'de> for Exclusion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            Type,
            Stem,
        }

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("field of exclusion: `type` or `stem`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "type" => Ok(Field::Type),
                            "stem" => Ok(Field::Stem),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ExclusionVisitor;

        const FIELDS: &'static [&'static str] = &["type", "stem"];

        impl<'de> Visitor<'de> for ExclusionVisitor {
            type Value = Exclusion;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("Exclusion value")
            }

            /*fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                FromStr::from_str(s)
                    .map_err(|e| de::Error::custom(format!("Error parsing string `{s}`: {e}")))
            }*/

            fn visit_map<V>(self, mut map: V) -> Result<Exclusion, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut type_: Option<ExclusionType> = None;
                let mut stem: Option<String> = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if type_.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            let value: String = map.next_value()?;

                            let parsed_type_ =
                                ExclusionType::parse(&value.as_str()).map_err(|e| {
                                    de::Error::custom(format!(
                                        "Error parsing Exclusion type, found: {value}. Error: {e:?}"
                                    ))
                                })?;
                            type_ = Some(parsed_type_);
                        }
                        Field::Stem => {
                            if stem.is_some() {
                                return Err(de::Error::duplicate_field("stem"));
                            }
                            stem = Some(map.next_value()?);
                        }
                    }
                }
                match type_ {
                    Some(ExclusionType::LiteralStem) => match stem {
                        Some(stem) => Ok(Exclusion::LiteralExclusion(
                            LiteralExclusion::LiteralStem(stem),
                        )),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ExclusionType::LanguageStem) => match stem {
                        Some(stem) => Ok(Exclusion::LanguageExclusion(
                            LanguageExclusion::LanguageStem(stem),
                        )),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    Some(ExclusionType::IriStem) => match stem {
                        Some(stem) => Ok(Exclusion::IriExclusion(IriExclusion::IriStem(stem))),
                        None => Err(de::Error::missing_field("stem")),
                    },
                    None => todo!(),
                }
            }
        }

        deserializer.deserialize_any(ExclusionVisitor)
    }
}

#[derive(Debug, PartialEq)]
enum ExclusionType {
    IriStem,
    LiteralStem,
    LanguageStem,
}

#[derive(Debug)]
struct ExclusionTypeError {
    value: String,
}

impl ExclusionType {
    fn parse(s: &str) -> Result<ExclusionType, ExclusionTypeError> {
        match s {
            "IriStem" => Ok(ExclusionType::IriStem),
            "LanguageStem" => Ok(ExclusionType::LanguageStem),
            "LiteralStem" => Ok(ExclusionType::LiteralStem),
            _ => Err(ExclusionTypeError {
                value: s.to_string(),
            }),
        }
    }
}