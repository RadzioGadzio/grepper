use regex::Regex;

pub trait PatternTrait {
    fn to_regex(&self) -> Regex;
    fn string(&self) -> String;
    fn matches_text(&self, text: &str, strict: bool) -> bool {
        if strict {
            text.contains(&self.string())
        } else {
            self.to_regex().is_match(text)
        }
    }
}
impl PatternTrait for Regex {
    fn to_regex(&self) -> Regex {
        self.clone()
    }
    fn string(&self) -> String {
        self.to_string()
    }
}
impl PatternTrait for String {
    fn to_regex(&self) -> Regex {
        Regex::new(self.as_str()).unwrap()
    }
    fn string(&self) -> String {
        self.clone()
    }
}