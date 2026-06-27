use tokio::task::JoinHandle;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Thread;

pub struct Task <T> {
    pub handle: JoinHandle<T>,
}

pub struct Group <T> {
    pub handles: Vec<JoinHandle<T>>,
}
