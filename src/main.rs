use std::io::Write;

#[derive(Clone, Eq, Ord)]
struct AutoCorrectionItem<'a> {
    priority: u16,
    word: &'a str,
}

impl<'a> AutoCorrectionItem<'a> {
    pub fn new(word: &'a str, priority: u16) -> Self {
        AutoCorrectionItem { word, priority }
    }
}

impl PartialEq for AutoCorrectionItem<'_> {
    fn eq(&self, other: &AutoCorrectionItem<'_>) -> bool {
        self.word == other.word
    }
}

impl std::cmp::PartialOrd for AutoCorrectionItem<'_> {
    fn partial_cmp(&self, other: &AutoCorrectionItem<'_>) -> Option<std::cmp::Ordering> {
        Some(other.priority.cmp(&self.priority))
    }
}

struct AutoCorrectionResult<'a, 'b> {
    word: &'a str,
    is_perfect_match: bool,
    suggestions: Vec<AutoCorrectionItem<'b>>,
}

impl AutoCorrectionResult<'_, '_> {
    pub fn best_match(&self) -> &str {
        if self.is_perfect_match || self.suggestions.is_empty() {
            self.word
        } else {
            self.suggestions[0].word
        }
    }
}

struct AutoCorrectionProvider {
    list: Vec<&'static str>,
}

impl AutoCorrectionProvider {
    pub fn new() -> Self {
        let data = include_str!("../data/en_us.txt");
        let list = data.split("\n").into_iter().collect();
        AutoCorrectionProvider { list }
    }

    pub fn suggest<'a>(
        &self,
        refstr: &'a str,
        count: usize,
        threshold: u16,
    ) -> AutoCorrectionResult<'a, '_> {
        let comp = |a: &str, b: &str| {
            (strsim::jaro_winkler(a, b) * strsim::normalized_levenshtein(a, b)).sqrt()
        };
        let items = {
            let mut tmp = self
                .list
                .iter()
                .map(|s| (*s, (comp(refstr, s) * 1000_f64).trunc() as u16))
                .filter(|(_, p)| p > &threshold)
                .map(|(s, p)| AutoCorrectionItem::new(s, p))
                .collect::<Box<[AutoCorrectionItem]>>();
            tmp.sort_unstable_by(|a, b| b.cmp(&a));
            tmp.into_iter()
                .take(count)
                .map(|x| x.clone())
                .collect::<Vec<AutoCorrectionItem>>()
        };
        AutoCorrectionResult {
            word: refstr,
            is_perfect_match: refstr
                == if items.is_empty() {
                    refstr
                } else {
                    items[0].word
                },
            suggestions: items,
        }
    }
}

fn main() {
    let ac = AutoCorrectionProvider::new();
    loop {
        print!("stdin> ");
        std::io::stdout().flush().unwrap();
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        let matches = ac.suggest(&buf, 5, 500);
        println!(
            "matches> {} [{}]",
            matches.best_match(),
            matches
                .suggestions
                .iter()
                .map(|s| format!("{}: {}", s.word, s.priority))
                .collect::<Vec<String>>()
                .join(", ")
        );
        std::io::stdout().flush().unwrap();
    }
}
