use rand::seq::SliceRandom;
use rand::thread_rng;

#[no_mangle]
pub extern "C" fn mix(arg:std::env::Args) -> () 
{
    let mut values:Vec<String>=arg.collect();
    let mut rng=thread_rng();
    values.shuffle(&mut rng);
    for value in values
    {
        print!("{} ",value);
    }
    print!("\n");
}
