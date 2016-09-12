extern crate select;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() {
    let document = Document::from(include_str!("../thepiratebay--top--207.html"));
    for node in document.find(Class("detName")).find(Name("a")).iter().take(5) {
        println!("{}", node.text());
        for n in node.parent().unwrap().parent().unwrap().find(Name("a")).iter() {
            if n.attr("href").unwrap().starts_with("magnet:") {
                println!("{}", n.attr("href").unwrap());
            }
        }
    }
}
