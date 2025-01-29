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
    let mut step:u32=1;
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
        eprintln!("No argument found after command \"choice\",the correct syntax:rdcore choice <low> <high>");
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
        eprintln!("argument \"<high>\" is missing,the correct syntax:rdcore choice <low> <high>");
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
        eprintln!("argument \"<amount>\" is missing,the value amount is set to 1,you can also use the syntax:rdcore choice <low> <high> <amount>");
    }
    if let Some(step_str)=arg.next()
    {
        if let Ok(step_read)=step_str.parse::<u32>()
        {
            step=step_read;
        }
        else 
        {
            eprintln!("failed to parse step");
            return;
        }
    }
    else 
    {
        eprintln!("argument \"<step>\" is missing,the value step is set to 1,you can also use the syntax:rdcore choice <low> <high> <amount> <step>")
    }
    if let Some(sort_str)=arg.next()
    {
        if sort_str=="--sort" {allow_sorting=true;}
    }

let start=std::time::Instant::now();
    if high-low<amount as i64
    {
        amount=(high-low) as usize;
    }
    if high-low<step as i64 || step==0
    {
        step=(high-low) as u32;
    }
    let mut rng=thread_rng();
    let rd=Uniform::new(low as i64,high as i64);
    let mut random_numbers:HashSet<i64>=HashSet::with_capacity(amount);
    if step==1
    {
        while random_numbers.len()<amount
        {
            random_numbers.insert(rng.sample(rd));
        }
    }
    else
    {
        while random_numbers.len()<amount
        {
            let n=rng.sample(rd);
            if (n-low)%step as i64==0{random_numbers.insert(n);}
        }
    }
    if allow_sorting
    {
        let mut sorted_numbers:Vec<i64>=random_numbers.into_iter().collect();
        sorted_numbers.sort();
        for i in sorted_numbers
        {
            print!("{} ",i as i64);
        }
        print!("\n");
    }
    else 
    {
        for i in random_numbers
        {
            print!("{} ",i as i64);
        }
        print!("\n");
    }

let end=std::time::Instant::now();
let duration=end-start;
println!("time consumption:{} microseconds",duration.as_micros());
}
