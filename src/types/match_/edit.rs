use crate::{PropertyKind, Value};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Edit {
    pub name: PropertyKind,
    #[serde(default)]
    pub mode: EditMode,
    #[serde(default)]
    pub binding: EditBinding,
    #[serde(rename = "$value")]
    pub value: Value,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditBinding {
    Strong,
    Weak,
    Same,
}

impl Default for EditBinding {
    fn default() -> Self {
        EditBinding::Weak
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditMode {
    Assign,
    AssignReplace,
    Prepend,
    PrependFirst,
    Append,
    AppendLast,
    Delete,
    DeleteAll,
}

impl Default for EditMode {
    fn default() -> Self {
        EditMode::Assign
    }
}
