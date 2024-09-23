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
            )
        .arg(
            Arg::new("After Context")
            .help("Shows a number of lines after the pattern match")
            .short('A')
            .long("after-context")
            )

        .arg(
            Arg::new("Before Context")
            .help("Shows a number of lines before the pattern match")
            .short('B')
            .long("before-context")
            )

        .arg(
            Arg::new("Context")
            .help("Shows a number of lines around the pattern match")
            .short('C')
            .long("context=")
            )

        .arg(
            Arg::new("Group Separator")
            .help("With context options prints a separator between matching contexts")
            .long("group-separator")
            )

        .arg(
            Arg::new("No Group Separator")
            .help("With context options this does not print a separator between matching contexts")
            .long("no-group-separator")
            )

        .arg(
            Arg::new("Binary as Text")
            .help("Processes binary files as text")
            .short('a')
            .long("text")
            )
        
        .arg(
            Arg::new("Binary Files")
            .help("placeholder: I think it assumes a binary is binary unless told otherwise but honestly not sure what the description for grep means")
            .long("binary-files=")
            )

        .arg(
            Arg::new("Devices")
            .help("If file is a device use an action to process it")
            .short('D')
            .long("devices=")
            )

        .arg(
            Arg::new("Directories")
            .help("If file is a directory use an action to process it")
            .short('d')
            .long("directories=")
            )

        .arg(
            Arg::new("Exclude")
            .help("Skip files with name that matches pattern")
            .long("exclude=")
            )

        .arg(
            Arg::new("Exclude From")
            .help("Skip files whose base name matches pattern")
            .long("exclude-from")
            )

        .arg(
            Arg::new("Exclude Dir")
            .long("exclude-dir")
            )
            
        .arg(
            Arg::new("Binary Files Withou Match")
            .help("Processes binary as if there are no matches")
            .short('I')
            )

        .arg(
            Arg::new("Include")
            .help("Includes specific files that matches patterns")
            .long("include")
            )

        .arg(
            Arg::new("Recursive")
            .help("Read all files under directories")
            .short('r')
            .long("recursive")
            )

        .arg(
            Arg::new("Dereference Recursive")
            .help("Recursive but also follows symbolic links")
            .short('R')
            .long("dereference-recursive")
            )

        .arg(
            Arg::new("Line Buffered")
            .help("Use line buffered output")
            .long("line-buffered")
            )

        .arg(
            Arg::new("Binary")
            .help("Treat files as binary")
            .short('U')
            .long("binary")
            )

        .arg(
            Arg::new("Null Data")
            .help("Replace the new line at the end of input and output data with a zero byte")
            .short('z')
            .long("null-data")
            )



        .get_matches();
    






}


