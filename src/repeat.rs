pub mod repeat {

    pub fn repeat(chara: &str, times: i8) -> String {
        let mut count = 0;
        let mut repeated: String = String::from("");
        while count < times {
            repeated.push_str(chara);
            count += 1;
        }
        return repeated;
    }
}
