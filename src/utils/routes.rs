use crate::utils::common::{Region, Game};

macro_rules! international_route {
    ($name:ident, $overseas:expr, $chinese:expr) => {
        pub const fn $name(region: Region) -> &'static str {
            match region {
                Region::Overseas => $overseas,
                Region::Chinese  => $chinese,
            }
        }
    };
}


macro_rules! game_route {
    (
        $name:ident,
        $( ($region:pat, $game:pat) => $url:expr ),+ $(,)?
    ) => {
        pub const fn $name(region: Region, game: Game) -> &'static str {
            #[allow(unreachable_patterns)]
            match (region, game) {
                $( ($region, $game) => $url ),+,
                _ => unreachable!()
            }
        }
    };
}

pub const WIKI_URL: &str = "https://sg-wiki-api.hoyolab.com/hoyowiki/wapi";
pub const GET_USER_REGION_URL: &str = "https://api-account-os.hoyoverse.com/account/binding/api/getUserGameRolesOfRegionByCookieToken";
pub const MIMO_URL: &str = "https://sg-public-api.hoyolab.com/event/e2023mimotravel";
pub const CALCULATOR_REFERER_URL: &str = "https://webstatic.mihoyo.com/ys/event/e20200923adopt_calculator/index.html";
pub const HK4E_URL: &str = "https://sg-hk4e-api.hoyoverse.com/common/hk4e_global/";
pub const ZZZ_URL: &str = "https://sg-announcement-static.hoyoverse.com/common/nap_global/";
pub const HKRPG_URL: &str = "https://sg-hkrpg-api.hoyoverse.com/common/hkrpg_global/";

pub const COOKIE_V2_REFRESH_URL: &str = "https://sg-public-api.hoyoverse.com/account/ma-passport/token/getBySToken";
pub const GET_STOKEN_BY_GAME_TOKEN_URL: &str = "https://passport-api.mihoyo.com/account/ma-cn-session/app/getTokenByGameToken";
pub const GET_COOKIE_TOKEN_BY_GAME_TOKEN_URL: &str = "https://api-takumi.mihoyo.com/auth/api/getCookieAccountInfoByGameToken";
pub const CN_GET_COOKIE_TOKEN_BY_STOKEN_URL: &str = "https://passport-api.mihoyo.com/account/auth/api/getCookieAccountInfoBySToken";

pub const APP_LOGIN_URL: &str = "https://sg-public-api.hoyoverse.com/account/ma-passport/api/appLoginByPassword";

pub const SEND_VERIFICATION_CODE_URL: &str = "https://sg-public-api.hoyoverse.com/account/ma-verifier/api/createEmailCaptchaByActionTicket";
pub const VERIFY_EMAIL_URL: &str = "https://sg-public-api.hoyoverse.com/account/ma-verifier/api/verifyActionTicketPartly";

pub const CHECK_MOBILE_VALIDITY_URL: &str = "https://webapi.account.mihoyo.com/Api/is_mobile_registrable";
pub const MOBILE_OTP_URL: &str = "https://passport-api.miyoushe.com/account/ma-cn-verifier/verifier/createLoginCaptcha";
pub const MOBILE_LOGIN_URL: &str = "https://passport-api.miyoushe.com/account/ma-cn-passport/web/loginByMobileCaptcha";

pub const CREATE_QRCODE_URL: &str = "https://passport-api.miyoushe.com/account/ma-cn-passport/web/createQRLogin";
pub const CHECK_QRCODE_URL: &str = "https://passport-api.miyoushe.com/account/ma-cn-passport/web/queryQRLoginStatus";

pub const VERIFY_MMT_URL: &str = "https://sg-public-api.hoyolab.com/event/toolcomsrv/risk/verifyGeetest";

pub const ZZZ_LEDGER_URL: &str = "https://sg-public-api.hoyolab.com/event/nap_ledger";

international_route!(web_login_url,
    "https://sg-public-api.hoyolab.com/account/ma-passport/api/webLoginByPassword",
    "https://passport-api.miyoushe.com/account/ma-cn-passport/web/loginByPassword"
);

international_route!(webstatic_url, 
    "https://operation-webstatic.hoyoverse.com/",
    "https://operation-webstatic.mihoyo.com/"
);

international_route!(webapi_url,
    "https://webapi-os.account.hoyoverse.com/Api/",
    "https://webapi.account.mihoyo.com/Api/"
);

international_route!(account_url,
    "https://api-account-os.hoyolab.com/account/auth/api",
    "https://api-takumi.mihoyo.com/account/auth/api/"
);

international_route!(bbs_url,
    "https://bbs-api-os.hoyolab.com/",
    "https://bbs-api.mihoyo.com/"
);

international_route!(bbs_referer_url,
    "https://www.hoyolab.com/",
    "https://bbs.mihoyo.com/"
);

international_route!(takumi_url,
    "https://api-os-takumi.mihoyo.com/",
    "https://api-takumi.mihoyo.com/"
);

international_route!(community_url,
    "https://bbs-api-os.hoyolab.com/community/",
    "https://api-takumi-record.mihoyo.com/community/"
);

international_route!(card_wapi_url,
    "https://bbs-api-os.hoyolab.com/game_record/card/wapi",
    "https://api-takumi-record.mihoyo.com/game_record/app/card/wapi"
);
international_route!(lineup_url,
    "https://sg-public-api.hoyoverse.com/event/simulatoros/",
    "https://api-takumi.mihoyo.com/event/platsimulator/"
);

international_route!(calculator_url,
    "https://sg-public-api.hoyoverse.com/event/calculateos/",
    "https://api-takumi.mihoyo.com/event/e20200928calculate/v1/"
);

international_route!(get_fp_url,
    "https://sg-public-data-api.hoyoverse.com/device-fp/api/getFp",
    "https://public-data-api.mihoyo.com/device-fp/api/getFp"
);

international_route!(teapot_url,
    "https://sg-hk4e-api.hoyolab.com/event/e20221121ugcos/",
    ""
);

international_route!(ysulog_url,
    "https://hk4e-api-os.hoyoverse.com/common/hk4e_self_help_query/User/",
    "https://hk4e-api.mihoyo.com/common/hk4e_self_help_query/User/"
);

international_route!(create_mmt_url,
    "https://sg-public-api.hoyolab.com/event/toolcomsrv/risk/createGeetest?is_high=true",
    "https://api-takumi-record.mihoyo.com/game_record/app/card/wapi/createVerification?is_high=false"
);

international_route!(game_risky_check_url,
    "https://api-account-os.hoyoverse.com/account/risky/api/check",
    "https://gameapi-account.mihoyo.com/account/risky/api/check"
);

international_route!(pre_grant_ticket_url,
    "https://api-account-os.hoyoverse.com/account/device/api/preGrantByTicket",
    "https://gameapi-account.mihoyo.com/account/device/api/preGrantByTicket"
);

international_route!(device_grant_url,
    "https://api-account-os.hoyoverse.com/account/device/api/grant",
    "https://gameapi-account.mihoyo.com/account/device/api/grant"
);

game_route!(
    game_login_url,
    (Region::Overseas, Game::Genshin) => "https://hk4e-sdk-os.hoyoverse.com/hk4e_global/combo/granter/login/v2/login",
    (Region::Overseas, Game::Honkai) => "https://bh3-sdk-os.hoyoverse.com/bh3_os/combo/granter/login/v2/login",
    (Region::Overseas, Game::Starrail) => "https://hkrpg-sdk-os.hoyoverse.com/hkrpg_global/combo/granter/login/v2/login",
    (Region::Overseas, Game::ZZZ) => "https://nap-sdk-os.hoyoverse.com/nap_global/combo/granter/login/v2/login",

    (Region::Chinese, Game::Genshin) => "https://hk4e-sdk.mihoyo.com/hk4e_cn/combo/granter/login/v2/login",
    (Region::Chinese, Game::Honkai) => "https://api-sdk.mihoyo.com/bh3_cn/combo/granter/login/v2/login",
    (Region::Chinese, Game::Starrail) => "https://hkrpg-sdk.mihoyo.com/hkrpg_cn/combo/granter/login/v2/login",
    (Region::Chinese, Game::ZZZ) => "https://nap-sdk.mihoyo.com/nap_cn/combo/granter/login/v2/login",
);

game_route!(
    shield_login_url,
    (Region::Overseas, Game::Genshin) => "https://hk4e-sdk-os.hoyoverse.com/hk4e_global/mdk/shield/api/login",
    (Region::Overseas, Game::Honkai) => "https://bh3-sdk-os.hoyoverse.com/bh3_os/mdk/shield/api/login",
    (Region::Overseas, Game::Starrail) => "https://hkrpg-sdk-os.hoyoverse.com/hkrpg_global/mdk/shield/api/login",
    (Region::Overseas, Game::ZZZ) => "https://nap-sdk-os.hoyoverse.com/nap_global/mdk/shield/api/login",

    (Region::Chinese, Game::Genshin) => "https://hk4e-sdk.mihoyo.com/hk4e_cn/mdk/shield/api/login",
    (Region::Chinese, Game::Honkai) => "https://api-sdk.mihoyo.com/bh3_cn/mdk/shield/api/login",
    (Region::Chinese, Game::Starrail) => "https://hkrpg-sdk.mihoyo.com/hkrpg_cn/mdk/shield/api/login",
    (Region::Chinese, Game::ZZZ) => "https://nap-sdk.mihoyo.com/nap_cn/mdk/shield/api/login",
);

game_route!(
    record_url,
    (Region::Overseas, Game::Genshin) => "https://sg-public-api.hoyolab.com/event/game_record/genshin/api",
    (Region::Overseas, Game::Starrail) => "https://bbs-api-os.hoyolab.com/game_record/hkrpg/api",
    (Region::Overseas, Game::Honkai) => "https://bbs-api-os.hoyolab.com/game_record/honkai3rd/api",
    (Region::Overseas, Game::ZZZ) => "https://sg-act-nap-api.hoyolab.com/event/game_record_zzz/api/zzz",

    (Region::Chinese, Game::Genshin) => "https://api-takumi-record.mihoyo.com/game_record/app/genshin/api",
    (Region::Chinese, Game::Starrail) => "https://api-takumi-record.mihoyo.com/game_record/app/hkrpg/api",
    (Region::Chinese, Game::Honkai) => "https://api-takumi-record.mihoyo.com/game_record/app/honkai3rd/api",
    (Region::Chinese, Game::ZZZ) => "https://api-takumi-record.mihoyo.com/event/game_record_zzz/api/zzz",
);

game_route!(
    reward_url,
    (Region::Overseas, Game::Genshin) => "https://sg-hk4e-api.hoyolab.com/event/sol?act_id=e202102251931481",
    (Region::Overseas, Game::Honkai) => "https://sg-public-api.hoyolab.com/event/mani?act_id=e202110291205111",
    (Region::Overseas, Game::Starrail) => "https://sg-public-api.hoyolab.com/event/luna/os?act_id=e202303301540311",
    (Region::Overseas, Game::ZZZ) => "https://sg-act-nap-api.hoyolab.com/event/luna/zzz/os?act_id=e202406031448091",
    (Region::Overseas, Game::Tot) => "https://sg-public-api.hoyoverse.com/event/luna/os?act_id=e202202281857121",

    (Region::Chinese, Game::Genshin) => "https://api-takumi.mihoyo.com/event/luna/?act_id=e202311201442471",
    (Region::Chinese, Game::Honkai) => "https://api-takumi.mihoyo.com/event/luna/?act_id=e202306201626331",
    (Region::Chinese, Game::Starrail) => "https://api-takumi.mihoyo.com/event/luna/?act_id=e202304121516551",
    (Region::Chinese, Game::ZZZ) => "https://api-takumi.mihoyo.com/event/luna/zzz?act_id=e202406242138391",
    (Region::Chinese, Game::Tot) => "https://api-takumi.mihoyo.com/event/luna?act_id=e202202251749321"
);

game_route!(
    code_url,
    (Region::Overseas, Game::Genshin) => "https://sg-hk4e-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkey",
    (Region::Overseas, Game::Starrail) => "https://sg-hkrpg-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkeyRisk",
    (Region::Overseas, Game::ZZZ) => "https://public-operation-nap.hoyoverse.com/common/apicdkey/api/webExchangeCdkey",
    (Region::Overseas, Game::Tot) => "https://sg-public-api.hoyoverse.com/common/apicdkey/api/webExchangeCdkey",
);

game_route!(
    gacha_url,
    (Region::Overseas, Game::Genshin) => "https://public-operation-hk4e-sg.hoyoverse.com/gacha_info/api/",
    (Region::Overseas, Game::Starrail) => "https://public-operation-hkrpg-sg.hoyoverse.com/common/gacha_record/api/",
    (Region::Overseas, Game::ZZZ) => "https://public-operation-nap-sg.hoyoverse.com/common/gacha_record/api/",

    (Region::Chinese, Game::Genshin) => "https://public-operation-hk4e.mihoyo.com/gacha_info/api/",
    (Region::Chinese, Game::Starrail) => "https://public-operation-hkrpg.mihoyo.com/common/gacha_record/api/",
    (Region::Chinese, Game::ZZZ) => "https://public-operation-nap.mihoyo.com/common/gacha_record/api/",
);

game_route!(
    info_ledger_url,
    (Region::Overseas, Game::Genshin) => "https://sg-hk4e-api.hoyolab.com/event/ysledgeros/month_info",
    (Region::Overseas, Game::Starrail) => "https://sg-public-api.hoyolab.com/event/srledger/month_info",

    (Region::Chinese, Game::Genshin) => "https://hk4e-api.mihoyo.com/event/ys_ledger/monthInfo",
    (Region::Chinese, Game::Starrail) => "https://api-takumi.mihoyo.com/event/srledger/month_info",
);

game_route!(
    detail_ledger_url,
    (Region::Overseas, Game::Genshin) => "https://sg-hk4e-api.hoyolab.com/event/ysledgeros/month_detail",
    (Region::Overseas, Game::Starrail) => "https://sg-public-api.hoyolab.com/event/srledger/month_detail",

    (Region::Chinese, Game::Genshin) => "https://hk4e-api.mihoyo.com/event/ys_ledger/monthDetail",
    (Region::Chinese, Game::Starrail) => "https://api-takumi.mihoyo.com/event/srledger/month_detail",
);
