use config;
use reqwest;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

use crate::asset::*;
use crate::util::*;

const STEAM_APP_ID: &'static str = "753";

#[derive(Debug)]
pub enum ListResult {
    Success,
    RequiresConfirmation,
    Failure(String),
    DryRun,
}

impl std::fmt::Display for ListResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ListResult::Success => write!(f, "Success"),
            ListResult::RequiresConfirmation => write!(f, "Requires Confirmation"),
            ListResult::Failure(msg) => write!(f, "Failure: {}", msg),
            ListResult::DryRun => write!(f, "Dry Run"),
        }
    }
}

impl Error for ListResult {}

#[derive(Debug, PartialEq)]
pub enum LoginResult {
    //None,
    Failure,
    Success,
    TwoFactor,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub username: String,
    pub password: String,
    pub steam_id: String,
    pub price_adjust: Option<i64>,
    pub dry_run: Option<bool>,
}

#[derive(Clone, Debug)]
pub struct Session {
    pub client: reqwest::Client,
    pub session_id: Option<String>,
    pub config: Config,
    pub inventory: serde_json::Value,
}

impl Session {
    pub fn new() -> Self {
        let mut config = config::Config::default();
        // config.toml
        config.merge(config::File::with_name("config")).unwrap();

        let config = config.try_into::<Config>().unwrap();
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap();

        let session = Session {
            session_id: None,
            config,
            client,
            inventory: serde_json::Value::Null,
        };

        println!("{:#?}", session);

        session
    }

    // Initializes session
    pub async fn init(mut self) -> Result<Self, Box<dyn Error>> {
        self.session_id = match self.get_session_id().await {
            Ok(option) => match option {
                Some(s_id) => Some(s_id),
                None => {
                    eprintln!("Unable to retrieve sessionid from login page");
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Unable to retrieve sessionid from login page: {}", e);
                std::process::exit(1);
            }
        };
        println!("Session ID: {:#?}", self.session_id.clone().unwrap());

        Ok(self)
    }

    // Lists the asset at the given price on the Steam market
    pub async fn list_asset(
        &self,
        asset: &Asset,
        price_cents: u64,
    ) -> Result<ListResult, Box<dyn Error>> {
        let params = [
            ("sessionid", self.session_id.clone().unwrap()),
            ("appid", asset.appid.to_string()),
            ("contextid", asset.contextid.to_string()),
            ("assetid", asset.assetid.to_string()),
            ("amount", "1".to_string()),
            ("price", price_cents.to_string()),
        ];

        println!(
            "Listing: {} ({}) for ${:.2} (${:.2})",
            asset.asset_type,
            asset.name,
            price_cents as f64 * 1.15 / 100.0,
            price_cents as f64 / 100.0
        );

        if self.config.dry_run.unwrap_or(false) {
            return Ok(ListResult::DryRun);
        }

        let res: serde_json::Value = self
            .client
            .post("https://steamcommunity.com/market/sellitem/")
            .header(
                "Referer",
                format!(
                    "https://steamcommunity.com/id/{}/inventory/",
                    self.config.username
                ),
            )
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", res);

        match res["success"].as_bool() {
            Some(true) => {
                let confirmation = res["requires_confirmation"].as_u64().unwrap() == 1;
                match confirmation {
                    true => Ok(ListResult::RequiresConfirmation),
                    false => Ok(ListResult::Success),
                }
            }
            Some(false) => {
                Err(ListResult::Failure(res["message"].as_str().unwrap().to_string()).into())
            }
            None => Err(ListResult::Failure("No success field in 'sellitem' reply".to_string()).into()),
        }
    }

    // Handles required login steps in order to authenticate with Steam
    pub async fn login(
        &self,
        two_factor_code: Option<String>,
    ) -> Result<LoginResult, Box<dyn Error>> {
        let rsa = self.steam_rsa_params().await?;
        println!("RSA: {:#?}", rsa);

        let b64_pass = rsa_encrypt(&rsa, self.config.password.clone())?;
        println!("b64_pass: {}", b64_pass);

        let login = self
            .steam_login(
                b64_pass.clone(),
                two_factor_code.clone(),
                rsa.timestamp.clone(),
            )
            .await?;

        println!("Login result: {:?}", login);
        Ok(login)
    }

    // Posts login credentials to Steam's servers as final authentication step
    pub async fn steam_login(
        &self,
        password_encrypted: String,
        two_factor_code: Option<String>,
        rsa_timestamp: String,
    ) -> Result<LoginResult, Box<dyn Error>> {
        let two_factor_code = match two_factor_code {
            Some(code) => code,
            _ => String::new(),
        };

        let params = [
            ("donotcache", timestamp_millis().to_string()),
            ("password", password_encrypted),
            ("username", self.config.username.clone()),
            ("twofactorcode", two_factor_code),
            ("rsatimestamp", rsa_timestamp),
            ("captchagid", String::from("-1")),
            ("remember_login", String::from("true")),
            ("captcha_text", String::new()),
            ("emailsteamid", String::new()),
            ("emailauth", String::new()),
            (
                "loginfriendlyname",
                String::from("steam_sell_trading_cards"),
            ),
        ];

        let login: Value = self
            .client
            .post("https://steamcommunity.com/login/dologin/")
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        let success = login["success"].as_bool().unwrap();

        match success {
            true => Ok(LoginResult::Success),
            false => {
                match login["requires_twofactor"].as_bool().unwrap() {
                    true => Ok(LoginResult::TwoFactor),
                    // TODO:
                    //false => return Box::new(Err(LoginResult::Failure)),
                    false => Ok(LoginResult::Failure),
                }
            }
        }
    }

    // Object({
    //     "publickey_exp": String(
    //         "010001",
    //     ),
    //     512 hex chars
    //     "publickey_mod": String(
    //         "ef8d54...",
    //     ),
    //     "success": Bool(
    //         true,
    //     ),
    //     "timestamp": String(
    //         "10842200000",
    //     ),
    //     "token_gid": String(
    //         "1455af270c9a69b3",
    //     ),
    // })
    // Retrieves Steam's RSA parameters which are used in the authentication process
    // for encrypting the password
    pub async fn steam_rsa_params(&self) -> Result<RSAParams, Box<dyn Error>> {
        let timestamp_millis = timestamp_millis().to_string();

        let params = [
            ("username", &self.config.username),
            ("donotcache", &timestamp_millis),
        ];

        let rsa: Value = self
            .client
            .post("https://steamcommunity.com/login/getrsakey/")
            .form(&params)
            .send()
            .await?
            .json()
            .await?;

        println!("{:#?}", rsa);

        Ok(RSAParams {
            modulo: rsa["publickey_mod"].to_string().replace("\"", ""),
            exponent: rsa["publickey_exp"].to_string().replace("\"", ""),
            timestamp: rsa["timestamp"].to_string().replace("\"", ""),
        })
    }

    // Begins a Steam session and retrieves the session ID
    pub async fn get_session_id(&self) -> Result<Option<String>, Box<dyn Error>> {
        let login = self
            .client
            .get("https://steamcommunity.com/login/")
            .send()
            .await?;

        let sess_cookie = login.cookies().find(|c| c.name() == "sessionid");
        match sess_cookie {
            Some(cookie) => Ok(Some(cookie.value().to_string())),
            None => Ok(None),
        }
    }

    // Retrieves user's inventory from Steam
    pub async fn get_inventory(&mut self) -> Result<&serde_json::Value, Box<dyn Error>> {
        self.inventory = self
            .client
            .get(
                format!(
                    "https://steamcommunity.com/inventory/{}/{}/6",
                    self.config.steam_id, STEAM_APP_ID
                )
                .as_str(),
            )
            .query(&[("l", "english"), ("count", "75")])
            .send()
            .await?
            .json()
            .await?;

        Ok(&self.inventory)
    }

    pub fn tradable(&self) -> Result<Vec<Asset>, Box<dyn Error>> {
        // TODO: wat
        match &self.inventory {
            Value::Object(inv) => match (inv.get("assets"), inv.get("descriptions")) {
                (Some(assets), Some(descriptions)) => {
                    println!("inventory ok");
                    let tradables = assets
                        .as_array()
                        .expect("Assets not array")
                        .into_iter()
                        .filter_map(|asset| {
                            println!("{:#?}", asset);
                            let asset = asset
                                .as_object()
                                .expect(format!("Asset not object: {:#?}", asset).as_str());

                            let asset_classid = asset["classid"].as_str().unwrap();

                            let description = descriptions
                                .as_array()
                                .expect("\"Descriptions\" is not an array")
                                .iter()
                                .find(|desc| {
                                    desc["tradable"].as_u64().unwrap() == 1
                                        && desc["classid"].as_str().unwrap() == asset_classid
                                });

                            match description {
                                Some(description) => Some(Asset {
                                    appid: asset["appid"].as_u64().unwrap().into(),
                                    contextid: asset["contextid"].as_str().unwrap().into(),
                                    assetid: asset["assetid"].as_str().unwrap().into(),
                                    classid: asset["classid"].as_str().unwrap().into(),
                                    name: description["name"].as_str().unwrap().into(),
                                    market_hash_name: description["market_hash_name"]
                                        .as_str()
                                        .unwrap()
                                        .into(),
                                    asset_type: description["type"].as_str().unwrap().into(),
                                    lowest_price: None,
                                }),
                                _ => None,
                            }
                        })
                        .collect();
                    return Ok(tradables);
                }
                _ => {
                    println!("inventory not ok");
                    return Ok(vec![]);
                }
            },
            // TODO: error
            _ => return Ok(vec![]),
        }
    }
}
