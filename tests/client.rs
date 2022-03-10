// Integration tests using API v1.
use openfoodfacts::{Locale, OffBuilder, Output};
use reqwest::StatusCode;

#[test]
fn taxonomy() {
    let off = OffBuilder::new().build_v0().unwrap();
    let response = off.taxonomy("nova_groups").unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://world.openfoodfacts.org/data/taxonomies/nova_groups.json"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn taxonomy_not_found() {
    let off = OffBuilder::new().build_v0().unwrap();
    let response = off.taxonomy("not_found").unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://world.openfoodfacts.org/data/taxonomies/not_found.json"
    );
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[test]
fn facet() {
    let off = OffBuilder::new().build_v0().unwrap();
    let response = off.facet("brands", None).unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://world.openfoodfacts.org/brands.json"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn facet_params() {
    let off = OffBuilder::new().build_v0().unwrap();
    let output = Output {
        locale: Some(Locale::new("fr", None)),
        page: Some(22),
        fields: Some("url"),
        nocache: Some(true),
        ..Output::default()
    };
    let response = off.facet("brands", Some(output)).unwrap();
    assert_eq!(
        response.url().as_str(),
        "https://fr.openfoodfacts.org/brands.json?page=22&fields=url&nocache=true"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn categories() {
    let off = OffBuilder::new().build_v0().unwrap();
    let response = off.categories(None).unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://world.openfoodfacts.org/categories.json"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn categories_params() {
    let off = OffBuilder::new().build_v0().unwrap();
    // Accepts only the locale parameter.
    let output = Output {
        locale: Some(Locale::new("fr", None)),
        page: Some(22),
        ..Output::default()
    };
    let response = off.categories(Some(output)).unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://fr.openfoodfacts.org/categories.json"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn nutrients() {
    let off = OffBuilder::new().build_v0().unwrap();
    let response = off.nutrients(None).unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://world.openfoodfacts.org/cgi/nutrients.pl"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn nutrients_params() {
    let off = OffBuilder::new().build_v0().unwrap();
    // Accepts only the locale parameter.
    let output = Output {
        locale: Some(Locale::new("fr", None)),
        page: Some(22),
        ..Output::default()
    };
    let response = off.nutrients(Some(output)).unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://fr.openfoodfacts.org/cgi/nutrients.pl"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn products_by_facet() {
    let off = OffBuilder::new().build_v0().unwrap();
    let response = off.products_by("additive", "e322-lecithins", None).unwrap();
    assert_eq!(
        response.url().as_str(),
        "https://world.openfoodfacts.org/additive/e322-lecithins.json"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn products_by_facet_params() {
    let off = OffBuilder::new().build_v0().unwrap();
    let output = Output {
        locale: Some(Locale::new("fr", None)),
        page: Some(22),
        page_size: Some(20),
        fields: Some("url"),
        ..Output::default()
    };
    let response = off
        .products_by("additif", "e322-lecithines", Some(output))
        .unwrap();
    assert_eq!(
        response.url().as_str(),
        "https://fr.openfoodfacts.org/additif/e322-lecithines.json?page=22&page_size=20&fields=url"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn products_by_category() {
    let off = OffBuilder::new().build_v0().unwrap();
    let response = off.products_by("category", "cheeses", None).unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://world.openfoodfacts.org/category/cheeses.json"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn products_by_category_params() {
    let off = OffBuilder::new().build_v0().unwrap();
    let output = Output {
        locale: Some(Locale::new("fr", None)),
        page: Some(22),
        page_size: Some(20),
        fields: Some("url"),
        ..Output::default()
    };
    let response = off
        .products_by("categorie", "fromages", Some(output))
        .unwrap();

    assert_eq!(
        response.url().as_str(),
        "https://fr.openfoodfacts.org/categorie/fromages.json?page=22&page_size=20&fields=url"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn product() {
    let off = OffBuilder::new().build_v0().unwrap();
    let response = off.product("069000019832", None).unwrap(); // Diet Pepsi

    assert_eq!(
        response.url().as_str(),
        "https://world.openfoodfacts.org/api/v0/product/069000019832"
    );
    assert_eq!(response.status().is_success(), true);
}

#[test]
fn product_params() {
    let off = OffBuilder::new().build_v0().unwrap();
    // Accepts only the locale and fields parameters.
    let output = Output {
        locale: Some(Locale::new("fr", None)),
        page: Some(22),
        page_size: Some(20),
        fields: Some("url"),
        ..Output::default()
    };
    let response = off.product("069000019832", Some(output)).unwrap(); // 069000019832 = Diet Pepsi

    assert_eq!(
        response.url().as_str(),
        "https://fr.openfoodfacts.org/api/v0/product/069000019832?fields=url"
    );
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
