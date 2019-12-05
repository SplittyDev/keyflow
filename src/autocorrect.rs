mod helper;
mod item;
mod result;
mod provider;

use self::helper::{CaseCorrectionMode, AutoCorrectionHelper};
pub use self::item::AutoCorrectionItem;
pub use self::result::AutoCorrectionResult;
pub use self::provider::AutoCorrectionProvider;