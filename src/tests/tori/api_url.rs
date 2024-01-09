use super::API_BASE;
use crate::tori::api::vahti_to_api;

#[test]
fn no_keyword() {
    let url = "https://www.tori.fi/koko_suomi?";
    let expected = API_BASE.to_owned();
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn basic_query() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad";
    let expected = API_BASE.to_owned() + "q=thinkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_non_ascii() {
    let url = "https://www.tori.fi/koko_suomi?q=th%F6nkpad";
    let expected = API_BASE.to_owned() + "q=thönkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_category() {
    let url = "https://www.tori.fi/koko_suomi?q=&cg=2030";
    let expected = API_BASE.to_owned() + "q=&category=2030";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_0_category() {
    let url = "https://www.tori.fi/koko_suomi?q=&cg=0";
    let expected = API_BASE.to_owned() + "q=";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_price_range() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&ps=2&pe=4";
    let expected = API_BASE.to_owned() + "q=thinkpad&suborder=50-100";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn price_range_no_start() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&pe=5";
    let expected = API_BASE.to_owned() + "q=thinkpad&suborder=-250";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn price_range_no_end() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&ps=6";
    let expected = API_BASE.to_owned() + "q=thinkpad&suborder=500-";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_ad_type() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&cg=0&st=s&st=g";
    let expected = API_BASE.to_owned() + "q=thinkpad&ad_type=s&ad_type=g";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_w() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&w=3";
    let expected = API_BASE.to_owned() + "q=thinkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_w_region() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&w=104";
    let expected = API_BASE.to_owned() + "q=thinkpad&region=4";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_area() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&m=7";
    let expected = API_BASE.to_owned() + "q=thinkpad&area=7";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_ca() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&ca=10";
    let expected = API_BASE.to_owned() + "q=thinkpad&region=10";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_ca_and_w() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&w=104&ca=10";
    let expected = API_BASE.to_owned() + "q=thinkpad&region=4";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_no_argument_name() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&=69";
    let expected = API_BASE.to_owned() + "q=thinkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_different_base() {
    let url = "https://www.tori.fi/lappi?q=thinkpad";
    let expected = API_BASE.to_owned() + "q=thinkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn multiquery1() {
    let url =
        "https://www.tori.fi/pohjanmaa?q=yoga-matto&cg=0&w=1&st=s&st=k&st=u&st=h&st=g&l=0&md=th";
    let expected =
        API_BASE.to_owned() + "q=yoga-matto&ad_type=s&ad_type=k&ad_type=u&ad_type=h&ad_type=g";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn multiquery2() {
    let url = "https://www.tori.fi/uusimaa?q=vinkulelu+koiralle&cg=0&w=1&st=s&st=k&st=u&st=h&st=g&l=0&md=th";
    let expected = API_BASE.to_owned()
        + "q=vinkulelu+koiralle&ad_type=s&ad_type=k&ad_type=u&ad_type=h&ad_type=g";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_gets_decoded() {
    let url = "https://www.tori.fi/koko_suomi?q=th%E4nkpad";
    let expected = API_BASE.to_owned() + "q=thänkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn category_from_cg() {
    let url = "https://www.tori.fi/koko_suomi?cg=5000";
    let expected = API_BASE.to_owned() + "category=5000";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn zero_category_is_ignored() {
    let url = "https://www.tori.fi/koko_suomi?cg=0";
    let expected = API_BASE.to_owned();
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn c_is_prioritized_over_cg() {
    let url1 = "https://www.tori.fi/koko_suomi?cg=5010&c=5012";
    let url2 = "https://www.tori.fi/koko_suomi?c=5012&cg=5010";
    let expected = API_BASE.to_owned() + "category=5012";

    assert_eq!(expected, vahti_to_api(url1));
    assert_eq!(expected, vahti_to_api(url2));
}

#[test]
fn ca_region() {
    let url = "https://www.tori.fi/li?ca=1";
    let expected = API_BASE.to_owned() + "region=1";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn w_region() {
    let url = "https://www.tori.fi/li?w=101";
    let expected = API_BASE.to_owned() + "region=1";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn w_is_prioritized_over_ca() {
    let url1 = "https://www.tori.fi/koko_suomi?w=105&ca=1";
    let url2 = "https://www.tori.fi/koko_suomi?ca=1&w=105";
    let expected = API_BASE.to_owned() + "region=5";

    assert_eq!(expected, vahti_to_api(url1));
    assert_eq!(expected, vahti_to_api(url2));
}

#[test]
fn company_ad() {
    let url = "https://www.tori.fi/li?f=c";
    let expected = API_BASE.to_owned() + "company_ad=1";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn private_ad() {
    let url = "https://www.tori.fi/li?f=p";
    let expected = API_BASE.to_owned() + "company_ad=0";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn both_company_and_private_ads() {
    let url = "https://www.tori.fi/li?f=a";
    let expected = API_BASE.to_owned();
    assert_eq!(expected, vahti_to_api(url));
}
