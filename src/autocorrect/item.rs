use super::{CaseCorrectionMode, AutoCorrectionHelper};

#[derive(Debug, Clone, Eq, Ord)]
pub struct AutoCorrectionItem<'a> {
    pub mode: CaseCorrectionMode,
    pub priority: u16,
    pub word: &'a str,
}

impl<'a> AutoCorrectionItem<'a> {
    pub fn new(word: &'a str, priority: u16, mode: CaseCorrectionMode) -> Self {
        AutoCorrectionItem { word, priority, mode }
    }

    pub fn case_corrected(&self) -> String {
        AutoCorrectionHelper::correct_case(&self.mode, self.word)
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