use std::fs::File;
use std::io::Read;

use crate::tori::parse::api_parse_after;
use crate::vahti::VahtiItem;

#[test]
fn basic_parse() {
    let mut file = File::open("testdata/tori/basic_parse.json").expect("Test data not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let expected = VahtiItem {
        deliver_to: None,
        delivery_method: None,
        site_id: crate::tori::ID,
        title: "Maalaisromanttinen peltipurkki ja eläimiä".to_string(),
        vahti_url: None,
        url: "https://www.tori.fi/vi/81076530.htm".to_string(),
        img_url: "https://images.tori.fi/api/v1/imagestori/images/9039260397.jpg?rule=medium_660"
            .to_string(),
        published: 1614890870,
        price: 7,
        seller_name: "H.S.M".to_string(),
        seller_id: 188169,
        location: "Maunula-Suursuo, Helsinki, Uusimaa".to_string(),
        ad_type: "Myydään".to_string(),
        ad_id: 79217488,
    };

    assert_eq!(
        *api_parse_after(&contents, 0).unwrap().first().unwrap(),
        expected
    );
}

#[test]
fn parse_after() {
    let mut file = File::open("testdata/tori/parse_after.json").expect("Test data not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    assert_eq!(api_parse_after(&contents, 1651416320).unwrap().len(), 1);
    assert_eq!(api_parse_after(&contents, 1651416319).unwrap().len(), 2);
}

#[test]
fn parse_multiple() {
    let mut file = File::open("testdata/tori/parse_multiple.json").expect("Test data not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut expected = vec![
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Naamiaisasu ".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/107951227.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/7574231064.jpg?rule=medium_660"
                    .to_string(),
            published: 1674035937,
            price: 25,
            seller_name: "Erja Latva".to_string(),
            seller_id: 289139,
            location: "Suvilahti, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 107463388,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Ninebot by Segway KickScooter sähköpotkulauta F25E".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/103805389.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100146113672.jpg?rule=medium_660"
                    .to_string(),
            published: 1673531834,
            price: 339,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 103120642,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Meta Quest 2 Elite hihna + akku".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/108452916.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100181065412.jpg?rule=medium_660"
                    .to_string(),
            published: 1675057180,
            price: 143,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 107987389,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Sea-doo rxt-x 300 rs".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/106281850.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100170569995.jpg?rule=medium_660"
                    .to_string(),
            published: 1674365101,
            price: 16990,
            seller_name: "Rinta-Joupin Autoliike, Tervajoki".to_string(),
            seller_id: 2349504,
            location: "Tervajoki, Laihia, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 105715838,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "ASUS PRIME Z790-P D4 ATX emolevy".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/107247726.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100171951188.jpg?rule=medium_660"
                    .to_string(),
            published: 1675853738,
            price: 268,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 106730945,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Bosch Ladattava pölynimuri BBH3ZOO28 (tornadon)".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/106947918.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100168416209.jpg?rule=medium_660"
                    .to_string(),
            published: 1675842778,
            price: 174,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 106414054,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Miele hood 90cm black".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/106692075.htm".to_string(),
            img_url: "".to_string(),
            published: 1675869730,
            price: 3329,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 106144962,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Ninebot by Segway KickScooter sähköpotkulauta E25D".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/101906085.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100134901177.jpg?rule=medium_660"
                    .to_string(),
            published: 1675853818,
            price: 402,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 101130082,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Mercury F20EPS".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/109023706.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100185031992.jpg?rule=medium_660"
                    .to_string(),
            published: 1676283023,
            price: 3700,
            seller_name: "Rinta-Joupin Autoliike, Tervajoki".to_string(),
            seller_id: 2349504,
            location: "Tervajoki, Laihia, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 108584455,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Savo hood a".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/101984681.htm".to_string(),
            img_url: "".to_string(),
            published: 1675873122,
            price: 299,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 101212772,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "E.t.m sports& casuals 52".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/109489503.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100188042265.jpg?rule=medium_660"
                    .to_string(),
            published: 1677350539,
            price: 20,
            seller_name: "moternimies".to_string(),
            seller_id: 2695759,
            location: "Vanha Vaasa, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 109060376,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "SoFlow sähköpotkulauta SOFLOW01".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/99732961.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100124634411.jpg?rule=medium_660"
                    .to_string(),
            published: 1676294880,
            price: 297,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 98836530,
        },
        VahtiItem {
            deliver_to: None,
            delivery_method: None,
            site_id: 1,
            title: "Audio Pro G10 älykäs monihuonekaiutin (vaaleanharm".to_string(),
            vahti_url: None,
            url: "https://www.tori.fi/vi/91855652.htm".to_string(),
            img_url:
                "https://images.tori.fi/api/v1/imagestori/images/100104289690.jpg?rule=medium_660"
                    .to_string(),
            published: 1677846660,
            price: 167,
            seller_name: "Gigantti outlet Vaasa".to_string(),
            seller_id: 3237298,
            location: "Asevelikylä, Vaasa, Pohjanmaa".to_string(),
            ad_type: "Myydään".to_string(),
            ad_id: 90554189,
        },
    ];

    let mut got = api_parse_after(&contents, 0).unwrap();

    expected.sort_by_key(|v| v.ad_id);
    got.sort_by_key(|v| v.ad_id);

    let _ = expected
        .iter()
        .zip(got.iter())
        .map(|(a, b)| assert_eq!(a, b))
        .collect::<Vec<_>>();
}
