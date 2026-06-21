use super::arch::Thread;

impl Thread {

    /// Map `items` in parallel, one scoped thread each, results in input order.
    /// A worker panic is re-raised on the caller (same as `rayon`).
    pub fn map<T, R, F> ( items: &[T], func: F ) -> Vec<R> where T: Sync, R: Send, F: Fn(&T) -> R + Sync {

        let func = &func;

        std::thread::scope(|scope| {

            let handles: Vec<_> = items.iter().map(|item| scope.spawn(move || func(item))).collect();

            handles.into_iter().map(|handle| handle.join().unwrap_or_else(|payload| std::panic::resume_unwind(payload))).collect()

        })

    }

    /// Run `func` over every item in parallel for its side effects.
    pub fn each<T, F> ( items: &[T], func: F ) where T: Sync, F: Fn(&T) + Sync {

        let func = &func;

        std::thread::scope(|scope| {

            for item in items {
                scope.spawn(move || func(item));
            }

        });

    }

}
