use agentx::core::error::AppExitCode;

fn main () -> AppExitCode {

    match agentx::cli::run() {
        Ok(()) => AppExitCode::SUCCESS,
        Err(error) => error.report(),
    }

}
