use rand::thread_rng;
use rand::Rng;
use rand::distributions::Uniform;
use std::collections::HashSet;

pub fn choicef(mut arg:std::env::Args)->()
{
    let low:i64;
    let high:i64;
    let mut amount:usize=1;
    let mut allow_sorting:bool=false;
    let factor:f64; //计算因子
    if let Some(precision_str)=arg.next()
    {
        if let Ok(precision_read)=precision_str.parse::<u32>()
        {
            factor=10.0f64.powf(precision_read as f64); //计算因子
        }
        else
        {
            eprintln!("failed to parse precision");
            return;
        }
    }
    else 
    {
        eprintln!("No argument found after command \"choicef\", the correct syntax:rdcore choicef <precision> <low> <high>");
        return;
    }
    if let Some(low_str)=arg.next()
    {
        if let Ok(low_read)=low_str.parse::<f64>()
        {
            low=(low_read*factor).round() as i64;
        }
        else
        {
            eprintln!("failed to parse low value");
            return;
        }
    }
    else 
    {
        eprintln!("argument \"<low>\" is missing after \"choicef\", the correct syntax:rdcore choicef <precision> <low> <high>");
        return;
    }

    if let Some(high_str)=arg.next()
    {
        if let Ok(high_read)=high_str.parse::<f64>()
        {
            high=(high_read*factor).round() as i64 +1;
        }
        else
        {
            eprintln!("failed to parse high value");
            return;
        }
    }
    else 
    {
        eprintln!("argument \"<high>\" is missing after \"choicef\" , the correct syntax:rdcore choicef <precision> <low> <high>");
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
        eprintln!("argument \"<amount>\" is missing after \"choicef\",the value amount is set to 1,you can also use the syntax:rdcore choicef <precision> <low> <high> <amount>");
    }
    if let Some(sort_str)=arg.next()
    {
        if sort_str=="--sort" {allow_sorting=true;}
    }

let start=std::time::Instant::now();

    let mut rng=thread_rng();
    let rd=Uniform::new(low,high);
    if high-low < amount as i64
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
            print!("{} ",i as f64/factor);
        }
        print!("\n");
    }
    else 
    {
        for i in random_numbers
        {
            print!("{} ",i as f64/factor);
        }
        print!("\n");
    }

let end=std::time::Instant::now();
let duration=end-start;
println!("time consumption:{} microseconds",duration.as_micros());
}
