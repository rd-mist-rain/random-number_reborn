use rand::thread_rng;
use rand::Rng;
use rand::distributions::Uniform;
pub fn choicef(mut arg:std::env::Args)->()
{
    let mut starter:f64=1.0;
    let mut ender:f64=50.0;
    let mut amount:usize=1;
    let mut precision:usize=2; 
    if let Some(precision_str)=arg.next()
    {
        if let Ok(precision_read)=precision_str.parse::<usize>()
        {
            precision=precision_read;
        }
        else
        {
            eprintln!("failed to parse precision");
            return;
        }
    }
    else 
    {
        eprintln!("No command found after argument \"choicef\", the correct syntax:rdcore choicef <precision> <starter> <ender>");
        return;
    }
    if let Some(starter_str)=arg.next()
    {
        if let Ok(starter_read)=starter_str.parse::<f64>()
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
        eprintln!("argument \"<starter>\" is missing after \"choicef\", the correct syntax:rdcore choicef <precision> <starter> <ender>");
        return;
    }

    if let Some(ender_str)=arg.next()
    {
        if let Ok(ender_read)=ender_str.parse::<f64>()
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
        eprintln!("argument \"<ender>\" is missing after \"choicef\" , the correct syntax:rdcore choicef <precision> <starter> <ender>");
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
        eprintln!("argument \"<amount>\" is missing after \"choicef\",the value amount is set to 1,you can also use the syntax:rdcore choicef <precision> <starter> <ender> <amount>");
    }


let start=std::time::Instant::now();

    let mut rng=thread_rng();
    let rd=Uniform::new(starter,ender);
    let mut random_numbers:Vec<f64>=Vec::with_capacity(amount);
    while random_numbers.len()<amount
    {
        random_numbers.push(rng.sample(rd));
    }
    for i in random_numbers
    {
        print!("{:.1$} ",i,precision);
    }
    print!("\n");

let end=std::time::Instant::now();
let duration=end-start;
println!("time consumption:{} microseconds",duration.as_micros());
}