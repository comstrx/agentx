use super::arch::Output;

impl Output {

    pub fn ok ( &self ) -> bool {

        self.code == 0 && !self.timed_out

    }

    pub fn failed ( &self ) -> bool {

        !self.ok()

    }

}
