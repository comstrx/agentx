use crate::core::error::AppError;
use super::arch::Halt;

impl From<AppError> for Halt {

    fn from ( error: AppError ) -> Self {

        Self::Failed(error)

    }

}
