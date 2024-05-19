use crate::types::Type;
use roxmltree::Node;

pub struct Entry {
    pub name: Option<String>,
    pub source: Option<String>,
}

impl Entry {
    pub fn new(node: &Node, tag: &Type) -> Self {
        let source = match tag {
            Type::Background => monster_source(node),
            Type::Class => item_source(node),
            Type::Feat => item_source(node),
            Type::Feature => item_source(node),
            Type::Item => item_source(node),
            Type::Monster => monster_source(node),
            Type::Race => monster_source(node),
            Type::Spell => item_source(node),
            Type::Unknown => None,
        };

        let name = find_first(node, |n| n.has_tag_name("name"));
        let name = name.and_then(|n| n.text().map(|s| s.to_string()));

        Self { name, source }
    }
}

fn monster_source(node: &Node) -> Option<String> {
    let traits = find_all(node, |n| n.has_tag_name("trait"));
    let first_trait = traits.first()?;

    item_source(first_trait)
}

fn item_source(node: &Node) -> Option<String> {
    let texts = find_all(node, |n| n.has_tag_name("text"));
    let name = find_first(node, |n| n.has_tag_name("name"));
    let source = texts.iter().find(|n| is_source_text(n.text()));
    let name = name?.text()?;

    if name == "Source" {
        return texts.last()?.text().map(|s| s.to_string());
    }

    source?.text().map(|s| s.to_string())
}

fn is_source_text(s: Option<&str>) -> bool {
    match s {
        Some(s) => s.trim().starts_with("Source:"),
        None => false,
    }
}

fn find_all<'a, P>(node: &'a Node, predicate: P) -> Vec<Node<'a, 'a>>
where
    P: Fn(&Node) -> bool,
{
    node.children()
        .filter(|n| n.is_element())
        .filter(|n| predicate(n))
        .collect()
}

fn find_first<'a, P>(node: &'a Node, predicate: P) -> Option<Node<'a, 'a>>
where
    P: Fn(&Node) -> bool,
{
    node.children()
        .filter(|n| n.is_element())
        .find(|n| predicate(n))
}
