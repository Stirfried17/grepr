use clap::{command, Arg, ArgMatches};

fn main() {

    let match_result: ArgMatches = command!()
        .arg(
            Arg::new("Input")
            )
        
        .arg(
            Arg::new("Pattern")
            )

        .arg(
            Arg::new("Extended Regex")
                .help("Enables pattern matchins using extended regular expressions")
                .short('E')
                .long("extended-regexp")
            )

        .arg(
            Arg::new("Fixed Strings")
                .help("Matches pattern as a fixed string not as a regular expression")
                .short('F')
                .long("fixed-strings")
            )
            
        



        .get_matches();
    






}


