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

            
        



        .get_matches();
    






}


