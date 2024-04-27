use std::io;
use clap::*;


fn makepattern ()

fn findoffset ()

fn main() {


    println!("DeBruijn Sequence Pattern Create and Offset Calc")

    let matches = command!()
    .arg(arg!([SIZE])
         .value_parser(value_parser!(u16))
         .default_value("400"),
         )
    .arg(Arg::new("find")
         .short('f')
         .long("find")
         .action(ArgAction::SetTrue),
         )
    .get_matches();
    
    println!("Find: {:?}", matches.get_flag("find"))




}
