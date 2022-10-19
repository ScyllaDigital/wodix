extern crate core;

use crate::web_scraper::scraper::Scraper;

mod web_scraper;

fn main() {
    let mut web_scraper = Scraper::new("");

    let node_tree = web_scraper.to_node_tree();

    for node in node_tree.tree.nodes().next() {
        println!("{}", node.value().as_text().unwrap().text.to_string());
    }
}
