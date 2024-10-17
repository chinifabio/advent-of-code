use std::str::FromStr;

pub trait Parser {
    fn to_vec<T: FromStr>(self, separator: &str) -> Vec<T>;
}

impl Parser for &str {
    fn to_vec<T: FromStr>(self, separator: &str) -> Vec<T> {
        self.split(separator)
            .filter_map(|x| x.parse::<T>().ok())
            .collect()
    }
}
