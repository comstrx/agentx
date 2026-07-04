use crate::config::Member;
use super::arch::Config;

impl Config {

    pub fn manager ( &self ) -> &str {

        &self.agent.manager.agent

    }

    pub fn roster ( &self, phase: &str ) -> Vec<String> {

        self.agent.roster(phase)

    }

    pub fn resolve_member ( &self, member: &Member ) -> ( String, String ) {

        member.resolved(&self.claude, &self.codex)

    }

    pub fn engine_of_key ( &self, key: &str ) -> ( String, String ) {

        self.resolve_member(&self.member_of_key(key))

    }

    fn member_of_key ( &self, key: &str ) -> Member {

        if key == "manager" { return self.agent.manager.clone(); }

        match key.split_once('-') {
            Some(( phase, seat )) => self.agent.member(phase, seat).unwrap_or_else(|| Member::backend(seat)),
            None => Member::backend(key),
        }

    }

}
