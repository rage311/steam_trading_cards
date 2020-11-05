
struct Inventory {
    assets: Vec<Asset>,
    descriptions: Vec<Description>,
}

// "amount": String(
//     "1",
// ),
// "appid": Number(
//     753,
// ),
// "assetid": String(
//     "14684030727",
// ),
// "classid": String(
//     "3563481296",
// ),
// "contextid": String(
//     "6",
// ),
// "instanceid": String(
//     "0",
// ),
struct Asset {
    amount: String,
    appid: u32,
    assetid: String,
    classid: String,
    contextid: String,
    instanceid: String,
}

struct Description {
    appid: u32,
    background_color: String,
    classid: String,
    commodity: i32,
    currency: i32,
    descriptions: Vec<serde_json::Value>,
}
            // "appid": Number(
            //     753,
            // ),
            // "background_color": String(
            //     "",
            // ),
            // "classid": String(
            //     "3563481296",
            // ),
            // "commodity": Number(
            //     1,
            // ),
            // "currency": Number(
            //     0,
            // ),
            // "descriptions": Array([
            //     Object({
            //         "value": String(
            //             "",
            //         ),
            //     }),
            // ]),
            // "icon_url": String(
            //     "IzMF03bk9WpSBq-S-ekoE33L-iLqGFHVaU25ZzQNQcXdBnY7ltYLvVIHHqLGePAYFJ9x6TqJAILfgoVWhysa22dML3_r0HVhaeJyVOIe",
            // ),
            // "icon_url_large": String(
            //     "IzMF03bk9WpSBq-S-ekoE33L-iLqGFHVaU25ZzQNQcXdBnY7ltYLvVIHHqLGePAYFJ9x6TqJAILfgoVWhysa22dML3_03GZqYN8chaYU",
            // ),
            // "instanceid": String(
            //     "0",
            // ),
            // "market_fee_app": Number(
            //     1005300,
            // ),
            // "market_hash_name": String(
            //     "1005300-The Jackbox Party Pack 6 Booster Pack",
            // ),
            // "market_marketable_restriction": Number(
            //     7,
            // ),
            // "market_name": String(
            //     "The Jackbox Party Pack 6 Booster Pack",
            // ),
            // "market_tradable_restriction": Number(
            //     7,
            // ),
            // "marketable": Number(
            //     1,
            // ),
            // "name": String(
            //     "The Jackbox Party Pack 6 Booster Pack",
            // ),
            // "owner_actions": Array([
            //     Object({
            //         "link": String(
            //             "javascript:OpenBooster( 1005300, \'%assetid%\' )",
            //         ),
            //         "name": String(
            //             "Unpack...",
            //         ),
            //     }),
            // ]),
            // "tags": Array([
            //     Object({
            //         "category": String(
            //             "droprate",
            //         ),
            //         "internal_name": String(
            //             "droprate_0",
            //         ),
            //         "localized_category_name": String(
            //             "Rarity",
            //         ),
            //         "localized_tag_name": String(
            //             "Common",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "Game",
            //         ),
            //         "internal_name": String(
            //             "app_1005300",
            //         ),
            //         "localized_category_name": String(
            //             "Game",
            //         ),
            //         "localized_tag_name": String(
            //             "The Jackbox Party Pack 6",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "item_class",
            //         ),
            //         "internal_name": String(
            //             "item_class_5",
            //         ),
            //         "localized_category_name": String(
            //             "Item Type",
            //         ),
            //         "localized_tag_name": String(
            //             "Booster Pack",
            //         ),
            //     }),
            // ]),
            // "tradable": Number(
            //     1,
            // ),
            // "type": String(
            //     "Booster Pack",
            // ),

// }

// Object({
    // "assets": Array([
        // Object({
            // "amount": String(
            //     "1",
            // ),
            // "appid": Number(
            //     753,
            // ),
            // "assetid": String(
            //     "14684030727",
            // ),
            // "classid": String(
            //     "3563481296",
            // ),
            // "contextid": String(
            //     "6",
            // ),
            // "instanceid": String(
            //     "0",
            // ),
        // }),
        // Object({
            // "amount": String(
            //     "1",
            // ),
            // "appid": Number(
            //     753,
            // ),
            // "assetid": String(
            //     "14860327162",
            // ),
            // "classid": String(
            //     "634184251",
            // ),
            // "contextid": String(
            //     "6",
            // ),
            // "instanceid": String(
            //     "0",
            // ),
        // }),
        // Object({
            // "amount": String(
            //     "1",
            // ),
            // "appid": Number(
            //     753,
            // ),
            // "assetid": String(
            //     "14262245904",
            // ),
            // "classid": String(
            //     "3873503140",
            // ),
            // "contextid": String(
            //     "6",
            // ),
            // "instanceid": String(
            //     "3873503133",
            // ),
        // }),
        // Object({
            // "amount": String(
            //     "1",
            // ),
            // "appid": Number(
            //     753,
            // ),
            // "assetid": String(
            //     "14169164425",
            // ),
            // "classid": String(
            //     "3873503084",
            // ),
            // "contextid": String(
            //     "6",
            // ),
            // "instanceid": String(
            //     "3873503133",
            // ),
        // }),
    // ]),
    // "descriptions": Array([
        // Object({
            // "appid": Number(
            //     753,
            // ),
            // "background_color": String(
            //     "",
            // ),
            // "classid": String(
            //     "3563481296",
            // ),
            // "commodity": Number(
            //     1,
            // ),
            // "currency": Number(
            //     0,
            // ),
            // "descriptions": Array([
            //     Object({
            //         "value": String(
            //             "",
            //         ),
            //     }),
            // ]),
            // "icon_url": String(
            //     "IzMF03bk9WpSBq-S-ekoE33L-iLqGFHVaU25ZzQNQcXdBnY7ltYLvVIHHqLGePAYFJ9x6TqJAILfgoVWhysa22dML3_r0HVhaeJyVOIe",
            // ),
            // "icon_url_large": String(
            //     "IzMF03bk9WpSBq-S-ekoE33L-iLqGFHVaU25ZzQNQcXdBnY7ltYLvVIHHqLGePAYFJ9x6TqJAILfgoVWhysa22dML3_03GZqYN8chaYU",
            // ),
            // "instanceid": String(
            //     "0",
            // ),
            // "market_fee_app": Number(
            //     1005300,
            // ),
            // "market_hash_name": String(
            //     "1005300-The Jackbox Party Pack 6 Booster Pack",
            // ),
            // "market_marketable_restriction": Number(
            //     7,
            // ),
            // "market_name": String(
            //     "The Jackbox Party Pack 6 Booster Pack",
            // ),
            // "market_tradable_restriction": Number(
            //     7,
            // ),
            // "marketable": Number(
            //     1,
            // ),
            // "name": String(
            //     "The Jackbox Party Pack 6 Booster Pack",
            // ),
            // "owner_actions": Array([
            //     Object({
            //         "link": String(
            //             "javascript:OpenBooster( 1005300, \'%assetid%\' )",
            //         ),
            //         "name": String(
            //             "Unpack...",
            //         ),
            //     }),
            // ]),
            // "tags": Array([
            //     Object({
            //         "category": String(
            //             "droprate",
            //         ),
            //         "internal_name": String(
            //             "droprate_0",
            //         ),
            //         "localized_category_name": String(
            //             "Rarity",
            //         ),
            //         "localized_tag_name": String(
            //             "Common",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "Game",
            //         ),
            //         "internal_name": String(
            //             "app_1005300",
            //         ),
            //         "localized_category_name": String(
            //             "Game",
            //         ),
            //         "localized_tag_name": String(
            //             "The Jackbox Party Pack 6",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "item_class",
            //         ),
            //         "internal_name": String(
            //             "item_class_5",
            //         ),
            //         "localized_category_name": String(
            //             "Item Type",
            //         ),
            //         "localized_tag_name": String(
            //             "Booster Pack",
            //         ),
            //     }),
            // ]),
            // "tradable": Number(
            //     1,
            // ),
            // "type": String(
            //     "Booster Pack",
            // ),
        // }),
        // Object({
            // "appid": Number(
            //     753,
            // ),
            // "background_color": String(
            //     "",
            // ),
            // "classid": String(
            //     "634184251",
            // ),
            // "commodity": Number(
            //     1,
            // ),
            // "currency": Number(
            //     0,
            // ),
            // "descriptions": Array([
            //     Object({
            //         "value": String(
            //             "",
            //         ),
            //     }),
            // ]),
            // "icon_url": String(
            //     "IzMF03bk9WpSBq-S-ekoE33L-iLqGFHVaU25ZzQNQcXdA3g5gMEPvUZZEfSMJ6dESN8p_2SVTY7V2NsKxHoMmChCIzb02ClVU8xFEYCLizHzpKmgE3v5Lj7JLibcQQs-G-ALZDzR-DP07OWTSj7MR-t5Ql8ELPQHpjBAb5-JNhFp1YRYqDP2h0p6WBQnYMFDYjCyx3UUNOBxmiQWJJIAmizydZXehA5hb05pUrC1X-3LO4ekkC4hWRgxGqUSYY_BpmWyr4G3Z_a7dsEw3A",
            // ),
            // "icon_url_large": String(
            //     "IzMF03bk9WpSBq-S-ekoE33L-iLqGFHVaU25ZzQNQcXdA3g5gMEPvUZZEfSMJ6dESN8p_2SVTY7V2NsKxHoMmChCIzb02ClVU8xFEYCLizHzpKmgE3v5Lj7JLibcQQs-G-ALZDzR-DP07OWTSj7MR-t5Ql8ELPQHpjBAb5-JNhFp1YRYqDP2h0p6WBQnYMFDYjCyx3UUNOBxmiQWJJIAmizydZXehA5hb05pUrC1X-3LO4ekkC4hWRgxGqUSYY_BpmWyr4G3Z_a7dsEw3A",
            // ),
            // "instanceid": String(
            //     "0",
            // ),
            // "market_fee_app": Number(
            //     250900,
            // ),
            // "market_hash_name": String(
            //     "250900-XVIII - The Moon",
            // ),
            // "market_marketable_restriction": Number(
            //     7,
            // ),
            // "market_name": String(
            //     "XVIII - The Moon",
            // ),
            // "market_tradable_restriction": Number(
            //     7,
            // ),
            // "marketable": Number(
            //     1,
            // ),
            // "name": String(
            //     "XVIII - The Moon",
            // ),
            // "owner_actions": Array([
            //     Object({
            //         "link": String(
            //             "https://steamcommunity.com/my/gamecards/250900/",
            //         ),
            //         "name": String(
            //             "View badge progress",
            //         ),
            //     }),
            //     Object({
            //         "link": String(
            //             "javascript:GetGooValue( \'%contextid%\', \'%assetid%\', 250900, 11, 0 )",
            //         ),
            //         "name": String(
            //             "Turn into Gems...",
            //         ),
            //     }),
            // ]),
            // "tags": Array([
            //     Object({
            //         "category": String(
            //             "droprate",
            //         ),
            //         "internal_name": String(
            //             "droprate_0",
            //         ),
            //         "localized_category_name": String(
            //             "Rarity",
            //         ),
            //         "localized_tag_name": String(
            //             "Common",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "Game",
            //         ),
            //         "internal_name": String(
            //             "app_250900",
            //         ),
            //         "localized_category_name": String(
            //             "Game",
            //         ),
            //         "localized_tag_name": String(
            //             "The Binding of Isaac: Rebirth",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "cardborder",
            //         ),
            //         "internal_name": String(
            //             "cardborder_0",
            //         ),
            //         "localized_category_name": String(
            //             "Card Border",
            //         ),
            //         "localized_tag_name": String(
            //             "Normal",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "item_class",
            //         ),
            //         "internal_name": String(
            //             "item_class_2",
            //         ),
            //         "localized_category_name": String(
            //             "Item Type",
            //         ),
            //         "localized_tag_name": String(
            //             "Trading Card",
            //         ),
            //     }),
            // ]),
            // "tradable": Number(
            //     1,
            // ),
            // "type": String(
            //     "The Binding of Isaac: Rebirth Trading Card",
            // ),
        // }),
        // Object({
            // "appid": Number(
            //     753,
            // ),
            // "background_color": String(
            //     "",
            // ),
            // "classid": String(
            //     "3873503140",
            // ),
            // "commodity": Number(
            //     1,
            // ),
            // "currency": Number(
            //     0,
            // ),
            // "descriptions": Array([
            //     Object({
            //         "value": String(
            //             "",
            //         ),
            //     }),
            // ]),
            // "icon_url": String(
            //     "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxH5rd9eDAjcFyv45SRYAFMIcKL_PArgVSL403ulRUWEndVKv8hZmACggkalIA4-mkf1Jmi6DOKDkRv4q1zYOIlaCkNePTzmkDvpRzjuiQpN702wX6ux07dqoQL3s",
            // ),
            // "icon_url_large": String(
            //     "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxH5rd9eDAjcFyv45SRYAFMIcKL_PArgVSL403ulRUWEndVKv8hZmACggkagIB7-ihe1BkhvWeIWtAtN-1wNeJx6_yYejQzz5VucR3276Qo9j33wD6ux07oYu5Nzc",
            // ),
            // "instanceid": String(
            //     "3873503133",
            // ),
            // "market_fee_app": Number(
            //     1263950,
            // ),
            // "market_hash_name": String(
            //     "1263950-Steam Hearts",
            // ),
            // "market_marketable_restriction": Number(
            //     7,
            // ),
            // "market_name": String(
            //     "Steam Hearts",
            // ),
            // "market_tradable_restriction": Number(
            //     7,
            // ),
            // "marketable": Number(
            //     0,
            // ),
            // "name": String(
            //     "Steam Hearts",
            // ),
            // "tags": Array([
            //     Object({
            //         "category": String(
            //             "droprate",
            //         ),
            //         "internal_name": String(
            //             "droprate_0",
            //         ),
            //         "localized_category_name": String(
            //             "Rarity",
            //         ),
            //         "localized_tag_name": String(
            //             "Common",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "Game",
            //         ),
            //         "internal_name": String(
            //             "app_1263950",
            //         ),
            //         "localized_category_name": String(
            //             "Game",
            //         ),
            //         "localized_tag_name": String(
            //             "The Debut Collection",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "item_class",
            //         ),
            //         "internal_name": String(
            //             "item_class_11",
            //         ),
            //         "localized_category_name": String(
            //             "Item Type",
            //         ),
            //         "localized_tag_name": String(
            //             "Sticker",
            //         ),
            //     }),
            // ]),
            // "tradable": Number(
            //     0,
            // ),
            // "type": String(
            //     "The Debut Collection Sticker",
            // ),
        // }),
        // Object({
            // "appid": Number(
            //     753,
            // ),
            // "background_color": String(
            //     "",
            // ),
            // "classid": String(
            //     "3873503084",
            // ),
            // "commodity": Number(
            //     1,
            // ),
            // "currency": Number(
            //     0,
            // ),
            // "descriptions": Array([
            //     Object({
            //         "value": String(
            //             "",
            //         ),
            //     }),
            // ]),
            // "icon_url": String(
            //     "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxH5rd9eDAjcFyv45SRYAFMIcKL_PArgVSL403ulRUWEndVKv8hZmACggkagZV4b71LVA1hqefdG0T792wxNOOwa6jZu-Cxm5QvZV1i77Hp4333wz6ux07Ykgtd98",
            // ),
            // "icon_url_large": String(
            //     "-9a81dlWLwJ2UUGcVs_nsVtzdOEdtWwKGZZLQHTxH5rd9eDAjcFyv45SRYAFMIcKL_PArgVSL403ulRUWEndVKv8hZmACggkalUAs-LzelNjhqSfdTwW6IS3xIaNxqOta76CzzxV7MEhiLmYoYmt2A36ux070_u1dBU",
            // ),
            // "instanceid": String(
            //     "3873503133",
            // ),
            // "market_fee_app": Number(
            //     1263950,
            // ),
            // "market_hash_name": String(
            //     "1263950-Excited",
            // ),
            // "market_marketable_restriction": Number(
            //     7,
            // ),
            // "market_name": String(
            //     "Excited",
            // ),
            // "market_tradable_restriction": Number(
            //     7,
            // ),
            // "marketable": Number(
            //     0,
            // ),
            // "name": String(
            //     "Excited",
            // ),
            // "tags": Array([
            //     Object({
            //         "category": String(
            //             "droprate",
            //         ),
            //         "internal_name": String(
            //             "droprate_0",
            //         ),
            //         "localized_category_name": String(
            //             "Rarity",
            //         ),
            //         "localized_tag_name": String(
            //             "Common",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "Game",
            //         ),
            //         "internal_name": String(
            //             "app_1263950",
            //         ),
            //         "localized_category_name": String(
            //             "Game",
            //         ),
            //         "localized_tag_name": String(
            //             "The Debut Collection",
            //         ),
            //     }),
            //     Object({
            //         "category": String(
            //             "item_class",
            //         ),
            //         "internal_name": String(
            //             "item_class_11",
            //         ),
            //         "localized_category_name": String(
            //             "Item Type",
            //         ),
            //         "localized_tag_name": String(
            //             "Sticker",
            //         ),
            //     }),
            // ]),
            // "tradable": Number(
            //     0,
            // ),
            // "type": String(
            //     "The Debut Collection Sticker",
            // ),
        // }),
    // ]),
    // "rwgrsn": Number(
        // -2,
    // ),
    // "success": Number(
        // 1,
    // ),
    // "total_inventory_count": Number(
        // 4,
    // ),
// })
