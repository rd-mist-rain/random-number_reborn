use rand::seq::SliceRandom;
use rand::thread_rng;

#[no_mangle]
pub extern "C" fn choicestr(mut arg:std::env::Args) -> () 
{
    let amount:usize;
    if let Some(amount_str)=arg.next()
    {
        if let Ok(amount_read)=amount_str.parse::<usize>()
        {
            amount=amount_read;
        }
        else
        {
            eprintln!("failed to parse amount value");
            return;
        }
    }
    else
    {
        eprintln!("No argument found after command \"choicestr\",the correct syntax:rdcore choicestr <amount> ...");
        return;
    }
    let mut values:Vec<String>=arg.collect();
    let mut rng=thread_rng();
    values.shuffle(&mut rng);
    for value in values.iter().take(amount)
    {
        print!("{} ",value);
    }
    print!("\n");
}
