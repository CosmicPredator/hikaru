use wildcard::Wildcard;


pub struct Filters {
    include: Vec<String>,
    exclude: Vec<String>,
    min_size: u32,
    max_size: u32
}

impl Filters {
    pub fn is_match(&self, item: &str, min_size: u32, max_size: u32) -> bool {
        let mut is_match = false;

        for filter in &self.include {
            let wc = Wildcard::new(filter.as_bytes());
            if let Ok(wildcard) = wc {
                is_match = wildcard.is_match(item.as_bytes());
            } else {
                is_match = false;
            }
        }

        for filter in &self.exclude {
            let wc = Wildcard::new(filter.as_bytes());
            if let Ok(wildcard) = wc {
                is_match = wildcard.is_match(item.as_bytes());
            } else {
                is_match = false;
            }
        }

        is_match = min_size >= self.min_size;
        is_match = max_size <= self.max_size;

        is_match
    }
}

