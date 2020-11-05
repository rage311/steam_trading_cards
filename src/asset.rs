use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Hash, Clone)]
// lowest price is separate
pub struct Asset {
    // asset
    pub appid: u64,
    pub contextid: String,
    pub assetid: String,
    pub classid: String,

    // description
    pub name: String,
    pub market_hash_name: String,
    pub asset_type: String,

    // separate lookup
    // cents
    pub lowest_price: Option<u64>,
}

impl Asset {
    pub async fn lowest_price(&self, client: &reqwest::Client) -> Result<u64, Box<dyn Error>> {
        let params = [
            ("country", "US"),
            ("currency", "1"),
            ("appid", &format!("{}", self.appid)[..]),
            ("market_hash_name", self.market_hash_name.as_str()),
        ];

        println!("{:#?}", params);

        let res: serde_json::Value = client
            .get("https://steamcommunity.com/market/priceoverview/")
            .query(&params)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", res);

        let price_info = res.as_object().unwrap();
        let price = price_info["lowest_price"]
            .as_str()
            .unwrap()
            .replace("$", "")
            .parse::<f64>()
            .unwrap();

        //self.lowest_price = Some((price * 100f64) as u64);
        Ok((price * 100.0f64) as u64)
    }
}

pub fn uniq_assets(assets: &Vec<Asset>) -> Vec<&String> {
    let mut uniq = HashSet::new();
    assets.iter().for_each(|e| {
        let _is_new = uniq.insert(&e.market_hash_name);
    });
    uniq.into_iter().collect()
}
