use std::future::Future;
use tokio::runtime::{Builder, Runtime};

use super::arch::{RUNTIME, Rt};

impl Rt {

    fn runtime () -> &'static Runtime {

        RUNTIME.get_or_init(|| {
            // Only fails on OS resource exhaustion — unrecoverable this early.
            Builder::new_multi_thread().enable_time().build().expect("failed to build tokio runtime")
        })

    }

    /// Drive an async future to completion from sync code.
    pub fn block_on<F: Future> ( future: F ) -> F::Output {

        Self::runtime().block_on(future)

    }

    /// Run many futures concurrently, results in input order. A task panic is
    /// re-raised on the caller.
    pub fn join_all<F> ( futures: Vec<F> ) -> Vec<F::Output> where F: Future + Send + 'static, F::Output: Send + 'static {

        Self::runtime().block_on(async {

            let handles: Vec<_> = futures.into_iter().map(tokio::spawn).collect();
            let mut out = Vec::with_capacity(handles.len());

            for handle in handles {

                match handle.await {
                    Ok(value) => out.push(value),
                    Err(error) => std::panic::resume_unwind(error.into_panic()),
                }
            }

            out

        })

    }

}
