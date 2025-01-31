use libloading::{Library, Symbol};

#[allow(unused_mut)]
pub fn load(name: &str,mut arg:std::env::Args) -> ()
{
    let path=format!(".\\extensions\\{}.dll",name);
    unsafe
    {
        let lib=Library::new(path).unwrap();
        let func:Symbol<unsafe extern "C" fn(std::env::Args) -> ()>=lib.get(name.as_bytes()).unwrap();
        func(arg);
    }
}