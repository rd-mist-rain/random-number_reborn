use std::env;
mod choice;
mod choicef;
mod load;
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
        "choicef"=>
        {
            choicef::choicef(args);
        }
        other=>
        {
            load::load(other,args);
        }
    }
    }
    else 
    {
        eprintln!("No command found");
        return;
    }
}
