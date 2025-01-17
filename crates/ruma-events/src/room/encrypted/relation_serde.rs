use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "unstable-pre-spec")]
use super::{Annotation, Reference, Replacement};
use super::{InReplyTo, Relation};

impl<'de> Deserialize<'de> for Relation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        fn convert_relation(ev: EventWithRelatesToJsonRepr) -> Relation {
            if let Some(in_reply_to) = ev.relates_to.in_reply_to {
                return Relation::Reply { in_reply_to };
            }

            #[cfg(feature = "unstable-pre-spec")]
            if let Some(relation) = ev.relates_to.relation {
                return match relation {
                    RelationJsonRepr::Annotation(a) => Relation::Annotation(a),
                    RelationJsonRepr::Reference(r) => Relation::Reference(r),
                    RelationJsonRepr::Replacement(Replacement { event_id }) => {
                        Relation::Replacement(Replacement { event_id })
                    }
                    // FIXME: Maybe we should log this, though at this point we don't even have
                    // access to the rel_type of the unknown relation.
                    RelationJsonRepr::Unknown => Relation::_Custom,
                };
            }

            Relation::_Custom
        }

        EventWithRelatesToJsonRepr::deserialize(deserializer).map(convert_relation)
    }
}

impl Serialize for Relation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[allow(clippy::needless_update)]
        let relates_to = match self {
            #[cfg(feature = "unstable-pre-spec")]
            Relation::Annotation(r) => RelatesToJsonRepr {
                relation: Some(RelationJsonRepr::Annotation(r.clone())),
                ..Default::default()
            },
            #[cfg(feature = "unstable-pre-spec")]
            Relation::Reference(r) => RelatesToJsonRepr {
                relation: Some(RelationJsonRepr::Reference(r.clone())),
                ..Default::default()
            },
            #[cfg(feature = "unstable-pre-spec")]
            Relation::Replacement(r) => RelatesToJsonRepr {
                relation: Some(RelationJsonRepr::Replacement(r.clone())),
                ..Default::default()
            },
            Relation::Reply { in_reply_to } => {
                RelatesToJsonRepr { in_reply_to: Some(in_reply_to.clone()), ..Default::default() }
            }
            Relation::_Custom => RelatesToJsonRepr::default(),
        };

        EventWithRelatesToJsonRepr { relates_to }.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
struct EventWithRelatesToJsonRepr {
    #[serde(rename = "m.relates_to", default, skip_serializing_if = "RelatesToJsonRepr::is_empty")]
    relates_to: RelatesToJsonRepr,
}

/// Enum modeling the different ways relationships can be expressed in a `m.relates_to` field of an
/// event.
#[derive(Default, Deserialize, Serialize)]
struct RelatesToJsonRepr {
    #[serde(rename = "m.in_reply_to", skip_serializing_if = "Option::is_none")]
    in_reply_to: Option<InReplyTo>,

    #[cfg(feature = "unstable-pre-spec")]
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    relation: Option<RelationJsonRepr>,
}

impl RelatesToJsonRepr {
    fn is_empty(&self) -> bool {
        #[cfg(not(feature = "unstable-pre-spec"))]
        {
            self.in_reply_to.is_none()
        }

        #[cfg(feature = "unstable-pre-spec")]
        {
            self.in_reply_to.is_none() && self.relation.is_none()
        }
    }
}

/// A relation, which associates new information to an existing event.
#[derive(Clone, Deserialize, Serialize)]
#[cfg(feature = "unstable-pre-spec")]
#[serde(tag = "rel_type")]
enum RelationJsonRepr {
    /// An annotation to an event.
    #[serde(rename = "m.annotation")]
    Annotation(Annotation),

    /// A reference to another event.
    #[serde(rename = "m.reference")]
    Reference(Reference),

    /// An event that replaces another event.
    #[serde(rename = "m.replace")]
    Replacement(Replacement),

    /// An unknown relation type.
    ///
    /// Not available in the public API, but exists here so deserialization
    /// doesn't fail with new / custom `rel_type`s.
    #[serde(other)]
    Unknown,
}
