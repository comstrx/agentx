use agentx::core::error::AppExitCode;

fn main () -> AppExitCode {

    match agentx::app::Cli::run() {
        Ok(()) => AppExitCode::SUCCESS,
        Err(error) => error.report(),
    }

}
