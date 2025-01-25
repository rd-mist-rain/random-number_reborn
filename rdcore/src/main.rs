use std::env;
mod choice;
fn main() 
{
    let mut args= env::args(); //迭代器
    args.next();
    if let Some(mode)=args.next()
    {
    match mode.as_str()
    {
        "choice"=>
        {
            choice::choice(args);
        }
        other=>
        {
            eprintln!("Unknown command:\"{other}\"");
            return;
        }
    }
    }
    else 
    {
        eprintln!("No command found");
        return;
    }
}
