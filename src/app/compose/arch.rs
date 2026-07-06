#[derive(Clone, Copy, Debug, Default)]
pub struct Compose;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Stage {
    Init,
    Confirm,
    Work,
    Intake,
    Convert,
    Discover,
    Gate,
    Review,
    Finalize,
    Fence,
    Evidence,
    Report,
}

impl Stage {

    pub(crate) fn manager_only ( self ) -> bool {

        matches!(self, Self::Intake | Self::Convert | Self::Discover | Self::Gate | Self::Review | Self::Finalize)

    }

    pub(crate) fn worker_only ( self ) -> bool {

        matches!(self, Self::Fence | Self::Evidence)

    }

}
