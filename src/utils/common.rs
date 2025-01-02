#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Region {
    Overseas,
    Chinese,
}

impl ToString for Region {
    fn to_string(&self) -> String {
        match self {
            Self::Overseas => "Overseas".to_string(),
            Self::Chinese => "Chinese".to_string(),
        }
    }
}

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Game {
    Genshin,
    Honkai,
    Starrail,
    ZZZ,
    Tot,
}

macro_rules! headermap {
    ($( $key:expr => $value:expr ),* $(,)?) => {{
        use reqwest::header;
        let mut map = header::HeaderMap::new();
        $(
            map.insert($key, header::HeaderValue::from_str($value).unwrap());
        )*
        map
    }};
}
pub(crate) use headermap;
