use crate::core::rt::Rt;
use super::arch::{Group, Task, Thread};

impl Thread {

    pub fn start <F, T> ( task: F ) -> Task<T> where F: FnOnce() -> T + Send + 'static, T: Send + 'static {

        Task { handle: Rt::spawn_blocking(task) }

    }

    pub fn gather <T> ( tasks: Vec<Box<dyn FnOnce() -> T + Send + 'static>> ) -> Group<T> where T: Send + 'static {

        let handles = tasks.into_iter().map(Rt::spawn_blocking).collect();

        Group { handles }

    }

    pub fn map <T, R, F> ( items: &[T], func: F ) -> Vec<R> where T: Sync, R: Send, F: Fn(&T) -> R + Sync {

        let func = &func;

        std::thread::scope(|scope| {

            let handles: Vec<_> = items.iter().map(|item| scope.spawn(move || func(item))).collect();

            handles.into_iter().map(|handle|
                handle.join().unwrap_or_else(|payload| std::panic::resume_unwind(payload))
            ).collect()

        })

    }

    pub fn each <T, F> ( items: &[T], func: F ) where T: Sync, F: Fn(&T) + Sync {

        let func = &func;

        std::thread::scope(|scope| {

            for item in items {

                scope.spawn(move || func(item));

            }

        });

    }

    pub fn try_map <T, R, E, F> ( items: &[T], func: F ) -> Result<Vec<R>, E> where T: Sync, R: Send, E: Send, F: Fn(&T) -> Result<R, E> + Sync {

        Self::map(items, func).into_iter().collect()

    }

}
