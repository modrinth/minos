use std::str::FromStr;

pub fn parse_var<T: FromStr>(var: &'static str) -> Option<T> {
    dotenvy::var(var).ok().and_then(|i| i.parse().ok())
}
