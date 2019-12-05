#![allow(dead_code)]

use super::AutoCorrectionItem;

#[derive(Debug)]
pub struct AutoCorrectionResult<'a, 'b> {
    word: &'a str,
    is_perfect_match: bool,
    suggestions: Vec<AutoCorrectionItem<'b>>,
}

impl AutoCorrectionResult<'_, '_> {
    pub fn new<'a, 'b>(
        word: &'a str,
        is_perfect_match: bool,
        suggestions: Vec<AutoCorrectionItem<'b>>,
    ) -> AutoCorrectionResult<'a, 'b> {
        AutoCorrectionResult {
            word,
            is_perfect_match,
            suggestions,
        }
    }

    pub fn best_match(&self) -> &str {
        if self.is_perfect_match || self.suggestions.is_empty() {
            self.word
        } else {
            self.suggestions[0].word
        }
    }

    pub fn is_perfect_match(&self) -> bool {
        self.is_perfect_match
    }

    pub fn get_suggestions(&self) -> &[AutoCorrectionItem<'_>] {
        &self.suggestions[..]
    }
}
