#[cfg(test)]
mod test;

pub mod plover_dict;
pub mod dictionary;
pub mod machine;
pub mod outline;

pub use outline::{Key, Stroke, Outline};
pub use dictionary::Dictionary;
