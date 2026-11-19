use ztd::kern::*;
use std::path::PathBuf;
use crate::LACK_OF_ARGUMENT_EXIT_CODE;

pub fn own_name_steamed(own_name: Option<String>) -> String
{
    let own_name = own_name
        .unwrap_or_bayern()
        .msgdln("Could not fetch the own name")
        .exit(LACK_OF_ARGUMENT_EXIT_CODE);

    let own_name = PathBuf::from(own_name);

    let own_name = own_name.file_stem()
        .unwrap_or_bayern()
        .msgdln("The own name steamed seems to be empty")
        .exit(LACK_OF_ARGUMENT_EXIT_CODE);

    let own_name = own_name.to_string_lossy();

return own_name.to_string() ; }
