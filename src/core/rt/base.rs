use std::{future::Future, time::Duration};
use tokio::runtime::{Builder, Runtime};
use tokio::task::JoinHandle;

use super::arch::{RUNTIME, Rt};

impl Rt {

    fn runtime () -> &'static Runtime {

        RUNTIME.get_or_init(|| Builder::new_multi_thread().enable_all().build().expect("failed to build tokio runtime"))

    }

    pub fn block_on <F: Future> ( future: F ) -> F::Output {

        Self::runtime().block_on(future)

    }

    pub fn spawn_blocking <F, T> ( task: F ) -> JoinHandle<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {

        Self::runtime().spawn_blocking(task)

    }

    pub fn timeout <F: Future> ( secs: u64, future: F ) -> Option<F::Output> {

        Self::block_on(async move {

            tokio::time::timeout(Duration::from_secs(secs), future).await.ok()

        })

    }

    pub fn join_all <F> ( futures: Vec<F> ) -> Vec<F::Output> where F: Future + Send + 'static, F::Output: Send + 'static {

        Self::block_on(async {

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
