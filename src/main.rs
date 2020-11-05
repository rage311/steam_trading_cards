use futures::future::join_all;
use std::error::Error;
use structopt::StructOpt;

mod session;
use crate::session::*;
mod util;
use crate::util::list_price;
mod asset;

#[derive(StructOpt, Debug)]
struct CliOpt {
    #[structopt(short = "n", long = "dry-run")]
    dry_run: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut session = Session::new().init().await?;

    let cli_args = CliOpt::from_args();
    dbg!(&cli_args);
    session.config.dry_run = Some(cli_args.dry_run);

    {
        let mut two_factor_code: Option<String> = None;
        loop {
            match session.login(two_factor_code).await? {
                LoginResult::Failure => {
                    eprintln!("Login failure");
                    std::process::exit(1);
                }
                LoginResult::Success => break println!("Login success"),
                LoginResult::TwoFactor => {
                    println!("Two factor code required");
                    two_factor_code = util::two_factor_prompt();
                }
            }
        }
    }

    // get inventory
    let inventory = session.get_inventory().await?;
    println!("{:#?}", inventory);

    let tradables = session.tradable()?;
    println!("{:#?}", tradables);

    // TODO: only do one lowest price lookup per unique market hash name
    let price_futures = tradables
        .iter()
        .map(|asset| asset.lowest_price(&session.client));

    let price_results = join_all(price_futures).await;

    println!("{:#?}", price_results);

    let price_adjust = session.config.price_adjust.unwrap_or(0);

    // cannot be done in parallel -- Steam gives error
    for (asset, price) in tradables.iter().zip(price_results) {
        match price {
            Ok(price) if price > 0 => {
                let list_price = list_price(price, price_adjust);
                println!(
                    "{}\n  lowest: {}\n  list: {}",
                    asset.market_hash_name, price, list_price
                );
                let list_result = session.list_asset(&asset, list_price).await?;
                println!("Result: {}", list_result);
            }
            Ok(_price) => eprintln!("Lowest price was 0 for {}.  Not listing.", asset.market_hash_name),
            Err(e) => eprintln!("lowest price doesn't exist for {}: {}", asset.market_hash_name, e),
        }
    }

    Ok(())
}

// TEST
// let my_asset = Asset {
//     // asset
//     appid: 753,
//     contextid: String::from("6"),
//     assetid: String::from("14860327162"),
//     classid: String::from("634184251"),

//     // description
//     name: String::from("XVIII - The Moon"),
//     market_hash_name: String::from("250900-XVIII - The Moon"),
//     asset_type: String::from("The Binding of Isaac: Rebirth Trading Card"),

//     // separate lookup
//     // cents
//     lowest_price: None,
// };

// println!("{:#?}", lowest_price(&session.client, &my_asset).await?);

// return Ok(());
