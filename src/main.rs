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

    #[structopt(short = "c", long = "config")]
    config_file_basename: Option<String>,

    #[structopt(short = "d", long = "debug")]
    debug: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli_args = CliOpt::from_args();

    let config_file_basename = cli_args
        .config_file_basename
        .unwrap_or(String::from("config"));

    let mut session = Session::new(config_file_basename.as_str()).init().await?;
    session.config.dry_run = Some(cli_args.dry_run);
    session.config.debug = Some(cli_args.debug);

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

    let _inventory = session.get_inventory().await?;
    let tradables = session.tradable()?;

    if tradables.len() == 0 {
        println!("No tradable items in inventory");
        return Ok(());
    }

    // TODO: only do one lowest price lookup per unique market hash name
    let price_futures = tradables
        .iter()
        .map(|asset| asset.lowest_price(&session.client));

    let price_results = join_all(price_futures).await;
    let price_adjust = session.config.price_adjust.unwrap_or(0);

    // cannot be done in parallel -- Steam gives error
    for (asset, price) in tradables.iter().zip(price_results) {
        match price {
            Ok(price) if price > 0 => {
                let list_price = list_price(price, price_adjust);
                let list_result = session.list_asset(&asset, list_price).await?;
                println!("Result: {}", list_result);
            }
            Ok(_price) => eprintln!(
                "Lowest price was 0 for {}.  Not listing.",
                asset.market_hash_name
            ),
            Err(e) => eprintln!(
                "lowest price doesn't exist for {}: {}",
                asset.market_hash_name, e
            ),
        }
    }

    Ok(())
}
