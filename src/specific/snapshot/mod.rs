use super::Context;

pub struct Snapshot {
    // share tree trait with Context
}

impl Snapshot {
    pub fn from(context: &Context) -> Self {
        Self {
            // recursively evaluate and hydrate references+expressions as much as possible
        }
    }
}
