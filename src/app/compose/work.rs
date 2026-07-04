use std::path::Path as StdPath;

use crate::config::{Config, base::prompts as P};
use crate::app::Compose;

impl Compose {

    pub(crate) fn architect ( cfg: &Config, agent: &str, has_review: bool ) -> String {

        let mut parts = vec![P::REQUIRES_WORK.to_string()];

        if has_review { parts.push(P::REVIEW_HANDOFF.to_string()); }

        parts.push(P::OWNERSHIP.to_string());
        parts.push(P::WORK_DISCIPLINE.to_string());
        parts.push(P::WRITE_FENCE.to_string());
        parts.push(P::EVIDENCE.to_string());
        parts.push(P::REQUIRES_REPORT.to_string());

        let mut pairs = Self::values(cfg, "requires", agent, None);
        pairs.push(( "fence", Self::fence(cfg, "requires", agent) ));

        Self::render(&parts, &pairs)

    }

    pub(crate) fn executor ( cfg: &Config, agent: &str, task: &StdPath, gate_failed: bool, has_review: bool ) -> String {

        let mut parts = vec![P::TASKS_WORK.to_string()];

        if gate_failed { parts.push(P::TASKS_GATE_FAIL.to_string()); }

        if has_review { parts.push(P::REVIEW_HANDOFF.to_string()); }

        parts.push(P::TASKS_REMEDIATION.to_string());
        parts.push(Self::author_policy(cfg));
        parts.push(P::OWNERSHIP.to_string());
        parts.push(P::WORK_DISCIPLINE.to_string());
        parts.push(P::WRITE_FENCE.to_string());
        parts.push(P::EVIDENCE.to_string());
        parts.push(P::TASKS_REPORT.to_string());

        let mut pairs = Self::values(cfg, "tasks", agent, Some(task));
        pairs.push(( "fence", Self::fence(cfg, "tasks", agent) ));

        Self::render(&parts, &pairs)

    }

    pub(crate) fn auditor ( cfg: &Config, agent: &str, has_review: bool ) -> String {

        let mut parts = vec![P::AUDITS_WORK.to_string()];

        if has_review { parts.push(P::REVIEW_HANDOFF.to_string()); }

        parts.push(P::OWNERSHIP.to_string());
        parts.push(P::WORK_DISCIPLINE.to_string());
        parts.push(P::WRITE_FENCE.to_string());
        parts.push(P::EVIDENCE.to_string());
        parts.push(P::AUDITS_REPORT.to_string());

        let mut pairs = Self::values(cfg, "audits", agent, None);
        pairs.push(( "fence", Self::fence(cfg, "audits", agent) ));

        Self::render(&parts, &pairs)

    }

    pub(crate) fn producer ( cfg: &Config, phase: &str, agent: &str, shipped: &[String], gate_failed: bool, has_review: bool ) -> String {

        let mut parts = vec![P::PRODUCE_WORK.to_string()];

        if gate_failed { parts.push(P::PRODUCE_GATE_FAIL.to_string()); }

        if has_review { parts.push(P::REVIEW_HANDOFF.to_string()); }

        parts.push(P::OWNERSHIP.to_string());
        parts.push(P::WORK_DISCIPLINE.to_string());
        parts.push(P::WRITE_FENCE.to_string());
        parts.push(P::EVIDENCE.to_string());
        parts.push(P::PRODUCE_REPORT.to_string());

        let scope = match shipped.is_empty() {
            true => "(no task has shipped yet)".to_string(),
            false => shipped.join(", "),
        };

        let mut pairs = Self::values(cfg, phase, agent, None);
        pairs.push(( "phase", phase.to_string() ));
        pairs.push(( "duty", Self::duty_of(phase) ));
        pairs.push(( "scope", scope ));
        pairs.push(( "fence", Self::fence(cfg, phase, agent) ));

        Self::render(&parts, &pairs)

    }

}
