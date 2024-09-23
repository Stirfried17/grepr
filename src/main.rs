use clap::{command, Arg, ArgMatches};
use std::env;

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
                .help("Enables pattern matching using extended regular expressions")
                .short('E')
                .long("extended-regexp")
            )

        .arg(
            Arg::new("Fixed Strings")
                .help("Matches pattern as a fixed string not as a regular expression")
                .short('F')
                .long("fixed-strings")
            )

        .arg(
            Arg::new("Basic Regex")
                .help("Matches patterns using basic regex")
                .short('G')
                .long("basic-regexp")
                )

        .arg(
            Arg::new("Perl Regex")
                .help("Matches pattern using perl regex")
                .short('P')
                .long("perl-regexp")
            )
        
        .arg(
            Arg::new("Patterns")
                .help("Selects patterns to match strings")
                .short('e')
                .long("regexp=")
            )

        .arg(
            Arg::new("File")
                .help("Selects a patterns from a file")
                .short('f')
                .long("file=")
            )
        .arg(
            Arg::new("Ignore Case")
                .help("Ignores case sensitivity")
                .short('i')
                .long("ignore-case")
            )

        .arg(
            Arg::new("No Ignore Case")
                .help("Does not ignore case differences")
                .long("no-ignore-case")
            )

        .arg(
            Arg::new("Invert Match")
                .help("Shows only lines that do not match pattern")
                .short('v')
                .long("invert-match")
            )

        .arg(
            Arg::new("Word Regex")
                .help("Selects only complete words")
                .short('w')
                .long("word-regexp")
            )

        .arg(
            Arg::new("Line Regex")
                .help("Selects the whole line with a match")
                .short('x')
                .long("line-regexp")
            )

        .arg(
            Arg::new("Count")
                .help("Counts the total number of matches")
                .short('c')
                .long("count")
            )

        .arg(
            Arg::new("color")
                .help("Sets whether to use color to highlight matches")
                .long("color")
                .alias("colour")
                )

        .arg(
            Arg::new("Files Without Match")
                .help("Shows only files without matches")
                .short('L')
                .long("files-without-matches")
            )

        .arg(
            Arg::new("Files With Match")
                .help("Shows only files with matches")
                .short('l')
                .long("files-with-matches")
            )

        .arg(
            Arg::new("Max Count")
                .help("Stops matching after the specified")
                .short('m')
                .long("max-count")
            )

        .arg(
            Arg::new("Only Matchin")
                .help("Shows only the matching strings")
                .short('o')
                .long("only-matching")
            )

        .arg(
            Arg::new("Quiet")
                .help("Does not write to standard output")
                .short('q')
                .long("quiet")
                .alias("silent")
            )

        .arg(
            Arg::new("No Messages")
                .help("No error messages are written to standard output")
                .short('s')
                .long("no-messages")
            )

        .arg(
            Arg::new("Byte Offset")
                .help("Print the byte offset within the inout file")
                .short('b')
                .long("byte-offset")
            )

        .arg(
            Arg::new("With Filename")
                .help("Print the file name for each match")
                .short('H')
                .long("with-filename")
            )
        
        .arg(
            Arg::new("No Filename")
            .help("Do not print the file name for each match")
            .short('h')
            .long("no-filename")
            )

        .arg(
            Arg::new("Label")
            .help("Displays input actually coming from standard input as coming from file label")
            .long("label=")
            )

        .arg(
            Arg::new("Line Number")
            .help("Displays line numbers")
            .short('n')
            .long("line-number")
            )

        .arg(
            Arg::new("Initial Tab")
            .help("The first character of a line must be a tab")
            .short('T')
            .long("intial-tab")
            )
        .arg(
            Arg::new("Null")
            .help("Outputs a zero byte character instead of the character that usually follows the a file name")
            .short('Z')
            .long("null")

        
            
        



        .get_matches();
    






}


