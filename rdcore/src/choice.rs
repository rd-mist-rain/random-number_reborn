use rand::thread_rng;
use rand::Rng;
use rand::distributions::Uniform;
use std::collections::HashSet;
pub fn choice(mut arg:std::env::Args)->()
{
    let low:i64;
    let high:i64;
    let mut amount:usize=1;
    let mut allow_sorting:bool=false;
    if let Some(low_str)=arg.next()
    {
        if let Ok(low_read)=low_str.parse::<i64>()
        {
            low=low_read;
        }
        else
        {
            eprintln!("failed to parse low value");
            return;
        }
    }
    else 
    {
        eprintln!("No argument found after command \"choice\", the correct syntax:rdcore choice <low> <high>");
        return;
    }

    if let Some(high_str)=arg.next()
    {
        if let Ok(high_read)=high_str.parse::<i64>()
        {
            high=high_read+1;
        }
        else
        {
            eprintln!("failed to parse high value");
            return;
        }
    }
    else 
    {
        eprintln!("argument \"<high>\" is missing after \"choice\" , the correct syntax:rdcore choice <low> <high>");
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
        eprintln!("argument \"<amount>\" is missing after \"choice\",the value amount is set to 1,you can also use the syntax:rdcore choice <low> <high> <amount>");
    }
    if let Some(sort_str)=arg.next()
    {
        if sort_str=="--sort" {allow_sorting=true;}
    }

let start=std::time::Instant::now();

    let mut rng=thread_rng();
    let rd=Uniform::new(low,high);
    if high-low<amount as i64
    {
        amount=(high-low) as usize;
    }
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

