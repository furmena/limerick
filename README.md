# Limerick

Limerick is a rust library (crate) for reading and writing metadata in the form of tags for all files. Currently, it comes prepackaged with a rudimentary shell in `bin/shell.rs` for managing tags. This library is moreso a learning experince for a project I might actually use day to day, so don't expect me to merge pull requests.

## Details
Currently, limerick is a windows only library. This is because to apply metadata to any file format, regardless if the file format supports extra metadata it relies on NTFS feature called ADS (alternate data streams).

## Adding support for other platforms
For any developers that want support on their favourite platform create a file that implements these functions, something like `apfs.rs`, `bfs.rs`, `ext4.rs`, etc. Then patch `lib.rs` on lines 9-12 to use your own filesystem. This does mean that for people with multiple filesystems you would need to complile this library twice (yikes!). Maybe I'll add a filesystem check into the `db_init()` function in `lib.rs`?
* `pub fn readtags(filepath: &str) -> Result<HashSet<String>>`
* `pub fn deletetag(filepath: &str, tag: &str) -> Result<()>`
* `pub fn appendtag(filepath: &str, tag: &str) -> Result<()>`

The `readtags()` function should return all the tags. Tags should be seperated by whitespace meaning that:
```
tag1
tag2 tag3
tag4    tag6
    tag7
```
Should return `{ "tag1", "tag2", "tag3", "tag4", "tag5", "tag6", "tag7", }`. The deleting function should remove the tag from the metadata store while exiting early if the tag doesn't exist. The appending function should check if the tag exists already before writing to make sure there are no duplicates in the metadata store. Any errors *should* be propagated upwards but I'm a little hypocritcal about this since writing ignores any erors.

## Usage
tldr; `cat db.rs | grep "^pub fn"`.
- `pub fn db_init(dirpath: &str) -> Result<HashMap<String, HashSet<String>>>`
This function returns a result of a HashMap of tags with the key being the tags themself and values being a HashSet of paths of files with that tag. There is one exception though with the `" "` key being a HashSet of all the files in the database.

- `pub fn db_gettagged(database: &HashMap<String, HashSet<String>>, tag: &str) -> HashSet<String>` 
This returns a HashSet of files with a certain tag, returning a empty HashSet if there are no files with that tag.

These functions are wrappers over the `filesystem.rs` functions that includes a check to make sure that the database has loaded that file into itself before using the `filesystem.rs` function.

- `pub fn db_deletetag(database: &HashMap<String, HashSet<String>>, filepath: &str, tag: &str) -> Result<()>`

- `pub fn db_readtags(database: &HashMap<String, HashSet<String>>, filepath: &str) -> Result<HashSet<String>>` 

- `pub fn db_appendtag(database: &HashMap<String, HashSet<String>>, filepath: &str, tag: &str) -> Result<()>` 

These functions wrappers over `HashSet.union()`, `HashSet.intersection()`, and `HashSet.difference()`. There is no real difference from just using the wrapped functions. I added these purely for completion percent. 

- `pub fn or_tag(a: &HashSet<String>, b: &HashSet<String>) -> - HashSet<String>`

- `pub fn and_tag(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String>`

- `pub fn not_tag(a: &HashSet<String>, b: &HashSet<String>) -> HashSet<String>` 

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.