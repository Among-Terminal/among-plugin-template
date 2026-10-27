//! A minimal Among managed plugin.
//!
//! Among starts the executable named by `entrypoint` in
//! `plugins/hello-status/among-plugin.json` and talks to it with one JSON
//! object per line on stdin and stdout. The plugin must answer the host's
//! `managed_hello` with the same identity, session and generation before
//! anything else counts, then answers each `status` request with fragments
//! until `shutdown`. Replace `fragments` with what your plugin shows.

use serde_json::{json, Value};
use std::io::{self, BufRead, Write};

/// The features this plugin uses. They must be offered by the host and listed
/// in the manifest's `required_features`.
const FEATURES: [&str; 1] = ["status"];

/// The fragments shown for one status request. `scope.slot` says where they
/// appear (for example `app_footer` or `pane_header`).
fn fragments(_scope: &Value) -> Value {
    json!([{
        "text": "hello",
        "style": "accent",
        "tooltip": "Edit src/main.rs to show your own status",
    }])
}

/// The admitted session and generation, once the handshake succeeded.
#[derive(Default)]
struct State {
    admitted: Option<(Value, Value)>,
}

enum Reply {
    Send(Value),
    Ignore,
    Stop,
}

impl State {
    fn handle(&mut self, message: &Value) -> Reply {
        match message["type"].as_str() {
            Some("managed_hello") => self.hello(message),
            Some("status") => self.status(message),
            Some("shutdown") => Reply::Stop,
            // Messages for features this plugin did not ask for, or newer
            // ones it does not know, need no answer.
            _ => Reply::Ignore,
        }
    }

    fn hello(&mut self, message: &Value) -> Reply {
        let offered = message["features"]
            .as_array()
            .map(|features| {
                features
                    .iter()
                    .filter_map(Value::as_str)
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if message["major"] != 1
            || message["minor"].as_u64().is_none_or(|minor| minor < 2)
            || !FEATURES.iter().all(|feature| offered.contains(feature))
            || !message["session"].is_string()
            || !message["generation"].is_string()
        {
            // Without a matching reply the host does not admit the plugin.
            return Reply::Ignore;
        }
        self.admitted = Some((message["session"].clone(), message["generation"].clone()));
        Reply::Send(json!({
            "type": "managed_hello",
            "major": 1,
            "minor": 2,
            "identity": message["identity"],
            "session": message["session"],
            "generation": message["generation"],
            "features": FEATURES,
        }))
    }

    fn status(&self, message: &Value) -> Reply {
        match &self.admitted {
            Some((session, generation))
                if &message["session"] == session && &message["generation"] == generation =>
            {
                Reply::Send(json!({
                    "type": "status",
                    "session": session,
                    "generation": generation,
                    "request_id": message["request_id"],
                    "fragments": fragments(&message["scope"]),
                    "refresh_ms": 60_000,
                }))
            }
            // A request from before admission or from another generation
            // is not ours to answer.
            _ => Reply::Ignore,
        }
    }
}

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout().lock();
    let mut state = State::default();
    for line in stdin.lock().lines() {
        let Ok(message) = serde_json::from_str::<Value>(&line?) else {
            continue;
        };
        match state.handle(&message) {
            Reply::Send(reply) => {
                serde_json::to_writer(&mut stdout, &reply)?;
                stdout.write_all(b"\n")?;
                // The host reads replies as they come; buffering would stall it.
                stdout.flush()?;
            }
            Reply::Ignore => {}
            Reply::Stop => break,
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hello() -> Value {
        json!({"type":"managed_hello","major":1,"minor":2,"identity":"github:1/hello-status",
            "session":"s","generation":"g","features":["context.terminals","status"]})
    }

    #[test]
    fn the_handshake_echoes_identity_session_and_generation() {
        let mut state = State::default();
        let Reply::Send(reply) = state.handle(&hello()) else {
            panic!("no handshake reply");
        };
        assert_eq!(
            reply,
            json!({"type":"managed_hello","major":1,"minor":2,"identity":"github:1/hello-status",
                "session":"s","generation":"g","features":["status"]})
        );
    }

    #[test]
    fn status_is_answered_only_after_admission_and_for_this_generation() {
        let mut state = State::default();
        let request = json!({"type":"status","session":"s","generation":"g","request_id":7,
            "scope":{"slot":"app_footer","target":{"kind":"application"}}});
        assert!(matches!(state.handle(&request), Reply::Ignore));
        state.handle(&hello());
        let Reply::Send(reply) = state.handle(&request) else {
            panic!("no status reply");
        };
        assert_eq!(reply["request_id"], 7);
        assert_eq!(reply["fragments"][0]["text"], "hello");
        let other = json!({"type":"status","session":"s","generation":"old","request_id":8});
        assert!(matches!(state.handle(&other), Reply::Ignore));
    }

    #[test]
    fn a_host_without_the_required_feature_is_not_answered() {
        let mut state = State::default();
        let mut offer = hello();
        offer["features"] = json!(["context.terminals"]);
        assert!(matches!(state.handle(&offer), Reply::Ignore));
        let mut old = hello();
        old["minor"] = json!(1);
        assert!(matches!(state.handle(&old), Reply::Ignore));
    }

    #[test]
    fn shutdown_stops_the_plugin() {
        assert!(matches!(
            State::default().handle(&json!({"type":"shutdown"})),
            Reply::Stop
        ));
    }

    #[test]
    fn the_manifest_requires_exactly_the_features_the_code_uses() {
        let manifest: Value =
            serde_json::from_str(include_str!("../plugins/hello-status/among-plugin.json"))
                .unwrap();
        assert_eq!(manifest["required_features"], json!(FEATURES));
        assert_eq!(manifest["entrypoint"], "bin/hello-status");
    }
}
