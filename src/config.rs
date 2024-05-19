use crate::entry::Entry;
use crate::types::Type;
use boml::prelude::Toml;
use std::collections::HashMap;
use std::fs::read_to_string;

struct Override {
    name: String,
    allow: bool,
}

pub struct Config {
    books: HashMap<String, Vec<String>>,
    overrides: Vec<Override>,
}

impl Config {
    fn parse_books(toml: &Toml) -> Option<HashMap<String, Vec<String>>> {
        let mut map = HashMap::new();

        for (key, value) in toml.iter() {
            let books = value
                .array()?
                .iter()
                .filter_map(|b| b.string())
                .map(|s| s.to_string())
                .collect();

            map.insert(key.to_string(), books);
        }

        Some(map)
    }

    fn parse_overrides(toml: &Toml) -> Option<Vec<Override>> {
        let array = toml.get_array("overrides").ok()?;

        let overrides = array.iter().filter_map(|o| {
            let name = o.table()?.get_string("name").ok()?.to_string();
            let allow = o.table()?.get_boolean("allow").ok()?;

            Some(Override { name, allow })
        });

        Some(overrides.collect())
    }

    pub fn new(path: String) -> Option<Self> {
        let contents = read_to_string(path).ok()?;
        let toml = Toml::parse(&contents).ok()?;
        let books = Self::parse_books(&toml)?;
        let overrides = Self::parse_overrides(&toml)?;

        Some(Self { books, overrides })
    }

    pub fn is_valid(&self, type_: Type, entry: &Entry) -> bool {
        let name = entry.name.as_deref().unwrap_or_default();
        let src = entry.source.as_deref().unwrap_or_default();

        if let Some(o) = &self.overrides.iter().find(|o| o.name == *name) {
            return o.allow;
        }

        if type_ == Type::Unknown || type_ == Type::Class {
            return true;
        }

        self.books
            .get(type_.to_string())
            .unwrap_or(&Vec::new())
            .iter()
            .any(|n| src.contains(n))
    }
}
