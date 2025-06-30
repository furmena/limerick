use std::collections::HashSet;
use std::io::Result;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;
use std::io::Read;

const TAG_ADS: &str = "TAGLIGHT_TAGS";

enum FileTaggingMode {
    Append,
    Delete,
    Read,
}

fn gettagfile(filepath: &str, filemode: FileTaggingMode) -> Result<File> {
    let mut file = OpenOptions::new();
    match filemode {
        FileTaggingMode::Append => file.read(true).append(true),
        FileTaggingMode::Delete => file.read(true).write(true),
        FileTaggingMode::Read => file.read(true),
    };

    let tagfile_name = format!("{filepath}:{TAG_ADS}");
    let tagfile_result = file.open(&tagfile_name);
    let tagfile = match tagfile_result {
        Ok(opened_file) => opened_file,
        Err(opening_error) => {
            eprintln!("Problem opening the {TAG_ADS} alternate data stream for tags: {opening_error:?}");
            return Err(opening_error);
        }
    };

    return Ok(tagfile);
}

fn readfile(file: &mut File) -> String {
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => return contents,
        Err(reading_error) => {
            eprintln!("Problem reading the {TAG_ADS} alternate data stream for tags: {reading_error:?}");
            return String::new();
        }
    }
}

fn stringtotags(file_contents: String) -> HashSet<String> {
    let mut tags: HashSet<String> = HashSet::new();
    for tag in file_contents.split_whitespace() {
        tags.insert(tag.to_owned());
    }
    
    return tags;
}

pub fn deletetag(filepath: &str, tag: &str) -> Result<()> {
    let mut tagfile = match gettagfile(filepath, FileTaggingMode::Delete) {
        Ok(file) => file,
        Err(deleting_error) => return Err(deleting_error),
    };

    let file_contents = readfile(&mut tagfile);
    let tags = stringtotags(file_contents);
    if tag.contains(tag) {
        for tag in tags.iter() {
            let _ = tagfile.write(format!("\n{tag}").as_bytes());
        }
        return Ok(());
    } else {
        return Ok(());
    }
}

pub fn appendtag(filepath: &str, tag: &str) -> Result<()> {
    let mut tagfile = match gettagfile(filepath, FileTaggingMode::Append) {
        Ok(file) => file,
        Err(_appending_error) => {
            let mut newfile = match File::create(format!("{filepath}:{TAG_ADS}")) {
                Ok(file) => file,
                Err(creating_error) => return Err(creating_error),
            };
            let _ = newfile.write(tag.as_bytes());
            return Ok(());
        },
    };
    
    let file_contents = readfile(&mut tagfile);
    let tags: HashSet<String> = stringtotags(file_contents);
    if tags.contains(tag) {
        return Ok(());
    } else {
        let _ = tagfile.write(format!("\n{tag}").as_bytes());
        return Ok(());
    }
}

pub fn readtags(filepath: &str) -> Result<HashSet<String>> {
    let mut tagfile = match gettagfile(filepath, FileTaggingMode::Read) {
        Ok(file) => file,
        Err(error) => return Err(error),
    };
    let file_contents = readfile(&mut tagfile);
    return Ok(stringtotags(file_contents));
}
