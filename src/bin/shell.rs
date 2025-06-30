use std::collections::HashMap;
use std::collections::HashSet;
use std::io::stdin;
use std::io::stdout;
use std::io::Write;

use taglight::db_init;
use taglight::db_gettagged;
use taglight::db_readtags;

const CMD_HELP : &str = "h";
const CMD_QUIT : &str = "q";
const CMD_LOAD : &str = "l";
const CMD_NOT : &str = "!";
const CMD_OR : &str = "|";
const CMD_TAG : &str = "#";
const CMD_DELETE : &str = "d";
const CMD_PRINT : &str = "p";
const CMD_PRINT_ALL : &str = "o";

const MANNUAL: &str = 
"
NAME
    tlsh - taglight tag shell

OPTIONS
    h   Print this help menu.

    q   Quit out of the shell.

    o   Print out all the files loaded into the database.

    l [ dir ]
        Load or reload a directory into the tag database, replacing the 
        currently loaded tag database.
    
    # [ tag ] [ file ]
        Tag a file loaded into the database with the given tag. If the file 
        isn't indexed by the database no tag is applied even if the file exists 
        on disk.

    d [ tag ] [ file ]
        Delete a tag on a file loaded into the database. If the tag isn't 
        already there it fails silently. The file must be indexed by the 
        database for it to try to delete the tag.
    
    p [ file ]
        Print all the tags on a file loaded into the database. If the file 
        isn't indexed by the database no tags are printed.

SEARCHING
    [ tag ] [ tag2 ] ...
        Search the database for files that all have these tags.

    ! [ tag ]
        Search the database for files that do not have this tag. Can be chained
        together with plain searching for tags.

    [ tag ] | [ tag2]
        Search the database for files that have [ tag ] or [ tag2 ]. No [ tag ]
        argument being provided is equivalent to plain searching for [ tag2 ].
";    

fn main() {
    let mut database: HashMap<String, HashSet<String>> = HashMap::new();
    let mut database_loaded = false;

    loop {
        print!("; ");
        match stdout().flush() {
            Ok(_) => (),
            Err(_error) => continue, /* try again */
        }

        let mut input = String::new();
        match stdin().read_line(&mut input) {
            Ok(_bytes) => (),
            Err(error) => {
                eprintln!("Problem reading stdin: {:?}", error);
                continue;
            }
        }

        let words: Vec<String> = input.split_whitespace()
            .map(|word| word.to_string())
            .collect();

        if words.len() == 0 {
            continue; /* prevents word[0] from throwing an error */
        }

        match words[0].as_str() {
            CMD_QUIT => {
                println!("Exiting tlsh");
                break;
            },
            CMD_HELP => {
                println!("{}", MANNUAL);
            },
            CMD_LOAD => {
                if words.len() == 2 {
                    database = match db_init(words[1].as_str()) {
                        Ok(db) => {
                            database_loaded = true;
                            println!("Database successfully initialised");
                            db
                        },
                        Err(error) => {
                            eprintln!("Database failed to initialise: {error:?}");
                            database_loaded = false;
                            continue;
                        }
                    }
                } else {
                    eprintln!("syntax error: {CMD_LOAD} [ dir ]");
                    database_loaded = false; /* comment this out to keep */
                    continue;
                }
            },
            CMD_PRINT_ALL => {
                if database_loaded == false {
                    eprintln!("No database loaded");
                    continue;
                }
                let tags = db_gettagged(&database, " ");
                for tag in tags.iter() {
                    println!("{}", tag);
                }
            },
            CMD_PRINT => {
                if database_loaded == false {
                    eprintln!("No database loaded");
                    continue;
                }
                
                if words.len() == 2 {
                    let tags = match db_readtags(&database, words[2].as_str()) {
                        Ok(tags) => tags,
                        Err(_error) => continue,
                    };
                    for tag in tags.iter() {
                        println!("{}", tag);
                    }
                } else {
                    eprintln!("syntax error: {CMD_PRINT} [ file ]");
                    continue;
                }
            },
            CMD_TAG => (),
            CMD_DELETE => (), 
            _ => {
                println!("parsing is a work in progress");
            }
        }
        
        //println!("{}", input);
        //println!("{:?}", words);
    }
}
