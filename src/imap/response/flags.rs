#[derive(Debug, PartialEq, Eq)]
pub enum Flags {
    NonExistent,
    NoInferiors,
    NoSelect,
    HasChildren,
    HasNoChildren,
    Marked,
    Unmarked,
    Subscribed,
    Remote,
    All,
    Archive,
    Drafts,
    Flagged,
    Junk,
    Sent,
    Trash,
}

impl From<&str> for Flags {
    fn from(s: &str) -> Self {
        match s {
            "NonExistent" => Self::NonExistent,
            "Noinferiors" => Self::NoInferiors,
            "Noselect" => Self::NoSelect,
            "HasChildren" => Self::HasChildren,
            "HasNoChildren" => Self::HasNoChildren,
            "Marked" => Self::Marked,
            "Unmarked" => Self::Unmarked,
            "Subscribed" => Self::Subscribed,
            "Remote" => Self::Remote,
            "All" => Self::All,
            "Archive" => Self::Archive,
            "Drafts" => Self::Drafts,
            "Flagged" => Self::Flagged,
            "Junk" => Self::Junk,
            "Sent" => Self::Sent,
            "Trash" => Self::Trash,
            _ => unreachable!(),
        }
    }
}
