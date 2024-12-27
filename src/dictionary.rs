pub mod dictionary {
    use std::collections::hash_map::Entry;
    use std::collections::HashMap;

    pub const ERROR_NOT_FOUND: Result<&str, &str> =
        Err("could not found the word you were looking for");
    pub const ERROR_WORD_EXIST: Result<String, &str> =
        Err("cannot add word because it already exists");
    pub const ERROR_WORD_NOT_EXIST: Result<String, &str> = Err("the word does not exist");

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
    ) -> Result<String, &'static str> {
        match dictionary.entry(key) {
            Entry::Occupied(_entry) => {
                return ERROR_WORD_EXIST;
            }
            Entry::Vacant(dictionary) => {
                dictionary.insert(value);
                Ok(format!("definition {} added for key {}", value, key))
            }
        }
    }
    pub fn update<'a>(
        dictionary: &'a mut HashMap<&'a str, &'a str>,
        key: &'a str,
        value: &'a str,
    ) -> Result<String, &'static str> {
        match dictionary.entry(key) {
            Entry::Occupied(_entry) => {
                dictionary.insert(key, value);
                let response = format!("updated {} to {}", key, value);
                return Ok(response);
            }

            Entry::Vacant(_entry) => return ERROR_WORD_NOT_EXIST,
        }
    }
    pub fn delete<'a>(
        dictionary: &'a mut HashMap<&'a str, &'a str>,
        key: &'a str,
    ) -> Result<String, &'static str> {
        match dictionary.entry(key) {
            Entry::Occupied(_entry) => {
                dictionary.remove(key);
                return Ok(format!("the definition and key {} removed", key));
            }
            Entry::Vacant(_entry) => {
                return ERROR_WORD_NOT_EXIST;
            }
        }
    }
}
