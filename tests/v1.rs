// Integration tests using API v1. Note that all integration tests are ignored
// by default.
use reqwest::StatusCode;
use openfoodfacts::Off;


#[test]
#[ignore]
fn taxonomy() {
    let off = Off::new("v0").build().unwrap();
    let response = off.taxonomy("nova_groups").unwrap();

    assert_eq!(response.url().as_str(),
               "https://world.openfoodfacts.org/data/taxonomies/nova_groups.json");
    assert_eq!(response.status().is_success(), true);
}


#[test]
#[ignore]
fn taxonomy_not_found() {
    let off = Off::new("v0").build().unwrap();
    let response = off.taxonomy("not_found").unwrap();

    assert_eq!(response.url().as_str(),
               "https://world.openfoodfacts.org/data/taxonomies/not_found.json");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}


#[test]
#[ignore]
fn facet() {
    let off = Off::new("v0").build().unwrap();
    let response = off.facet("brands", Some("gr")).unwrap();

    assert_eq!(response.url().as_str(), "https://gr.openfoodfacts.org/brands.json");
    assert_eq!(response.status().is_success(), true);
}


#[test]
#[ignore]
fn categories() {
    let off = Off::new("v0").build().unwrap();
    let response = off.categories().unwrap();

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/categories.json");
    assert_eq!(response.status().is_success(), true);
}


#[test]
#[ignore]
fn products_by_category() {
    let off = Off::new("v0").build().unwrap();
    let response = off.products_by_category("cheeses", None).unwrap();

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/category/cheeses.json");
    assert_eq!(response.status().is_success(), true);
}


#[test]
#[ignore]
fn products_with_additive() {
    let off = Off::new("v0").build().unwrap();
    let response = off.products_with_additive("e322-lecithins", None).unwrap();
    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/additive/e322-lecithins.json");
    assert_eq!(response.status().is_success(), true);
}

#[test]
#[ignore]
fn products_in_state() {
    let off = Off::new("v0").build().unwrap();
    let response = off.products_in_state("empty", None).unwrap();

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/state/empty.json");
    assert_eq!(response.status().is_success(), true);
}


#[test]
#[ignore]
fn product_by_barcode() {
    let off = Off::new("v0").build().unwrap();
    let response = off.product_by_barcode("069000019832", None).unwrap();  // Diet Pepsi

    assert_eq!(response.url().as_str(), "https://world.openfoodfacts.org/api/v0/product/069000019832");
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

