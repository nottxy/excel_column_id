use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[cfg(feature = "with-serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq)]
#[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
pub struct ColumnId {
    id: usize,
    name: String,
}

impl ColumnId {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl From<usize> for ColumnId {
    fn from(id: usize) -> ColumnId {
        let name = id_to_name(id);
        ColumnId { id, name }
    }
}

impl Into<(usize, String)> for ColumnId {
    fn into(self) -> (usize, String) {
        (self.id, self.name)
    }
}

impl Ord for ColumnId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for ColumnId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl PartialEq for ColumnId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for ColumnId {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

fn id_to_name(id: usize) -> String {
    let mut dividers = Vec::default();

    let mut reminder = id;
    loop {
        let next_reminder = reminder / 27;
        dividers.push(reminder - next_reminder * 26);

        if next_reminder == 0 {
            break;
        }

        reminder = next_reminder;
    }

    dividers
        .into_iter()
        .rev()
        .map(|c| char::from(c as u8 + 64))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::ColumnId;

    #[test]
    fn test_from() {
        assert_eq!(ColumnId::from(1).name(), "A");
        assert_eq!(ColumnId::from(26).name(), "Z");
        assert_eq!(ColumnId::from(27).name(), "AA");
    }
}
