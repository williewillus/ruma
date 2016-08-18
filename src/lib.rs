//! Crate ruma_events contains serializable types for the events in the [Matrix](https://matrix.org)
//! specification that can be shared by client and server code.

#![feature(custom_derive, plugin, question_mark)]
#![plugin(serde_macros)]
#![deny(missing_docs)]

extern crate ruma_identifiers;
extern crate serde;
extern crate serde_json;

use std::fmt::{Display, Formatter, Error as FmtError};

use ruma_identifiers::{EventId, RoomId, UserId};
use serde::{Deserialize, Deserializer, Error as SerdeError, Serialize, Serializer};
use serde::de::Visitor;
use serde_json::Value;

pub mod call;
pub mod presence;
pub mod receipt;
pub mod room;
pub mod stripped;
pub mod tag;
pub mod typing;

/// The type of an event.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EventType {
    /// m.call.answer
    CallAnswer,
    /// m.call.candidates
    CallCandidates,
    /// m.call.hangup
    CallHangup,
    /// m.call.invite
    CallInvite,
    /// m.presence
    Presence,
    /// m.receipt
    Receipt,
    /// m.room.aliases
    RoomAliases,
    /// m.room.avatar
    RoomAvatar,
    /// m.room.canonical_alias
    RoomCanonicalAlias,
    /// m.room.create
    RoomCreate,
    /// m.room.guest_access
    RoomGuestAccess,
    /// m.room.history_visibility
    RoomHistoryVisibility,
    /// m.room.join_rules
    RoomJoinRules,
    /// m.room.member
    RoomMember,
    /// m.room.message
    RoomMessage,
    /// m.room.name
    RoomName,
    /// m.room.power_levels
    RoomPowerLevels,
    /// m.room.redaction
    RoomRedaction,
    /// m.room.third_party_invite
    RoomThirdPartyInvite,
    /// m.room.topic
    RoomTopic,
    /// m.tag
    Tag,
    /// m.typing
    Typing,
    /// Any event that is not part of the specification.
    Custom(String),
}

/// A basic event.
#[derive(Debug, Deserialize, Serialize)]
pub struct Event<C, E> where C: Deserialize + Serialize, E: Deserialize + Serialize {
    /// Data specific to the event type.
    pub content: C,

    /// The type of the event.
    #[serde(rename="type")]
    pub event_type: EventType,

    /// Extra top-level key-value pairs specific to this event type, but that are not under the
    /// `content` field.
    pub extra_content: E,
}

/// An event within the context of a room.
#[derive(Debug, Deserialize, Serialize)]
pub struct RoomEvent<C, E> where C: Deserialize + Serialize, E: Deserialize + Serialize {
    /// Data specific to the event type.
    pub content: C,

    /// The unique identifier for the event.
    pub event_id: EventId,

    /// Extra top-level key-value pairs specific to this event type, but that are not under the
    /// `content` field.
    pub extra_content: E,

    /// The type of the event.
    #[serde(rename="type")]
    pub event_type: EventType,

    /// The unique identifier for the room associated with this event.
    pub room_id: RoomId,

    /// Additional key-value pairs not signed by the homeserver.
    #[serde(skip_serializing_if="Option::is_none")]
    pub unsigned: Option<Value>,

    /// The unique identifier for the user associated with this event.
    #[serde(rename="sender")]
    pub user_id: UserId,
}

/// An event that describes persistent state about a room.
#[derive(Debug, Deserialize, Serialize)]
pub struct StateEvent<C, E> where C: Deserialize + Serialize, E: Deserialize + Serialize {
    /// Data specific to the event type.
    pub content: C,

    /// The unique identifier for the event.
    pub event_id: EventId,

    /// The type of the event.
    #[serde(rename="type")]
    pub event_type: EventType,

    /// Extra top-level key-value pairs specific to this event type, but that are not under the
    /// `content` field.
    pub extra_content: E,

    /// The previous content for this state key, if any.
    #[serde(skip_serializing_if="Option::is_none")]
    pub prev_content: Option<C>,

    /// The unique identifier for the room associated with this event.
    pub room_id: RoomId,

    /// A key that determines which piece of room state the event represents.
    pub state_key: String,

    /// Additional key-value pairs not signed by the homeserver.
    #[serde(skip_serializing_if="Option::is_none")]
    pub unsigned: Option<Value>,

    /// The unique identifier for the user associated with this event.
    #[serde(rename="sender")]
    pub user_id: UserId,
}

/// A custom basic event not covered by the Matrix specification.
pub type CustomEvent = Event<Value, ()>;

/// A custom room event not covered by the Matrix specification.
pub type CustomRoomEvent = RoomEvent<Value, ()>;

/// A custom state event not covered by the Matrix specification.
pub type CustomStateEvent = StateEvent<Value, ()>;

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        let event_type_str = match *self {
            EventType::CallAnswer => "m.call.answer",
            EventType::CallCandidates => "m.call.candidates",
            EventType::CallHangup => "m.call.hangup",
            EventType::CallInvite => "m.call.invite",
            EventType::Presence => "m.presence",
            EventType::Receipt => "m.receipt",
            EventType::RoomAliases => "m.room.aliases",
            EventType::RoomAvatar => "m.room.avatar",
            EventType::RoomCanonicalAlias => "m.room.canonical_alias",
            EventType::RoomCreate => "m.room.create",
            EventType::RoomGuestAccess => "m.room.guest_access",
            EventType::RoomHistoryVisibility => "m.room.history_visibility",
            EventType::RoomJoinRules => "m.room.join_rules",
            EventType::RoomMember => "m.room.member",
            EventType::RoomMessage => "m.room.message",
            EventType::RoomName => "m.room.name",
            EventType::RoomPowerLevels => "m.room.power_levels",
            EventType::RoomRedaction => "m.room.redaction",
            EventType::RoomThirdPartyInvite => "m.room.third_party_invite",
            EventType::RoomTopic => "m.room.topic",
            EventType::Tag => "m.tag",
            EventType::Typing => "m.typing",
            EventType::Custom(ref event_type) => event_type,
        };

        write!(f, "{}", event_type_str)
    }
}

impl<'a> From<&'a str> for EventType {
    fn from(s: &'a str) -> EventType {
        match s {
            "m.call.answer" => EventType::CallAnswer,
            "m.call.candidates" => EventType::CallCandidates,
            "m.call.hangup" => EventType::CallHangup,
            "m.call.invite" => EventType::CallInvite,
            "m.presence" => EventType::Presence,
            "m.receipt" => EventType::Receipt,
            "m.room.aliases" => EventType::RoomAliases,
            "m.room.avatar" => EventType::RoomAvatar,
            "m.room.canonical_alias" => EventType::RoomCanonicalAlias,
            "m.room.create" => EventType::RoomCreate,
            "m.room.guest_access" => EventType::RoomGuestAccess,
            "m.room.history_visibility" => EventType::RoomHistoryVisibility,
            "m.room.join_rules" => EventType::RoomJoinRules,
            "m.room.member" => EventType::RoomMember,
            "m.room.message" => EventType::RoomMessage,
            "m.room.name" => EventType::RoomName,
            "m.room.power_levels" => EventType::RoomPowerLevels,
            "m.room.redaction" => EventType::RoomRedaction,
            "m.room.third_party_invite" => EventType::RoomThirdPartyInvite,
            "m.room.topic" => EventType::RoomTopic,
            "m.tag" => EventType::Tag,
            "m.typing" => EventType::Typing,
            event_type => EventType::Custom(event_type.to_string()),
        }
    }
}

impl Serialize for EventType {
    fn serialize<S>(&self, serializer: &mut S) -> Result<(), S::Error> where S: Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl Deserialize for EventType {
    fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error> where D: Deserializer {
        struct EventTypeVisitor;

        impl Visitor for EventTypeVisitor {
            type Value = EventType;

            fn visit_str<E>(&mut self, v: &str) -> Result<Self::Value, E> where E: SerdeError {
                Ok(EventType::from(v))
            }
        }

        deserializer.deserialize_str(EventTypeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_str, to_string};

    use super::EventType;

    #[test]
    fn event_types_serialize_to_display_form() {
        assert_eq!(
            to_string(&EventType::RoomCreate).unwrap(),
            r#""m.room.create""#
        );
    }

    #[test]
    fn custom_event_types_serialize_to_display_form() {
        assert_eq!(
            to_string(&EventType::Custom("io.ruma.test".to_string())).unwrap(),
            r#""io.ruma.test""#
        );
    }

    #[test]
    fn event_types_deserialize_from_display_form() {
        assert_eq!(
            from_str::<EventType>(r#""m.room.create""#).unwrap(),
            EventType::RoomCreate
        );
    }

    #[test]
    fn custom_event_types_deserialize_from_display_form() {
        assert_eq!(
            from_str::<EventType>(r#""io.ruma.test""#).unwrap(),
            EventType::Custom("io.ruma.test".to_string())
        )
    }
}
