use std::convert::TryFrom;

use matches::assert_matches;
use ruma_events::{AnyInitialStateEvent, InitialStateEvent};
use ruma_identifiers::RoomNameBox;
use serde_json::json;

#[test]
fn deserialize_initial_state_event() {
    assert_matches!(
        serde_json::from_value(json!({
            "type": "m.room.name",
            "content": { "name": "foo" }
        }))
        .unwrap(),
        AnyInitialStateEvent::RoomName(InitialStateEvent { content, state_key})
        if content.name == Some(RoomNameBox::try_from("foo").unwrap())
            && state_key.is_empty()
    );
}
