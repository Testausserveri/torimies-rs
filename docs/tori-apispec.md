# Tori.fi api specification

This document describes our current understanding of the Tori.fi API.
Please note that all of the information here is gathered through reverse-engineering the API
and none of it is confirmed in any way.

Torimies relies on the Tori.fi API rather than scraping the website. This allows Torimies to
be faster and more accurate than any of its scraper alternatives.

Using the API also comes with some problems.
Tori.fi has not released any public documentation for the API which means that all features
used by Torimies were discovered through different methods of reverse-engineering the
Tori.fi API.

## API Overview

The Tori-API `BASE\_URL` used by Torimies is `https://api.tori.fi/api/v1.2`

The Tori.fi API consists of multiple routes and endpoints of which Torimies
only utilizes the ads endpoint found at `$BASE_URL/public/ads` which allows retrieving
a filtered list of Tori.fi-ads using query-parameters for the filtering.

In a nutshell the Tori.fi API is an excellent example on **how to not design an API** and
it is highly recommended to not proceed further in this document if one has the intention
to preserve their sanity.

## `/public/ads`

The endpoint for fetching a filtered list of ads.

The filters can be applied with query-parameters.

The output has the following type:
```yaml
{
    config_etag: string,
    counter_map: {
        all: int
    },
    list_ads: [
        {
            ad: {
                account: {
                    code: string,
                    label: string,
                },
                account_ads: {
                    code: string,
                    label: string,
                },
                ad_id: string,
                body: string,
                category: {
                    code: string,
                    label: string,
                    name: string,
                    path_en: string,
                    parent: string,
                    aurora_vertical: string,
                },
                company_ad: bool,
                full_details: bool,
                images: [
                    {
                        base_url: string,
                        media_id: string,
                        path: string,
                        width: int,
                        height: int,

                    },
                    ...
                ],
                list_id: string,
                list_id_code: string,
                list_price: {
                    currency: string,
                    price_value: int,
                    label: string,
                },
                locations: [
                    {
                        code: string,
                        key: string,
                        label: string
                        locations: [
                            ...
                        ]
                    },
                    ...
                ],
                mc_settings: {
                    use_form: boolean,
                },
                phone_hidden: boolean,
                prices: [
                    {
                        currency: string,
                        price_value: int,
                        label: string,
                        old_price: {
                            price_value: int,
                            label: string,
                        }
                    },
                    ...
                ],
                polepos_ad: int,
                status: string,
                store_details: {
                    id: string,
                    name: string,
                    plan: string,
                    slogan: string,
                    address: string,
                    city: string,
                    zipcode: string,
                    category: string,
                    link: string,
                },
                subject: string,
                thumbnail: {
                    base_url: string,
                    media_id: string,
                    path: string,
                    width: int,
                    height: int,
                },
                type: {
                    code: string,
                    label: string,
                },
                user: {
                    account: {
                        name: string,
                        created: string,
                    },
                    uuid: string,
                },
                share_link: string,
                link: {
                    label: string,
                    url: string,
                },
                highlight_price: boolean,
                external_integration: {
                    url: string,
                    label: string,
                    type: string,
                },
                pivo: {
                    enable: bool
                },
                list_time: {
                    label: string,
                    value: int,
                }
            },
            labelmap: {
                category: string,
                type: string,
            },
            spt_metadata: {
                category: string,
                contentid: string,
                details: {
                    currency: string,
                    locality: string,
                    postalCode: string,
                    price: string,
                    region: string,
                },
            }
        },
        ...
    ],
    next_page: string,
    proximity_slices: [],
    sorting: string,
    spt_metadata: [
        contentid: string,
        filter: {
            currency: string,
            numResults: int,
        }
    ]
}
```

### Fields and descriptions (as far as we know them)

Below is a table containing some of the fields in the above example response paired with brief descriptions.
Some fields we deem irrelevant are intentionally left out.

#### DISCLAIMER
All the descriptions provided below are educated guesses and not facts. One is recommended
to do their own research before blindly utilizing any information provided here.

| Field | Type | Description |
|-------|------|-------------|
| counter\_map | Object | Contains a single field `all` which contains the total count of ads in the response |
| counter\_map.all | int | The total count of ads in the response |
| list\_ads | Array | A list of Objects each of which the fields `ad`, `labelmap` and `spt_metadata`, The Object in the `ad` is described below in great detail |
| sorting | string | The sorting method or whatever value is specified with `?sort=value`, defaults to `date`. The only other value that does something seems to be `price` |
| spt\_metadata | Object | Some metadata...? |
| spt\_metadata.filter | Object | Contains two fields `currency` and `numResults` |
| spt\_metadata.filter.currency | string | Value is `EUR` we are not aware of any way to change this |
| spt\_metadata.filter.numResults | int | Same value as `counter_map.all`, **NOTE: for some reason the name is camelCase :D** |

### Ad-Object

For Torimies' purposes the most important part of the response is the object describing an individual ad.
Therefore it is also the most in-detail part of this document.

Below is a brief summary of each field and their description (if known).

#### NOTE
Some of the fields are not always present. This makes for a great deal of fun and deserializer-errors :]

##### TODO:
Quite a lot of `ad_details` are missing.. `clothing_*` and `car_*`, `cx_*`, `regdate` to give a few examples.
These will be more important once we start implementing support for Tori Autot.


| Field | Type | Description |
|-------|------|-------------|
| account | Object | The publisher of the ad |
| account.code | string | The account id (a number) of the publisher account **as a string** |
| account.label | string | Exactly the same as `account.code` |
| account\_ads | Object | Total ad count of the publisher account |
| ad\_id | string | Not an id, actually a path of format `/private/accounts/{account_id}/ads/{ad_id}` where the *filename* is the actual ad id which is an integer |
| body | string | The ad body, the description written by the publisher |
| category | Object | Describes the category the ad belongs in |
| category.code | string | The category id (a number) **as a string** |
| category.label | string | The category label, for example `Leikkurit ja koneet` |
| category.name | string | An empty string |
| category.path\_en | string | An empty string |
| category.parent | string | An empty string |
| category.aurora\_vertical | string | No idea what this describes, values include `mobility` and `recommerce` |
| company\_ad | bool | Whether the publisher is a company or not |
| ad\_details | Object | Contains some ad-specific details. These often depend on the type of item being sold and therefore are not always present |
| ad\_details.delivery\_options | Object | The available delivery options |
| ad\_details.delivery\_options.multiple | Array | List of available delivery options, `multiple` seems to be always present on `delivery_options` regardless of the actual amount of delivery options|
| ad\_details.delivery\_options.multiple[index] | Object | A delivery option |
| ad\_details.delivery\_options.multiple[index].code | String | The code for a delivery option, for example `delivery_send` |
| ad\_details.delivery\_options.multiple[index].label | String | The label for a delivery option, for example `Lähetys` |
| ad\_details.general\_condition | Object | The general condition of the item |
| ad\_details.general\_condition.single | Object | The general condition of the item |
| ad\_details.general\_condition.single.code | String | The code for the condition, for example `excellent` |
| ad\_details.general\_condition.single.label | String | The label for the condition, for example `Erinomainen` |
| images | Array | List of images associated with the ad |
| images[index] | Object | An individual image object |
| images[index].base\_url | string | The base url for images, the value seems to always be `https://img.tori.fi/image` |
| images[index].media\_id | string | Not an id, actually the base path for the ad images of format `/public/media/ad/{ad_id}` |
| images[index].path | string | A path of format `10/{image_id}` where the `image_id` is a number and actually the only thing we care about in this Object. One can fetch the image from the following url `"https://images.tori.fi/api/v1/imagestori/images/{image_id}?rule=medium_660"` |
| images[index].width | int | The width of the image |
| images[index].height | int | The height of the image |
| list\_id | string | A path of the format `/public/ads/{ad_id}` |
| list\_id\_code | string | The actual `ad_id` (a number) **as a string** |
| list\_price | Object | Contains information about the current price for the ad |
| list\_price.currency | string | The currency for the price, seems to always be `EUR` |
| list\_price.price\_value | int | The value for the price |
| list\_price.label | string | The value paired with a currency symbol, for example `995 €` |
| locations | Array | A list of locations that always contains one item, a location-Object which is a recursive data-type |
| locations[0].code | string | The code for the location (a number) **as a string** |
| locations[0].key | string | The key for the location for example `region`, `area` or `zipcode` |
| locations[0].label | string | The label for the location for example `Kymenlaakso`, `Kouvola` or `Korjala-Kaunisnurmi` |
| locations[0].locations | Array | The sub-locations list of the same type as the `locations`-array (might not be present) |
| mc\_settings | Object | No idea |
| mc\_settings.use\_form | bool | No idea |
| phone\_hidden | bool | Whether or not to hide the phone number??! |
| prices | Array | A List of price-Objects, always of length 1 |
| prices[0] | Object | An object depicting a price (current or historical) for the ad, a recursive data-type |
| prices[0].currency | string | The currency for the price, present on the current price |
| prices[0].price\_value | int | The value for the price |
| prices[0].label | string | The value paired with a currency symbol |
| prices[0].old\_price | Object | Of same type as `prices[0]`, describes an older price. The field is not present if there is no older price |
| status | string | The ad status, seems to always be `active` |
| subject | string | The subject for the ad |
| thumbnail | Object | The ad thumbnail, same type as `images[index]` |
| type | Object | The ad type |
| type.code | string | The code for the ad type, for example `s` |
| type.label | string | The label for the ad type, for example `Myydään` |
| user | Object | The publisher user |
| user.account | Object | The publisher user-account (not the same as `account`) |
| user.account.name | string | The username for the publisher account |
| user.account.created | string | A string depicting the account age, for example `maaliskuusta 2023` |
| user.uuid | string | The uuid for the publisher |
| share\_link | string | The share link for the ad |
| pivo | Object | No idea |
| pivo.enabled | bool | No idea |
| list\_time | Object | The time the listing was published **NOTE: please see further information below** |
| list\_time.label | string | The label for the time, for example `tänään 17:50` |
| list\_time.value | int | A unix EPOCH timestamp for the release time (either UTC or Finnish local time) |

### The thing about timestamps

Yeah.. so there's a thing...

One could think that the timestamp in `list_time` is assigned to a new ad on creation and
doesn't change after that. That would seem more than reasonable. But as it turn out
that is in fact not the case.

There seems to be a time period of approximately 10 minutes after a new ad is published to Tori.fi
during which the `list_time` of that specific ad is updated to the time when it is requested.
One can even experience this phenomena live by opening up a new Tori.fi ad, waiting for a minute
and then refreshing the page just to see the creation time update before their own eyes.

Torimies has to get around this somehow and that is why we implement the `ItemHistory`.
But yeah... this is the stuff that makes one go insane.

## Query parameters

This is a brief overview of the API query options. This only covers the options relevant for Torimies but there
are many more of them.

[This endpoint](https://api.tori.fi/api/v1.7/public/filters) seems to list out some of the options.

### Account

Torimies uses the ads endpoint with the `account={account_id}` argument to fetch information about a seller account.
This is a bit sub-optimal due to the fact that no account information is available for an account with no active ads.

### Limit

The number of ads returned can be limited with the argument `lim={number}`

### Location
Available values listed [here](https://api.tori.fi/api/v1.2/public/regions).

The location is specified with three parameters. Only the first one of which is supported by the Tori.fi search.
Those parameters are `region`, `area` and `zipcode`.

#### Examples:
| Location | Arguments |
|----------|-----------|
| Ahvenanmaa | `region=15` |
| Ahvenanmaa, Brändö | `region=15&area=256` |

Multiple regions and can be selected by just chaining multiple `region` arguments.

### Category
Available values listed [here](https://api.tori.fi/api/v1.2/public/categories/filter).

#### Examples
| Category | Arguments |
|----------|-----------|
| Elektroniikka | `category=5000` |
| Elektroniikka, Puhelimet | `category=5012` |

### Ad type

Specified with the `ad_type` parameter. Valid values include
| Value | Meaning |
|-------|---------|
| s | Myydään |
| k | Ostetaan |
| g | Annetaan |
| u | Vuokrataan |
| h | Halutaan vuokrata |

## Tori.fi query parameters

The most difficult task for Torimies is the conversion between Tori.fi-search urls
and tori api query urls. This is by no means an easy task since the argument conversion rules range from slightly inconvenient
to completely nonsensical.

Here is the set of conversion rules used by Torimies

| Search url argument | API-query argument | Description |
|---------------------|--------------------|---------|
| `q` | `q` | The search keywords (uses the ISO-8859-2 encoding) |
| `cg` | `category` | The category, the value of `0` is ignored |
| `c` | `category` | Also the category, used over the `cg` value if present (this contains sub-category information) |
| `ps` and `pe` | `suborder` | Suborder specifies a price range. See the conversion table below |
| `ca` | `region` | The region |
| `m` | `area` | The area |
| `w` | `region` | The region, the conversion is done by subtracting 100 from the w-value, if the w value is below 100 it should be ignored otherwise the value should be used over any `ca` argument |
| `f` | `company_ad` | Whether the ad is from a company. The conversion is the following: `a=>ignore`,`p=>0`,`c=>1` |
| `st` | `ad_type` | The ad type described above |
| Any other argument | ignored/unsupported | Anything else is ignored so that the query isn't messed up |

### Price range conversion table

The price ranges in the tori.fi search url are converted to the suborder argument
according to the following table

| `ps`/`pe` value | `suborder` value |
|-----------------|------------------|
| 0 | 0 |
| 1 | 25 |
| 2 | 50 |
| 3 | 75 |
| 4 | 100 |
| 5 | 250 |
| 6 | 500 |
| 7 | 1000 |
| 8 | 2000 |
