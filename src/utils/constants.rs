use chrono::FixedOffset;
use std::sync::LazyLock;

use crate::{
    utils::common::{Game, Region},
    HoyoError,
};

#[derive()]
pub enum Lang {
    ZhCn,
    ZhTw,
    DeDe,
    EnUs,
    EsEs,
    FrFr,
    IdId,
    ItIt,
    JaJp,
    KoKr,
    PtPt,
    RuRu,
    ThTh,
    ViVn,
    TrTr,
}

impl Lang {
    /// get string val
    pub const fn val(&self) -> &'static str {
        match self {
            Self::ZhCn => "zh-cn",
            Self::ZhTw => "zh-tw",
            Self::DeDe => "de-de",
            Self::EnUs => "en-us",
            Self::EsEs => "es-es",
            Self::FrFr => "fr-fr",
            Self::IdId => "id-id",
            Self::ItIt => "it-it",
            Self::JaJp => "ja-jp",
            Self::KoKr => "ko-kr",
            Self::PtPt => "pt-pt",
            Self::RuRu => "ru-ru",
            Self::ThTh => "th-th",
            Self::ViVn => "vi-vn",
            Self::TrTr => "tr-tr",
        }
    }
    /// 2 letter language code.
    pub const fn val_2l(&self) -> &'static str {
        match self {
            Self::ZhCn => "zh",
            Self::ZhTw => "zh",
            Self::DeDe => "de",
            Self::EnUs => "en",
            Self::EsEs => "es",
            Self::FrFr => "fr",
            Self::IdId => "id",
            Self::ItIt => "it",
            Self::JaJp => "ja",
            Self::KoKr => "ko",
            Self::PtPt => "pt",
            Self::RuRu => "ru",
            Self::ThTh => "th",
            Self::ViVn => "vi",
            Self::TrTr => "tr",
        }
    }
}

impl TryFrom<&str> for Lang {
    type Error = HoyoError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "zh-cn" => Ok(Self::ZhCn),
            "zh-tw" => Ok(Self::ZhTw),
            "de-de" => Ok(Self::DeDe),
            "en-us" => Ok(Self::EnUs),
            "es-es" => Ok(Self::EsEs),
            "fr-fr" => Ok(Self::FrFr),
            "id-id" => Ok(Self::IdId),
            "it-it" => Ok(Self::ItIt),
            "ja-jp" => Ok(Self::JaJp),
            "ko-kr" => Ok(Self::KoKr),
            "pt-pt" => Ok(Self::PtPt),
            "ru-ru" => Ok(Self::RuRu),
            "th-th" => Ok(Self::ThTh),
            "vi-vn" => Ok(Self::ViVn),
            "tr-tr" => Ok(Self::TrTr),
            _ => Err(HoyoError::LanguageParse),
        }
    }
}

pub enum DsSalt {
    Overseas,
    Chinese,
    AppLogin,
    CnSignin,
    CnPassport,
}

impl DsSalt {
    pub const fn val(&self) -> &'static str {
        match self {
            Self::Overseas => "6s25p5ox5y14umn1p61aqyyvbvvl3lrt",
            Self::Chinese => "xV8v4Qu54lUKrEYFZkJhB8cuOh9Asafs",
            Self::AppLogin => "IZPgfb0dRPtBeLuFkdDznSZ6f4wWt6y2",
            Self::CnSignin => "LyD1rXqMv2GJhnwdvCBjFOKGiKuLY3aO",
            Self::CnPassport => "JwYDpKvLj6MrMqqYU6jTKF17KNO2PXoS",
        }
    }
}

impl From<Region> for DsSalt {
    fn from(value: Region) -> Self {
        match value {
            Region::Overseas => Self::Overseas,
            Region::Chinese => Self::Chinese,
        }
    }
}

pub(crate) const GEETEST_RETCODES: [u32; 4] = [10035, 5003, 10041, 1034];

pub(crate) const fn get_app_keys(game: Game, region: Region) -> &'static str {
    match (game, region) {
        (Game::Genshin, Region::Overseas) => "6a4c78fe0356ba4673b8071127b28123",
        (Game::Genshin, Region::Chinese) => "d0d3a7342df2026a70f650b907800111",

        (Game::Starrail, Region::Overseas) => "d74818dabd4182d4fbac7f8df1622648",
        (Game::Starrail, Region::Chinese) => "4650f3a396d34d576c3d65df26415394",

        (Game::Honkai, Region::Overseas) => "243187699ab762b682a2a2e50ba02285",
        (Game::Honkai, Region::Chinese) => "0ebc517adb1b62c6b408df153331f9aa",

        (Game::ZZZ, Region::Overseas) => "ff0f2776bf515d79d1f8ff1fb98b2a06",
        (Game::ZZZ, Region::Chinese) => "4650f3a396d34d576c3d65df26415394",
        _ => unreachable!(),
    }
}

pub(crate) const fn get_app_ids(game: Game, region: Region) -> &'static str {
    match (game, region) {
        (Game::Genshin, Region::Overseas) => "4",
        (Game::Genshin, Region::Chinese) => "4",

        (Game::Starrail, Region::Overseas) => "11",
        (Game::Starrail, Region::Chinese) => "8",

        (Game::Honkai, Region::Overseas) => "8",
        (Game::Honkai, Region::Chinese) => "1",

        (Game::ZZZ, Region::Overseas) => "15",
        (Game::ZZZ, Region::Chinese) => "12",
        _ => unreachable!(),
    }
}

pub(crate) const fn get_geetest_record_keys(game: Game) -> &'static str {
    match game {
        Game::Genshin => "genshin_game_record",
        Game::Starrail => "hkrpg_game_record",
        Game::Honkai => "bh3_game_record",
        Game::ZZZ => "nap_game_record",
        _ => unreachable!(),
    }
}

pub(crate) const fn get_bizs(game: Game, region: Region) -> &'static str {
    match (game, region) {
        (Game::Genshin, Region::Overseas) => "hk4e_global",
        (Game::Genshin, Region::Chinese) => "hk4e_cn",

        (Game::Starrail, Region::Overseas) => "hkrpg_global",
        (Game::Starrail, Region::Chinese) => "hkrpg_cn",

        (Game::Honkai, Region::Overseas) => "bh3_os",
        (Game::Honkai, Region::Chinese) => "bh3_cn",

        (Game::ZZZ, Region::Overseas) => "nap_global",
        (Game::ZZZ, Region::Chinese) => "nap_cn",
        _ => unreachable!(),
    }
}

pub const CN_TIMEZONE: LazyLock<FixedOffset> =
    LazyLock::new(|| FixedOffset::east_opt(8 * 3600).unwrap());
