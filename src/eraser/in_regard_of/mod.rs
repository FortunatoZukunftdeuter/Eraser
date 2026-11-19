use ztd::kern::*;
use crossterm::terminal::ClearType;

mod handler;

pub fn in_regard_of(name: &str) {
    let erase_type = if name.is_lowercase() {
        Some(ClearType::All)
    } else if name.is_uppercase() {
        Some(ClearType::Purge)
    } else {
        None
    };
    handler::erase_if_set(erase_type);
}
