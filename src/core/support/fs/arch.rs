/// Pure path queries (name, stem, extension, relative).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Path;

/// Directory operations (entries, markdown, copy-tree, numbering, clear).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Dir;

/// File operations (read, write, append, copy, remove).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct File;
