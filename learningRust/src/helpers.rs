pub mod name_helpers {
    pub fn get_fullname(first: String, last: String) -> String {
        let full_name: String = format!("{0} {1}", first, last);
        full_name
    }
}
