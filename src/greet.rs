pub mod greet {

    const SPANISH: &str = "spanish";
    const FRENCH: &str = "french";
    const HELLO_IN_ENGLISH: &str = "Hello, ";
    const HELLO_IN_SPANISH: &str = "Hola, ";
    const HELLO_IN_FRENCH: &str = "Bonjor, ";

    pub fn hello(mut name: &str, lang: &str) -> String {
        if name == "" {
            name = "World";
        }
        let mut greet: String;
        match lang {
            SPANISH => greet = String::from(HELLO_IN_SPANISH),
            FRENCH => greet = String::from(HELLO_IN_FRENCH),
            _ => greet = String::from(HELLO_IN_ENGLISH),
        }
        greet.push_str(name);
        return greet;
    }
}
