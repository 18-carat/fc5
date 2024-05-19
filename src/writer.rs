use crate::config::Config;
use crate::entry::Entry;
use crate::types::Type;
use roxmltree::Node;
use std::fs::File;
use uxmlwriter::XmlWriter;

pub fn xml(node: Node, config: Config, file: File) {
    let mut w = XmlWriter::new(file);

    w.write_declaration();
    write_node(&mut w, node, true, &config);
    w.end_document();
}

fn encode_text(text: &str) -> String {
    text.replace('&', "&amp;")
}

fn write_node(w: &mut XmlWriter, node: Node, is_root: bool, config: &Config) {
    let tag_name = node.tag_name().name();
    let tag_type = Type::from_str(tag_name);
    let entry = Entry::new(&node, &tag_type);

    if !config.is_valid(tag_type, &entry) {
        return;
    }

    w.start_element(tag_name);

    if is_root {
        w.write_attribute("xmlns:exsl", "http://exslt.org/common");
    }

    node.attributes().for_each(|attr| {
        w.write_attribute(attr.name(), attr.value());
    });

    if let Some(text) = node.text() {
        if !text.trim().is_empty() {
            w.write_text(&encode_text(text));
            w.close_element();

            return;
        }
    }

    node.children()
        .filter(|c| c.is_element())
        .for_each(|n| write_node(w, n, false, config));

    w.close_element();
}
