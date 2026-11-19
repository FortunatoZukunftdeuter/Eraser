use ztd::kern::*;
use std::io::stdout;
use crossterm::{
    execute,
    cursor::MoveTo,
    terminal::{
        Clear,
        ClearType
    }
};

use crate::CANT_ERASE_EXITCODE;

pub fn erase_if_set(erase_type: Option<ClearType>) 
{
    if let Some(erase_type) = erase_type 
    {
        execute!(stdout(), Clear(erase_type), MoveTo(0, 0))
        .unwrap_or_bye(|bayern, err| 
        {
            let msg_err = err.to_string();
            bayern
                .msgdln("Could not erase the screen")
                .msgdln(f!("Error: {msg_err}"))
                .exit(CANT_ERASE_EXITCODE)
        });
    }
}
