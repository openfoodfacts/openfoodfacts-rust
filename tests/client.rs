// Integration tests using API v1.
use reqwest::StatusCode;
use openfoodfacts::{Off, ApiVersion, Output, Locale};

#[test]
fn taxonomy() {
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let response = off.taxonomy("nova_groups").unwrap();

    assert_eq!(response.url().as_str(),
               "https://world.openfoodfacts.org/data/taxonomies/nova_groups.json");
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn taxonomy_not_found() {
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let response = off.taxonomy("not_found").unwrap();

    assert_eq!(response.url().as_str(),
               "https://world.openfoodfacts.org/data/taxonomies/not_found.json");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn facet() {
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let response = off.facet("brands", None).unwrap();

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/brands.json");
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn facet_params() {
    // Only supports the locale parameter.
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let output = Output {
        locale: Some(Locale::new("gr", None)),
        page: Some(22),
        ..Output::default()
    };
    let response = off.facet("brands", Some(output)).unwrap();
    assert_eq!(response.url().as_str(), "https://gr.openfoodfacts.org/brands.json");
}

#[test]
fn categories() {
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let response = off.categories().unwrap();

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/categories.json");
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn products_by_category() {
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let response = off.products_by_category("cheeses", None).unwrap();

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/category/cheeses.json");
    assert_eq!(response.status().is_success(), true);
}

#[test]
#[ignore]   // TODO: Fix it. The response.url does not contain the query params.
fn products_by_category_params() {
    // Only supports the locale and pagination parameters.
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let output = Output {
        locale: Some(Locale::new("fr", Some("ca"))),
        page: Some(22),
        page_size: Some(20),
        fields: Some("a,b"),
        ..Output::default()
    };
    let response = off.products_by_category("cheeses", Some(output)).unwrap();

    assert_eq!(response.url().as_str(), "https://fr-ca.openfoodfacts.org/category/cheeses.json?page=22&page_size=20");
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn products_with_additive() {
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let response = off.products_with_additive("e322-lecithins", None).unwrap();
    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/additive/e322-lecithins.json");
    assert_eq!(response.status().is_success(), true);
}

#[test]
#[ignore]   // TODO: Fix it. The response.url does not contain the query params.
fn products_with_additive_params() {
    // Only supports the locale and pagination parameters.
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let output = Output {
        locale: Some(Locale::new("fr", Some("ca"))),
        page: Some(22),
        page_size: Some(20),
        fields: Some("a,b"),
        ..Output::default()
    };
    let response = off.products_with_additive("e322-lecithins", Some(output)).unwrap();
    assert_eq!(response.url().as_str(), "https://fr-ca.openfoodfacts.org/additive/e322-lecithins.json?page=22&page_size=20");
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn products_in_state() {
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let response = off.products_in_state("empty", None).unwrap();

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/state/empty.json");
    assert_eq!(response.status().is_success(), true);
}

#[test]
#[ignore]   // TODO: Fix it. The response.url does not contain the query params.
fn products_in_state_params() {
    // Only supports the locale and pagination parameters.
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let output = Output {
        locale: Some(Locale::new("fr", Some("ca"))),
        page: Some(22),
        page_size: Some(20),
        fields: Some("a,b"),
        ..Output::default()
    };
    let response = off.products_in_state("empty", Some(output)).unwrap();

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/state/empty.json?page=22&page_size=20");
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn product_by_barcode() {
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let response = off.product_by_barcode("069000019832", None).unwrap();  // Diet Pepsi

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/api/v0/product/069000019832");
    assert_eq!(response.status().is_success(), true);
}

#[test]
#[ignore]   // TODO: Fix it. The response.url does not contain the query params.
fn product_by_barcode_params() {
    // Only supports the locale and fields parameters.
    let off = Off::new(ApiVersion::V0).build().unwrap();
    let output = Output {
        locale: Some(Locale::new("fr", Some("ca"))),
        page: Some(22),
        page_size: Some(20),
        fields: Some("a,b"),
        ..Output::default()
    };
    let response = off.product_by_barcode("069000019832", Some(output)).unwrap();  // Diet Pepsi

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/api/v0/product/069000019832?fields=a,b");
    assert_eq!(response.status().is_success(), true);
}

// Use/keep as example.
//
// use std::collections::HashMap;
// use serde_json::Value;
//
// #[test]
// fn test_off_json() {
//   let off = client().unwrap();
//   let response = off.category("cheeses", Some("gr")).unwrap();
//   println!("JSON: {:?}", response.json::<HashMap::<String, Value>>().unwrap());
// }

