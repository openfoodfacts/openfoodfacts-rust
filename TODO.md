# Componets

## Off builder

Builds an OffClient allowing some configuration options.
One application only needs to build a sigle OffClient.

## OffClient

This is the actual Off API client. Each OffClient owns
its own reqwest::Client object.

In principle, there should be different implementations
for different API versions.

The OffClient provides both API version independent and 
specific methods:

OffClient.taxonomy()
    Returns the given taxonomy JSON file.
    API version: independent

OffClient.facets()
    Returns the given facet.
    API version: independent

OffClient.categories()
    Returns the given category.
    API version: independent

OffClient.nutrients()
    Return the list of nutrients.
    API version: independent

OffClient.products_by()
    Return all products matching the given category or facet.
    API version: independent

OffClient.product()
    Return nutrition facts for a given product (barcode)
    API Version: v0/V2

OffClient::search()
    Search for products
    API version: V0

OffClient::search_by_barcodes
    Search products by barcodes
    API Version: V2

## Output

Defines query parameters that affect the results of several calls:

locale - Overrides the base locale defined in the Off builder
page, page_size - Pagination of results
fields - Return only the indicated fields
nocache - Ignore cached facets

Not all calls support all output parameters. The ones supported are
documented in each client method.

## SearchParams V0

Defines an abstraction on the search query parameters. There is one
implementation (incomplete) for each APU version. Only the search for
V0 is fully implemented.

Each method produces a type of query parameters:

SearchParamsV0.criteria()
    tagtype_N=<criteria>
    tag_contains_N=<op>
    tag_N=<value>
 
    criteria_tags=<value> or criteria_tags_<locale>=<value>

    The implementation takes care of incrementing N on each new criteria parameter. 

SearchParamsV0.ingredient()
    <ingredient>=<value>
 

SearchParamsV0.nutrient()
    nutriment_N=<nutriment>
    nutriment_compare_N=<op>
    nutriment_value_N=<quantity>

    The implementation takes care of incrementing N on each new nutrient parameter. 

SearchParams.sort_by()
    Sets/clears the sort_by query parameter

## SearchParams V2

SearchParamsV2.criteria()
    criteria_tags=<value> or criteria_tags_<locale>=<value>

SearchParamsV2.nutrient()
    <nutrient>_<unit>=<value> if opertor is =, otherwise 

SearchParamsV2.sort_by()
    Sets/clears the sort_by query parameter

Both SearchParamsVx implementations implenment the SearchParams trait. This is the
type expected by OffClient.search() to produce the full list of query parameters
in an API version independent fashion.

The search() method must use a different endpoint per API version.

## Other

* reqwest used in blocking mode
* The library performs no deserialization. This is left to the caller
* Write endpoints not implemented
* Some endpoints need more testing

# Notes

Not sure if the idea of using Off::new(<api version>) is a good idea and instead
of trying to have a single client supporting all API versions we should have
different clients for different versions.

    Off.build_v0() -> OffClientV0
    Off.build_v2() -> OffClientV2

But this may lead to a lot of duplicated code.

# Alternatives

* One OffClient with search_v0() and search_v2()
* Drop V0
* Support all API version using a separate module per version. Have the common code
  implemented as functions in a common module.

# Async

Move the client to async and provide a sync wrapper around it (as the reqwest does with
the blocking module). 


* Beta 1
    - blocking mode
    - taxonomies, etc (v0 and v2 or v2 only ?)
    - read and search requests (v0 and v2 or v2 only ?)

* Beta 2
    - non blocking mode
    - write operations
    - robotoff support ?

* Beta 3
    - Supports the autosuggestions API at
      `https://<locale>.openfoodfacts.org/cgi/suggest.pl?lc=<language>&tagtype=<tag_type>`
    - GraphQL ?
    - WASM client ?
