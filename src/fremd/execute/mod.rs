use std::process::Command;
use ztd::kern::*;

use crate::CANT_EXECUTE_EXIT_CODE;

pub fn execute(args: &mut Vec<String>) -> i32
{
    let mut args = args.iter();

    if let Some(cmd) = args.next() 
    {
        let status = Command::new_inherited(cmd)
            .args(args)
            .status();

        let exit_code = status
        .unwrap_or_bye(|bayern, err| 
        {
            let msg_err = err.to_string();
            bayern
                .msgdln(f!("{msg_err}"))
                .exit(CANT_EXECUTE_EXIT_CODE)
        }).code();

        if let Some(exit_code) = exit_code 
        {
            return exit_code
        };
    }

return 0 ; }
