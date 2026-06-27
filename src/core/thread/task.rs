use crate::core::rt::Rt;
use super::arch::Task;

impl <T: Send + 'static> Task<T> {

    pub fn is_finished ( &self ) -> bool {

        self.handle.is_finished()

    }

    pub fn wait ( self ) -> T {

        Rt::block_on(self.handle).unwrap_or_else(|error| std::panic::resume_unwind(error.into_panic()))

    }

    pub fn wait_timeout ( self, secs: u64 ) -> Option<T> {

        match Rt::timeout(secs, self.handle) {
            Some(Ok(value)) => Some(value),
            Some(Err(error)) => std::panic::resume_unwind(error.into_panic()),
            None => None,
        }

    }

    pub fn stop ( self ) {

        self.handle.abort();

    }

}
