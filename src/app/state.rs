use std::collections::HashMap;

use crate::core::context::{AppContext, ContextValue};

/// Type-safe, namespaced views over the global [`AppContext`] for the live run:
/// per-agent sessions, init tracking, frozen tasks, and blocked steps. Sessions
/// are mirrored to `sessions.json` by the orchestrator.
pub struct State;

impl State {

    fn session_key ( agent: &str ) -> String {

        format!("session.{agent}")

    }

    fn primed_key ( agent: &str ) -> String {

        format!("primed.{agent}")

    }

    pub fn session ( agent: &str ) -> Option<String> {

        AppContext::get(Self::session_key(agent))

    }

    pub fn set_session ( agent: &str, id: &str ) {

        AppContext::set(Self::session_key(agent), id);

    }

    /// Seed sessions read from disk into the context.
    pub fn load_sessions ( sessions: HashMap<String, String> ) {

        for ( agent, id ) in sessions {
            AppContext::set(Self::session_key(&agent), id);
        }

    }

    /// Collect all live sessions for persistence.
    pub fn sessions () -> HashMap<String, String> {

        AppContext::with(|map| {

            map.iter().filter_map(|( key, value )| {
                let agent = key.strip_prefix("session.")?;
                value.as_str().map(|id| ( agent.to_string(), id.to_string() ))
            }).collect()

        })

    }

    /// Record the first turn for `agent`; returns true if this *was* the init turn.
    pub fn take_init ( agent: &str ) -> bool {

        let key = Self::primed_key(agent);
        let fresh = AppContext::get::<bool>(&key) != Some(true);
        AppContext::set(key, true);

        fresh

    }

    pub fn set_frozen ( names: Vec<String> ) {

        AppContext::set("run.frozen", ContextValue::list(names));

    }

    pub fn frozen () -> Vec<String> {

        AppContext::get("run.frozen").unwrap_or_default()

    }

    pub fn blocked () -> Vec<String> {

        AppContext::get("run.blocked").unwrap_or_default()

    }

    pub fn add_blocked ( step: &str ) {

        let mut blocked = Self::blocked();

        if !blocked.iter().any(|name| name == step) {
            blocked.push(step.to_string());
            AppContext::set("run.blocked", ContextValue::list(blocked));
        }

    }

}
