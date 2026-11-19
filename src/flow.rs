use crate::{
    eraser,
    fremd
};
use std::{
    env, 
    process::exit
};

pub fn init() 
{
    let mut args = env::args();

    let own_name = fremd::own_name_steamed(
        args
        .next()
    );

    eraser::in_regard_of(&own_name);
    
    let mut args: Vec<String> = args.collect();

    let last = fremd::pop_last(&mut args, &own_name);

    let code = fremd::execute(&mut args);
    
    if let Some(last) = last 
    {
        eraser::in_regard_of(&last);
    }

exit(code); }
