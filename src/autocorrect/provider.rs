#![allow(dead_code)]

use super::{AutoCorrectionItem, AutoCorrectionResult, AutoCorrectionHelper};

pub struct AutoCorrectionProvider {
    list: Vec<&'static str>,
}

impl AutoCorrectionProvider {
    pub fn new() -> Self {
        let data = include_str!("./../../data/en_us.txt");
        let list = data.lines().collect();
        AutoCorrectionProvider { list }
    }

    pub fn suggest<'a>(
        &self,
        refstr: &'a str,
        count: usize,
        threshold: u16,
    ) -> AutoCorrectionResult<'a, '_> {
        let comp = |a: &str, b: &str| {
            strsim::normalized_damerau_levenshtein(a, b)
        };
        let mode = AutoCorrectionHelper::determine_case_mode(refstr);
        let refstr_lower = refstr.to_lowercase();
        let items = {
            let mut tmp = self
                .list
                .iter()
                .map(|s| (*s, (comp(&refstr_lower, s) * 1000_f64).trunc() as u16))
                .filter(|(_, p)| p > &threshold)
                .map(|(s, p)| AutoCorrectionItem::new(s, p, mode))
                .collect::<Box<[AutoCorrectionItem]>>();
            tmp.sort_unstable_by(|a, b| b.cmp(&a));
            tmp.into_iter()
                .take(count)
                .map(|x| x.clone())
                .collect::<Vec<AutoCorrectionItem>>()
        };
        AutoCorrectionResult::new(
            refstr,
            items.iter().any(|i| i.word.to_lowercase() == refstr_lower),
            items,
        )
    }
}
