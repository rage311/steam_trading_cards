You know those useless trading cards that sit in your Steam inventory that lack enough value to even be worth taking the time to list manually on the Steam marketplace?  This app is here to alleviate that pain and still earn the market value for each of the cards.

Warning: there are probably bugs.  Use at your own risk.  Run it with the `--dry-run` flag first to be sure of what you're doing.  Steam Guard 2-factor authentication is required.  This has only been tested in the US market in USD currency.  Please open an issue to report any bugs.

First, edit the config.toml file with your actual user name, password, and Steam ID.

By default, it will list all of the "Trading Cards" in your Steam inventory for their current going rate (i.e. the "Starting at: $X.XX" price).  A price adjustment can be configured in the `price_adjust` variable in the `config.toml` file.  A setting of `0` indicates that the card will be listed at the current market price, `-1` will list at the current market price minus 1 cent, `+1` raises the list price by 1 cent, etc.

Then run in a terminal/command prompt:

From source: `cargo run -- --dry-run`

From the Windows executable: `steam_trading_cards.exe --dry-run`

(then remove the `--dry-run` flag when you're confident things look correct)

```
USAGE:
    steam_trading_cards [FLAGS] [OPTIONS]

FLAGS:
    -d, --debug
    -n, --dry-run
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <config-file-basename>
```
