extern crate select;
use select::document::Document;
use select::predicate::{Predicate, Attr, Class, Name};

fn main() {
    let document = Document::from(include_str!("../thepiratebay--top--207.html"));
    println!("[");
    for node in document.find(Attr("id", "searchResult"))
        .find(Name("tbody"))
        .find(Name("tr"))
        .iter()
        .take(5) {

        let children = node.children();
        let mut iter = children.iter();
        let td_name_magnet = iter.next().unwrap().next().unwrap().next().unwrap().next().unwrap();

        let name_selection = td_name_magnet.find(Class("detName")).find(Name("a"));
        let mut name_selection_iter = name_selection.iter();
        let name = name_selection_iter.next().unwrap().text();

        let mut magnet = String::new();


        for n in td_name_magnet.find(Name("a")).iter() {
            if n.attr("href").unwrap().starts_with("magnet:") {
                magnet = n.attr("href").unwrap().to_string();
                break;
            }
        }

        let seeders = td_name_magnet.next().unwrap().next().unwrap();
        let leechers = seeders.next().unwrap().next().unwrap();

        println!("  {{");
        println!("    name: \"{}\"", name);
        println!("    magnet: \"{}\"", magnet);
        println!("    seeders: {}", seeders.text());
        println!("    leechers: {}", leechers.text());
        println!("  }},");
    }
    println!("]");
}
