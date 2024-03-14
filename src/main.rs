use std::io;
use reqwest;
use serde_json::Value;

static CURRENCY_CODES: [&str; 161]=  [
    "aed", "afn", "all", "amd", "ang", "aoa", "ars", "aud", "awg", "azn",
    "bam", "bbd", "bdt", "bgn", "bhd", "bif", "bmd", "bnd", "bob", "brl",
    "bsd", "btn", "bwp", "byn", "bzd", "cad", "cdf", "chf", "clp", "cny",
    "cop", "crc", "cup", "cve", "czk", "djf", "dkk", "dop", "dzd", "egp",
    "ern", "etb", "eur", "fjd", "fkp", "fok", "gbp", "gel", "ggp", "ghs",
    "gip", "gmd", "gnf", "gtq", "gyd", "hkd", "hnl", "hrk", "htg", "huf",
    "idr", "ils", "imp", "inr", "iqd", "irr", "isk", "jep", "jmd", "jod",
    "jpy", "kes", "kgs", "khr", "kid", "kmf", "krw", "kwd", "kyd", "kzt",
    "lak", "lbp", "lkr", "lrd", "lsl", "lyd", "mad", "mdl", "mga", "mkd",
    "mmk", "mnt", "mop", "mru", "mur", "mvr", "mwk", "mxn", "myr", "mzn",
    "nad", "ngn", "nio", "nok", "npr", "nzd", "omr", "pab", "pen", "pgk",
    "php", "pkr", "pln", "pyg", "qar", "ron", "rsd", "rub", "rwf", "sar",
    "sbd", "scr", "sdg", "sek", "sgd", "shp", "sle", "sos", "srd", "ssp",
    "stn", "syp", "szl", "thb", "tjs", "tmt", "tnd", "top", "try", "ttd",
    "tvd", "twd", "tzs", "uah", "ugx", "usd", "uyu", "uzs", "ves", "vnd",
    "vuv", "wst", "xaf", "xcd", "xdr", "xof", "xpf", "yer", "zar", "zmw",
    "zwl"];


fn make_request(currency: &str) -> reqwest::Result<reqwest::blocking::Response> {
    let url = "https://open.er-api.com/v6/latest/".to_owned() + currency;
    return reqwest::blocking::get(url);
}

fn currency_details(currency: &str) {
    if CURRENCY_CODES.contains(&currency) {
        match make_request(currency) {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => {
                            let json: Value =  serde_json::from_str(&body).unwrap();
                            let rates: &Value = json.get("rates").unwrap();
                            let rates = serde_json::to_string(rates).unwrap().replace("\"", "\'");
                            println!("Rates for {}:", currency);
                            println!("{:?}", rates)
                        }
                        Err(err) => eprintln!("Error: {:?}", err)
                    }
                } else {
                    eprintln!("Error: {:?}", response.status());
                }
            }
            Err(err) => eprintln!("Error: {:?}", err),
        }
    }
}

fn exchange_manager(source: &str, destiny: &str, quantity: &str) {
    let quantity: f64 = quantity.parse::<f64>().unwrap();
    if CURRENCY_CODES.contains(&source) && CURRENCY_CODES.contains(&destiny) && quantity > 0.0 {
        match make_request(source) {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => {
                            let json: Value =  serde_json::from_str(&body).unwrap();
                            let rates: f64 = json.get("rates").unwrap().get(destiny.to_uppercase()).unwrap().as_f64().unwrap();
                            if !rates.is_nan() {
                                println!("Converted value: {} for rates: {}", rates * quantity, rates);
                            } else {
                                eprintln!("There is no currency code as: {}", destiny);
                            }
                        }
                        Err(err) => eprintln!("Error: {:?}", err)
                    }
                } else {
                    eprintln!("Error: {:?}", response.status());
                }
            }
            Err(err) => eprintln!("Error: {:?}", err),
        }
    } else {
        eprintln!("At least one of the parameters is incorrect. For all currencies type \"list\".\n\
                  Source: {:?}\n\
                  Destiny: {:?}\n\
                  Quantity: {:?}\n",
                  source, destiny, quantity);
    }
}

fn help_manager() {
    println!("Welcome in Help Page.\n\
              Full list of commends:\n\
              Name\tDescription\n\
              list\tList all offered currency's.\n\
              list X\tAll rates of the specific currency. X=currency\n\
              exit\tExit the program.\n\
              X Y Z\tYou can write two currency codes and program will exchange the amount in quantity from first one to send.\n\
              \tX=First currency, Y=currency Z=Amount.")
}
fn main() {
    println!("Welcome to CLI-app - real-time currency exchanger.\n\
              If you don't know what to do type \"help\".");

    loop {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        let input = buffer.trim().to_lowercase();
        let input: Vec<&str> = input.split(" ").collect();

        match input[..] {
            ["list"] => println!("{:?}", CURRENCY_CODES),
            ["list", currency] => currency_details(currency),
            ["help"] => help_manager(),
            ["exit"] => break,
            [source, destiny, quantity] => exchange_manager(source, destiny, quantity),
            _ => eprintln!("I don't recognize: \n{:?}", input.join(" "))
        }
    }
}
