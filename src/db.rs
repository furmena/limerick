use std::fs::read_dir;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;

/* === REPLACE ntfstag WITH YOUR FILESYSTEM === */
mod db::ntfs;
use ntfs::appendtag;
use ntfs::readtags;
use ntfs::deletetag;

fn fileindb(database: &HashMap<String, HashSet<String>>, filepath: &str) -> bool {
    let files = database[" "].clone();
    return files.contains(filepath);
}

pub fn db_init(dirpath: &str) -> Result<HashMap<String, HashSet<String>>> {
    let dbdir = match read_dir(dirpath) {
        Ok(files) => files,
        Err(reading_dir_error) => return Err(reading_dir_error),
    };

    /* populates the tagggedfiles vec while skipping any files that give erorrs */ 
    let mut taggedfiles = Vec::new(); 
    for entry in dbdir {
        let entry = match entry {
            Ok(entry) => entry,
            Err(entry_error) => {
                eprintln!("Problem reading entry in the db dir {dirpath}: {entry_error:?}");
                continue;
            }
        };
        
        let entry_file_type = match entry.file_type() {
            Ok(entry_file_type) => entry_file_type,
            Err(entry_file_type_error) => {
                eprintln!("Problem determining entry file type in db dir {dirpath}: {entry_file_type_error:?}");
                continue;
            }
        };

        if entry_file_type.is_file() == true {
            let file = match entry.path().to_str() {
               Some(path) => path.to_string(),
               _ => continue,
            };

            taggedfiles.push(file);
        }
    }
    
    let mut database: HashMap<String, HashSet<String>> = HashMap::new();
    for file in taggedfiles.iter() {
        let tags = match db_readtags(&database, file) {
            Ok(tags) => tags,
            Err(error) => {
                eprintln!("Problem reading tag alternate data stream for tags: {error}");
                HashSet::<String>::new()
            }
        };
        for tag in tags.iter() {
            let mut tag_hashset: HashSet<String>;
            if database.contains_key(tag) {
                tag_hashset = database[tag].clone();
            } else {
                tag_hashset = HashSet::new();
            }
            tag_hashset.insert(file.to_string());
            database.insert(tag.to_string(), tag_hashset);
        }

        let mut file_hashset: HashSet<String> = database[" "].clone();
        file_hashset.insert(file.to_string());
        database.insert(" ".to_string(), file_hashset);

    }

    return Ok(database);
}

pub fn db_gettagged(database: &HashMap<String, HashSet<String>>, tag: &str) -> HashSet<String> {
    if database.contains_key(tag) {
        return database[tag].clone();
    } else {
        return HashSet::new();
    }
}

pub fn db_deletetag(database: &HashMap<String, HashSet<String>>, filepath: &str, tag: &str) -> Result<()> {
    if fileindb(database, filepath) {
        return deletetag(filepath, tag);
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Cannot find file in the database"));
    }
}

pub fn db_readtags(database: &HashMap<String, HashSet<String>>, filepath: &str) -> Result<HashSet<String>> {
    if fileindb(database, filepath) {
        return readtags(filepath);
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Cannot find the file in the database"));
    }
}

pub fn db_appendtag(database: &HashMap<String, HashSet<String>>, filepath: &str, tag: &str) -> Result<()> {
    if fileindb(database, filepath) {
        return appendtag(filepath, tag);
    } else {
        return Err(Error::new(ErrorKind::NotFound, "Cannot find the file in the database"));
    }
}


pub fn or_tag(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String> {
    let tagged: HashSet<String> = a.union(b)
        .map(|tagged_file_path| tagged_file_path.to_owned())
        .collect();
    return tagged;
}
 

pub fn and_tag(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String> {
    let tagged: HashSet<String> = a.intersection(b)
        .map(|tagged_file_path| tagged_file_path.to_owned())
        .collect();
    return tagged;
}

pub fn not_tag(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String> {
    let tagged: HashSet<String> = a.difference(b)
        .map(|tagged_file_path| tagged_file_path.to_owned())
        .collect();
    return tagged;
}
