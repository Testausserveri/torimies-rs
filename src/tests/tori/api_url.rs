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
    let expected = API_BASE.to_owned() + "&q=thinkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_non_ascii() {
    let url = "https://www.tori.fi/koko_suomi?q=th%F6nkpad";
    let expected = API_BASE.to_owned() + "&q=thönkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_category() {
    let url = "https://www.tori.fi/koko_suomi?q=&cg=2030";
    let expected = API_BASE.to_owned() + "&q=&category=2030";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_0_category() {
    let url = "https://www.tori.fi/koko_suomi?q=&cg=0";
    let expected = API_BASE.to_owned() + "&q=";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_price_range() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&ps=2&pe=4";
    let expected = API_BASE.to_owned() + "&q=thinkpad&suborder=50-100";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn price_range_no_start() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&pe=5";
    let expected = API_BASE.to_owned() + "&q=thinkpad&suborder=-250";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn price_range_no_end() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&ps=6";
    let expected = API_BASE.to_owned() + "&q=thinkpad&suborder=500-";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_ad_type() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&cg=0&st=s&st=g";
    let expected = API_BASE.to_owned() + "&q=thinkpad&ad_type=s&ad_type=g";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_w() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&w=3";
    let expected = API_BASE.to_owned() + "&q=thinkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_w_region() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&w=104";
    let expected = API_BASE.to_owned() + "&q=thinkpad&region=4";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_area() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&m=7";
    let expected = API_BASE.to_owned() + "&q=thinkpad&area=7";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_caregion() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&ca=10";
    let expected = API_BASE.to_owned() + "&q=thinkpad&region=10";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_ca_and_w() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&w=104&ca=10";
    let expected = API_BASE.to_owned() + "&q=thinkpad&region=4";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_no_argument_name() {
    let url = "https://www.tori.fi/koko_suomi?q=thinkpad&=69";
    let expected = API_BASE.to_owned() + "&q=thinkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn query_with_different_base() {
    let url = "https://www.tori.fi/lappi?q=thinkpad";
    let expected = API_BASE.to_owned() + "&q=thinkpad";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn multiquery1() {
    let url = "https://www.tori.fi/pohjanmaa?q=yoga-matto&cg=0&w=1&st=s&st=k&st=u&st=h&st=g&ca=5&l=0&md=th";
    let expected = API_BASE.to_owned()
        + "&q=yoga-matto&ad_type=s&ad_type=k&ad_type=u&ad_type=h&ad_type=g&region=5&l=0&md=th";
    assert_eq!(expected, vahti_to_api(url));
}

#[test]
fn multiquery2() {
    let url = "https://www.tori.fi/uusimaa?q=vinkulelu+koiralle&cg=0&w=1&st=s&st=k&st=u&st=h&st=g&ca=18&l=0&md=th";
    let expected =
        API_BASE.to_owned() + "&q=vinkulelu+koiralle&ad_type=s&ad_type=k&ad_type=u&ad_type=h&ad_type=g&region=18&l=0&md=th";
    assert_eq!(expected, vahti_to_api(url));
}
