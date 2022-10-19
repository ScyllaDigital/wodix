use scraper::Html;

pub struct Scraper<'a> {
    url: &'a String,
    raw_content: Option<String>,
}

pub struct LinkNode {
    href: String,
    text: String,
}

impl Scraper {
    pub fn new(url: &String) -> Self {
        Scraper {
            url,
            raw_content: Scraper::fetch_raw(url),
        }
    }

    fn fetch_raw(url: &String) -> Option<String> {
        let result = match reqwest::blocking::get(url).unwrap().text() {
            Ok(v) => Some(v),
            Err(err) => None
        };

        result
    }


    pub fn to_node_tree(self) -> Html {
        match self.raw_content {
            Some(content) => Html::parse_document(&content),
            None => panic!("Fatal error when attempting to parse document - exiting")
        }
    }

    pub fn collect_page_links(&mut self) -> Vec<LinkNode> {
        Vec::new()
    }
}