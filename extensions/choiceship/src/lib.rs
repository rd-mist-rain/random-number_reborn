use rand::thread_rng;
use std::collections::HashMap;
use std::ops::Range;
use std::collections::HashSet;
use rand::distributions::Uniform;
use rand::Rng;

#[no_mangle]
pub extern "C" fn choiceship(mut arg:std::env::Args) -> () 
{
    let low:i64;
    let high:i64;
    let mut amount:usize;
    let allow_sorting:bool;
    let allow_deduplicate:bool;
    let mut step:u32;
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
        eprintln!("No argument found after command \"choiceship\",the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort --dedup/--dup <value1>-<ship1> <value2>-<ship2>...");
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
        eprintln!("argument \"<high>\" is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort --dedup/--dup <value1>-<ship1> <value2>-<ship2>...");
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
        eprintln!("argument \"<amount>\" is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort --dedup/--dup <value1>-<ship1> <value2>-<ship2>...");
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
        eprintln!("argument \"<step>\" is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort --dedup/--dup <value1>-<ship1> <value2>-<ship2>...");
        return;
    }
    if let Some(sort_str)=arg.next()
    {
        if sort_str=="--sort"{allow_sorting=true;}
        else{allow_sorting=false;}
    }
    else
    {
        eprintln!("sort argument is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort --dedup/--dup <value1>-<ship1> <value2>-<ship2>...");
        return;
    }
    if let Some(dedup_str)=arg.next() //dedup:deduplicate(去重)
    {
        if dedup_str=="--dedup"{allow_deduplicate=true;}
        else{allow_deduplicate=false;}
    }
    else 
    {
        eprintln!("dedup argument is missing,the correct syntax:rdcore choiceship <low> <high> <amount> <step> --sort/--unsort --dedup/--dup <value1>-<ship1> <value2>-<ship2>...");
        return;
    }

    
    let arg_vec:Vec<String>=arg.collect();
    let mut mappings:HashMap<Range<i64>,i64>=HashMap::new(); //映射值,实际值
    let mut remove_nums:HashSet<i64>=HashSet::new();
    let mut sum_ships:u64=0;

    for i in arg_vec
    {
        let value:i64;
        let ship:u16;
        let ships=i.split_once("-").unwrap();
        if let Ok(value_read)=ships.0.parse::<i64>(){value=value_read;}
        else{eprintln!("failed to parse value in {}",i);continue;}
        if let Ok(ship_read)=ships.1.parse::<u16>(){ship=ship_read;}
        else{eprintln!("failed to parse ship in {}",i);continue;}
        if ship!=0
        {
            let a:i64=high+sum_ships as i64;
            sum_ships+=ship as u64;
            mappings.insert(a..a+ship as i64,value);
        }
        else{remove_nums.insert(value);}
       
    }
    if high-low<step as i64 || step==0
    {
        step=1;
    }


    let mut rng=thread_rng();
    let rd=Uniform::new(low,high+sum_ships as i64);
    if allow_deduplicate
    {
        if high-low<amount as i64
        {
            amount=(high-low) as usize;
        }
        let mut random_numbers:HashSet<i64>=HashSet::with_capacity(amount);
        if step==1
        {
            while random_numbers.len()<amount
            {
                let mut n=rng.sample(rd);
                if remove_nums.contains(&n){continue;}
                for i in mappings.keys()
                {
                    if i.contains(&n)
                    {
                        n=* mappings.get(i).unwrap();
                        break;
                    }
                }
                random_numbers.insert(n);
            }
        }
        else
        {
            while random_numbers.len()<amount
            {
                let mut n=rng.sample(rd);
                if remove_nums.contains(&n) {continue;}
                for i in mappings.keys()
                {
                    if i.contains(&n)
                    {
                        n=* mappings.get(i).unwrap();
                        break;
                    }
                }
                if (n-low)%step as i64==0 {random_numbers.insert(n);}
            }
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
    }
    else
    {
        if high+sum_ships as i64-low<amount as i64
        {
            amount=(high+sum_ships as i64-low) as usize;
        }
        let mut random_numbers:Vec<i64>=Vec::with_capacity(amount);
        if step==1
        {
            while random_numbers.len()<amount
            {
                let mut n=rng.sample(rd);
                if remove_nums.contains(&n){continue;}
                for i in mappings.keys()
                {
                    if i.contains(&n)
                    {
                        n=* mappings.get(i).unwrap();
                        break;
                    }
                }
                random_numbers.push(n);
            }
        }
        else
        {
            while random_numbers.len()<amount
            {
                let mut n=rng.sample(rd);
                if remove_nums.contains(&n) {continue;}
                for i in mappings.keys()
                {
                    if i.contains(&n)
                    {
                        n=* mappings.get(i).unwrap();
                        break;
                    }
                }
                if (n-low)%step as i64==0 {random_numbers.push(n);}
            }
        }
        if allow_sorting {random_numbers.sort();}
        for i in random_numbers
        {
            print!("{} ",i);
        }
        print!("\n");
    }
}
