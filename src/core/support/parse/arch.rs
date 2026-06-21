/// TOML codec (via the `toml` crate).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Toml;

/// JSON codec (via `serde_json`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Json;

/// YAML codec — stub surface until a maintained backend is wired in.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Yaml;

/// Format-dispatching loader/saver, keyed by file extension.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Parse;
