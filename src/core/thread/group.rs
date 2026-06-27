use crate::core::rt::Rt;
use super::arch::Group;

impl <T: Send + 'static> Group<T> {

    pub fn len ( &self ) -> usize {

        self.handles.len()

    }

    pub fn is_empty ( &self ) -> bool {

        self.handles.is_empty()

    }

    pub fn is_finished ( &self ) -> bool {

        self.handles.iter().all(|handle| handle.is_finished())

    }

    pub fn wait ( self ) -> Vec<T> {

        Rt::block_on(async {

            let mut out = Vec::with_capacity(self.handles.len());

            for handle in self.handles {

                match handle.await {
                    Ok(value) => out.push(value),
                    Err(error) => std::panic::resume_unwind(error.into_panic()),
                }

            }

            out

        })

    }

    pub fn collect ( self ) -> Vec<T> {

        self.wait()

    }

    pub fn stop ( self ) {

        for handle in &self.handles {

            handle.abort();

        }

    }

}
