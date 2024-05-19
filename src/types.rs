#[derive(PartialEq)]
pub enum Type {
    Background,
    Class,
    Feat,
    Feature,
    Item,
    Monster,
    Race,
    Spell,
    Unknown,
}

impl Type {
    pub fn from_str(s: &str) -> Self {
        match s {
            "background" => Self::Background,
            "class" => Self::Class,
            "feat" => Self::Feat,
            "feature" => Self::Feature,
            "item" => Self::Item,
            "monster" => Self::Monster,
            "race" => Self::Race,
            "spell" => Self::Spell,
            _ => Self::Unknown,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::Background => "backgrounds",
            Self::Feat => "feats",
            Self::Feature => "features",
            Self::Item => "items",
            Self::Monster => "monsters",
            Self::Race => "races",
            Self::Spell => "spells",
            _ => "unknown",
        }
    }
}
