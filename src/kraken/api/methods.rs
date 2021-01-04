pub enum Method {
    Balance,
    Time,
    Assets,
}

impl From<Method> for &str {
    fn from(m: Method) -> Self {
        match m {
            Method::Balance => "Balance",
            Method::Time => "Time",
            Method::Assets => "Assets",
        }
    }
}
