# openfoodfacts-rust

<img src="https://avatars.githubusercontent.com/t/4526186?s=280&v=4">

This implements a wrapper around Open Food Facts API in Rust.

[![GitHub Super-Linter](https://github.com/openfoodfacts/openfoodfacts-rust/workflows/Lint%20Code%20Base/badge.svg)](https://github.com/marketplace/actions/super-linter)


## Installation
In the __Cargo.toml__ file, under __[dependencies]__, add the following lines:
```
openfoodfacts = { git = "https://github.com/openfoodfacts/openfoodfacts-rust.git"}
reqwest = {version = "0.11", features = ["blocking", "json"]}
serde_json = "1.0.73"
```

## Examples
_Get information about a product_
```
let client = off::v0().build().unwrap();
let code = "3850102123681";

let response = client.product(code, None).unwrap();
let result_json = json!(response.json::<HashMap::<String, Value>>().unwrap());
```

_Search products based on criteria_

```
use openfoodfacts::{self as off, Locale, Output};
use serde_json::{json, Value};
use std::collections::HashMap;


fn main() {
    // create client and query with criteria
    let client = off::v2().build().unwrap();
    let query = client
        .query()
        .criteria("brands", "Vindija", Some("hr"));

    // run query and convert result as json
    let res = client.search(query, None).unwrap();
    let result_json = json!(res.json::<HashMap::<String, Value>>().unwrap());

    // parse json
    if let Some(products) = result_json["products"].as_array() {
        println!("That many products {}", products.len());
        for product in products {
            println!("Barcode: {}, name: {}, allergens: {}.", product["code"], product["product_name"], product["allergens"]);
        }
    }
}
```

More details about criteria here: https://openfoodfacts.github.io/api-documentation/#5Filtering


## Features
Refer to the following API documentation: https://openfoodfacts.github.io/api-documentation/


List of methods of the client.

|                        | features \ api version | v-0 |                      v-0 example                                                | v-2 | v-2 example |
|------------------------|------------------------|-----|---------------------------------------------------------------------------------|-----|-------------|
| 2-Read                 | product                |  v  | let response = client.product("069000019832", `None`).unwrap();();                |  x  | x           |
| 3-Search / 5-Filtering | search                 |  v  | let response = client.search(**query**, `None`).unwrap();                             |  x  | let response = client.search(**query**, `None`).unwrap(); |
| 7-Metadata             | taxonomy               |  v  | let response = client.taxonomy("nova_groups").unwrap();                         |  x  | x           |
| 7-Metadata             | facet                  |  v  | let response = client.facet("allergens", `None`).unwrap();                        |  x  | x           |
| 7-Metadata             | facet                  |  v  | let response = client.products_by("additive", "e322-lecithins", `None`).unwrap(); |  x  | x           |
| 7-Metadata             | categories             |  v  | let response = client.categories(`None`).unwrap();                                |  x  | x           |
| 7-Metadata             | categories             |  v  | let response = client.products_by("category", "cheeses", `None`).unwrap();        |  x  | x           |
| 7-Metadata             | nutrients              |  v  | let response = client.nutrients(`None`).unwrap();                                 |  x  | x           |

`None` can be replaced by output parameters (language (country code, `cc`; and optional language code, `lc`), page, page_size, fields (fields is used to reduce the response to only the fields you need)):
```
let output = Output::new()
    .locale(Locale::new("fr", None))
    .pagination(22, 20)
    .fields("url");
```

Whereas **query** can be created using criteria, ingredient, nutrient, as follows for v-0 and v-2, respectively:
```v-0
let query = client
    .query()
    .criteria("brands", "contains", "Nestlé")
    .criteria("categories", "does_not_contain", "cheese")
    .ingredient("additives", "without")
    .ingredient("ingredients_that_may_be_from_palm_oil", "indifferent")
    .nutrient("fiber", "lt", 500)
    .nutrient("salt", "gt", 100);
```

```v-0
let query = client
    .query()
    .criteria("brands", "Nestlé", Some("fr"))
    .criteria("categories", "-cheese", None)
    .nutrient_100g("fiber", "<", 500)
    .nutrient_serving("salt", "=", 100);
```


## Third party applications
If you use this SDK, feel free to open a PR to add your application in this list.
