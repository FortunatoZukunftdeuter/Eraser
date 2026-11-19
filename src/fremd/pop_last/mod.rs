use ztd::kern::*;

pub fn pop_last(args: &mut Vec<String>, own_name: &str) -> Option<String>
{
    let last = match own_name.is_alphabetic()
    {
        false => args.pop(),
        true => args.pop_if(|last| 
            last.to_lowercase() == own_name.to_lowercase()
        )
    };
return last ; }
