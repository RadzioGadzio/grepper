use regex::Regex;

pub trait PatternTrait {
    fn regex(&self) -> Regex;
    fn string(&self) -> String;
    fn matches_text(&self, text: &str, strict: bool) -> bool {
        if strict {
            text.contains(&self.string())
        } else {
            self.regex().is_match(text)
        }
    }
}
impl PatternTrait for Regex {
    fn regex(&self) -> Regex {
        self.clone()
    }
    fn string(&self) -> String {
        self.to_string()
    }
}
impl PatternTrait for String {
    fn regex(&self) -> Regex {
        Regex::new(self.as_str()).unwrap()
    }
    fn string(&self) -> String {
        self.clone()
    }
}