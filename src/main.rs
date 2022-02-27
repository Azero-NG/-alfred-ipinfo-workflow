extern crate clap;
mod ipinfo;
use clap::{Arg, Command};

#[tokio::main]
async fn main() {
    let get_local = Command::new("get_local").about("get local machine's ip info");
    let get_ipinfo = Command::new("get_ipinfo")
        .about("get local machine's ip info")
        .arg(
            Arg::new("ip")
                .short('p')
                .long("ip")
                .required(true)
                .value_name("ip"),
        );

    let matches = Command::new("ipinfo")
        .version("0.1")
        .author("azero")
        .about("get ipinfo")
        .subcommand(get_local)
        .subcommand(get_ipinfo)
        .arg_required_else_help(true)
        .get_matches();

    if matches.subcommand_matches("get_local").is_some() {
        match ipinfo::get_local().await {
            Ok(resp) => {
                println!("{}", resp);
            }
            Err(e) => println!("Error {:?}", e),
        }
    }

    if let Some(matches) = matches.subcommand_matches("get_ipinfo") {
        if let Some(ip) = matches.value_of("ip") {
            match ipinfo::get_ipinfo(ip).await {
                Ok(resp) => {
                    println!("{}", resp);
                }
                Err(e) => println!("Error {:?}", e),
            }
        }
    }
}
