use rand::seq::SliceRandom;
use rand::thread_rng;

#[no_mangle]
pub extern "C" fn choiceship(mut arg:std::env::Args) -> () 
{
    let low:i64;
    let high:i64;
    let amount:usize;
    let allow_sorting:bool;
    let step:u32;
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
        eprintln!("No argument found after command \"choiceship\",the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort <value1>-<ship1> <value2>-<ship2>...");
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
        eprintln!("argument \"<high>\" is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort <value1>-<ship1> <value2>-<ship2>...");
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
        eprintln!("argument \"<amount>\" is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort <value1>-<ship1> <value2>-<ship2>...");
        return;
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
        eprintln!("argument \"<step>\" is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort <value1>-<ship1> <value2>-<ship2>...");
        return;
    }
    if let Some(sort_str)=arg.next()
    {
        if sort_str=="--sort"{allow_sorting=true;}
        else{allow_sorting=false;}
    }
    else
    {
        eprintln!("sort argument is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort <value1>-<ship1> <value2>-<ship2>...");
        return;
    }

    let mut values:Vec<i64>=Vec::from_iter((low..high).step_by(step as usize));
    let arg_vec:Vec<String>=arg.collect();
    for i in arg_vec
    {
        let value:i64;
        let ship:u16;
        let ships=i.split_once("-").unwrap();
        if let Ok(value_read)=ships.0.parse::<i64>(){value=value_read;}
        else{eprintln!("failed to parse value in {}",i);continue;}
        if let Ok(ship_read)=ships.1.parse::<u16>(){ship=ship_read;}
        else{eprintln!("failed to parse ship in {}",i);continue;}
        if values.contains(&value)
        {
            if ship==0{values.retain(|&x| x!=value );}
            else if ship>=1
            {
                for _ in 1..=ship
                {
                    values.push(value);
                }
            }
        }
    }
    values.shuffle(&mut thread_rng());
    if allow_sorting
    {
        let mut results=values.into_iter().take(amount).collect::<Vec<i64>>();
        results.sort();
        for i in results
        {
            print!("{} ",i);
        }
        print!("\n");
    }
    else
    {
        for i in values.iter().take(amount)
        {
            print!("{} ",i);
        }
        print!("\n");
    }
}
