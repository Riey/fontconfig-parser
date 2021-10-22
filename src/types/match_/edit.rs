use crate::Property;
use strum_macros::EnumString;

#[derive(Clone, Debug, Default)]
pub struct Edit {
    pub mode: EditMode,
    pub binding: EditBinding,
    pub value: Property,
}

#[derive(Copy, Clone, Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
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

#[derive(Copy, Clone, Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
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
