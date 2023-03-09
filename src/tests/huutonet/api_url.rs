use super::API_BASE;
use crate::huutonet::api::vahti_to_api;

#[test]
fn no_keyword() {
    let url = "https://www.huuto.net/haku?words=&area=";
    let expected = API_BASE.to_owned() + "words=&area=&sort=newest";
    assert_eq!(vahti_to_api(url), expected);
}

#[test]
fn basic_query() {
    let url = "https://www.huuto.net/haku?words=thinkpad&area=";
    let expected = API_BASE.to_owned() + "words=thinkpad&area=&sort=newest";
    assert_eq!(vahti_to_api(url), expected);
}

#[test]
fn slash_query() {
    let url = "https://www.huuto.net/haku/words/thinkpad";
    let expected = API_BASE.to_owned() + "words=thinkpad&sort=newest";
    assert_eq!(vahti_to_api(url), expected);
}

#[test]
fn query_with_non_ascii() {
    let url = "https://www.huuto.net/haku?words=th%C3%B6nkp%C3%A4d";
    let slash_url = "https://www.huuto.net/haku/words/th%C3%B6nkp%C3%A4d";
    let expected = API_BASE.to_owned() + "words=th%C3%B6nkp%C3%A4d&sort=newest";

    assert_eq!(vahti_to_api(url), expected);
    assert_eq!(vahti_to_api(slash_url), expected);
}

#[test]
fn multiquery1() {
    let url = "https://www.huuto.net/haku?words=thinkpad&classification=new&area=uusimaa";
    let slash_url = "https://www.huuto.net/haku/words/thinkpad/classification/new/area/uusimaa";
    let expected =
        API_BASE.to_owned() + "words=thinkpad&classification=new&area=uusimaa&sort=newest";

    assert_eq!(vahti_to_api(url), expected);
    assert_eq!(vahti_to_api(slash_url), expected);
}

#[test]
fn multiquery2() {
    let url = "https://www.huuto.net/haku?sort=lowprice&category=502";
    let slash_url = "https://www.huuto.net/haku/sort/lowprice/category/502";
    let expected = API_BASE.to_owned() + "sort=lowprice&category=502&sort=newest";

    assert_eq!(vahti_to_api(url), expected);
    assert_eq!(vahti_to_api(slash_url), expected);
}
