pub mod dictionary {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;
    pub const ERROR_NOT_FOUND: Result<&str, &str> =
        Err("could not found the word you were looking for");

    pub fn search<'a>(
        dictionary: &'a HashMap<&'a str, &'a str>,
        key: &str,
    ) -> Result<&'a str, &'a str> {
        match dictionary.get(key) {
            Some(value) => Ok(value),
            None => ERROR_NOT_FOUND,
        }
    }
    pub fn add<'a>(
        dictionary: &'a mut HashMap<&'a str, &'a str>,
        key: &'a str,
        value: &'a str,
    ) -> Result<(), &'static str> {
        match dictionary.entry(key) {
            Entry::Occupied(_entry) => {
                return Err("cannot add word because it already exists");
            }
            Entry::Vacant(dictionary) => {
                dictionary.insert(value);
                Ok(())
            }
        }
    }
}
