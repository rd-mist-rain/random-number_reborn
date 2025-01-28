use rand::thread_rng;
use rand::Rng;
use rand::distributions::Uniform;
use std::collections::HashSet;
pub fn choice(mut arg:std::env::Args)->()
{
    let mut starter:i64=1;
    let mut ender:i64=50;
    let mut amount:usize=1;
    let mut allow_sorting=false;
    if let Some(starter_str)=arg.next()
    {
        if let Ok(starter_read)=starter_str.parse::<i64>()
        {
            starter=starter_read;
        }
        else
        {
            eprintln!("failed to parse starter");
            return;
        }
    }
    else 
    {
        eprintln!("No command found after argument \"choice\", the correct syntax:rdcore choice <starter> <ender>");
        return;
    }

    if let Some(ender_str)=arg.next()
    {
        if let Ok(ender_read)=ender_str.parse::<i64>()
        {
            ender=ender_read;
        }
        else
        {
            eprintln!("failed to parse ender");
            return;
        }
    }
    else 
    {
        eprintln!("argument \"<ender>\" is missing after \"choice\" , the correct syntax:rdcore choice <starter> <ender>");
        return;
    }
    
    if let Some(amount_str)=arg.next()
    {
        if let Ok(amount_read)=amount_str.parse::<usize>()
        {
            amount=amount_read;
        }
        else 
        {
            eprintln!("failed to parse amount");
            return;
        }
    }
    else 
    {
        eprintln!("argument \"<amount>\" is missing after \"choice\",the value amount is set to 1,you can also use the syntax:rdcore choice <starter> <ender> <amount>");
    }
    if ender-starter+1<amount as i64
    {
        amount=(ender-starter+1) as usize;
    }
    if let Some(sort_str)=arg.next()
    {
        if sort_str=="--sort" {allow_sorting=true;}
    }

let start=std::time::Instant::now();

    let mut rng=thread_rng();
    let rd=Uniform::new(starter,ender+1);
    let mut random_numbers:HashSet<i64>=HashSet::with_capacity(amount);
    while random_numbers.len()<amount
    {
        random_numbers.insert(rng.sample(rd));
    }
    if allow_sorting
    {
        let mut sorted_numbers:Vec<i64>=random_numbers.into_iter().collect();
        sorted_numbers.sort();
        for i in sorted_numbers
        {
            print!("{} ",i);
        }
        print!("\n");
    }
    else 
    {
        for i in random_numbers
        {
            print!("{} ",i);
        }
        print!("\n");
    }

let end=std::time::Instant::now();
let duration=end-start;
println!("time consumption:{} microseconds",duration.as_micros());
}
