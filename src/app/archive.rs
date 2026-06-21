use crate::config::Paths;
use crate::core::error::AppResult;
use crate::core::support::fs::{Dir, File, Path};

/// Snapshot the whole run into `.agentx/runs/<stamp>/` for the record.
pub fn run ( paths: &Paths ) -> AppResult<()> {

    let run_dir = paths.runs.join(Dir::next_stamp(&paths.runs));
    Dir::ensure(&run_dir)?;

    for ( source, label ) in [
        ( &paths.requires, "requires" ),
        ( &paths.tasks, "tasks" ),
        ( &paths.reports, "reports" ),
        ( &paths.rounds, "rounds" ),
        ( &paths.tests, "tests" ),
        ( &paths.probes, "probes" ),
        ( &paths.prompts, "prompts" ),
    ] {
        Dir::copy_tree(source, &run_dir.join(label))?;
    }

    for single in [&paths.review, &paths.control, &paths.gate_log] {

        if single.exists() {
            File::copy(single, &run_dir.join(Path::name_of(single)))?;
        }
    }

    Ok(())

}
