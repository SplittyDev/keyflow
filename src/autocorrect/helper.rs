#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum CaseCorrectionMode {
    Screaming,
    Standard,
    Whispering,
}

pub struct AutoCorrectionHelper;

impl AutoCorrectionHelper {
    fn convert_to_standard_case_string(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
    pub fn determine_case_mode(refword: &str) -> CaseCorrectionMode {
        let chars = refword.chars();
        if refword.len() > 1 && chars.clone().all(|c| c.is_uppercase()) {
            CaseCorrectionMode::Screaming
        } else if refword.len() > 1 && chars.clone().all(|c| c.is_lowercase()) {
            CaseCorrectionMode::Whispering
        } else {
            CaseCorrectionMode::Standard
        }
    }
    pub fn correct_case(mode: &CaseCorrectionMode, word: &str) -> String {
        match mode {
            CaseCorrectionMode::Screaming => word.to_uppercase(),
            CaseCorrectionMode::Whispering => word.to_lowercase(),
            _ => AutoCorrectionHelper::convert_to_standard_case_string(word),
        }
    }
}