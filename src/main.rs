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

static URL_BASE: &str = "https://open.er-api.com/v6/latest/";

fn make_request(currency: &str) -> reqwest::Result<reqwest::blocking::Response> {
    let url = URL_BASE.to_owned() + currency;
    return reqwest::blocking::get(url);
}

fn currency_details(currency: &str) -> String {
    let result: String;
    if CURRENCY_CODES.contains(&currency) {
        match make_request(currency) {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => {
                            let json: Value =  serde_json::from_str(&body).unwrap();
                            let rates: &Value = json.get("rates").unwrap();
                            let rates = serde_json::to_string(rates).unwrap().replace("\"", "\'");
                            result = format!("Rates for {}:\n{:?}", currency, rates);
                        }
                        Err(err) => result = format!("Error: {:?}", err)
                    }
                } else {
                    result = format!("Error: {:?}", response.status());
                }
            }
            Err(err) => result = format!("Error: {:?}", err),
        }
    }else {
        result = format!("Currency code \"{currency}\" not in the list.")
    }
    return result
}

fn represent_error_template(source: &str, destiny: &str, quantity: &str) -> String{
    return format!("At least one of the parameters is incorrect. For all currencies type \"list\".\n\
                  Source: {:?}\n\
                  Destiny: {:?}\n\
                  Quantity: {:?}\n",
                  source, destiny, quantity);
}

fn exchange_manager(source: &str, destiny: &str, quantity: &str) ->String {
    let _quantity: f64;
    let result: String;

    match quantity.parse::<f64>() {
        Ok(q) => { _quantity = q; }
        Err(_parse_float_error) => {return  represent_error_template(source, destiny, quantity);}
    }

    if CURRENCY_CODES.contains(&source) && CURRENCY_CODES.contains(&destiny) && _quantity > 0.0 {
        match make_request(source) {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => {
                            let json: Value =  serde_json::from_str(&body).unwrap();
                            let rates: f64 = json.get("rates").unwrap().get(destiny.to_uppercase()).unwrap().as_f64().unwrap();
                            if !rates.is_nan() {
                                result = format!("Converted value: {} for rates: {}", rates * _quantity, rates);
                            } else {
                                result = format!("There is no currency code as: {}", destiny);
                            }
                        }
                        Err(err) => result = format!("Error: {:?}", err)
                    }
                } else {
                    result = format!("Error: {:?}", response);
                }
            }
            Err(err) => result = format!("Error: {:?}", err.to_string()),
        }
    } else {
        result = represent_error_template(source, destiny, quantity);
    }
    return result
}

fn help_manager() -> String {
    return "Welcome in Help Page.\n\
            Full list of commends:\n\
            Name\tDescription\n\
            list\tList all offered currency's.\n\
            list X\tAll rates of the specific currency. X=currency\n\
            exit\tExit the program.\n\
            X Y Z\tYou can write two currency codes and program will exchange the amount in quantity from first one to send.\n\
            \tX=First currency, Y=currency Z=Amount.".parse().unwrap();
}
fn main() {
    println!("Welcome to CLI-app - real-time currency exchanger.\n\
              If you don't know what to do type \"help\".");
    let mut result : String = String::new();
    let mut buffer = String::with_capacity(20);
    loop {
        result.clear();
        buffer.clear();
        let _ = io::stdin().read_line(&mut buffer);
        let input = buffer.trim().to_lowercase();
        let input: Vec<&str> = input.split(" ").collect();

        match input[..] {
            ["list"] => result = format!("{:?}", CURRENCY_CODES),
            ["list", currency] => result = currency_details(currency),
            ["help"] => result = help_manager().parse().unwrap(),
            ["exit"] => break,
            [source, destiny, quantity] => result = exchange_manager(source, destiny, quantity),
            _ => result = format!("I don't recognize: \n{:?}", input.join(" "))
        }
        println!("{result}");
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::string::String;

    #[test]
    fn test_help_manager(){
        let origin: String = "Welcome in Help Page.\n\
            Full list of commends:\n\
            Name\tDescription\n\
            list\tList all offered currency's.\n\
            list X\tAll rates of the specific currency. X=currency\n\
            exit\tExit the program.\n\
            X Y Z\tYou can write two currency codes and program will exchange the amount in quantity from first one to send.\n\
            \tX=First currency, Y=currency Z=Amount.".parse().unwrap();

        let target: String = help_manager();

        assert_eq!(origin, target);
    }

    #[test]
    fn test_currency_details() {
        use std::string::String;
        let proper_currency = "pln";
        let bad_currency = "xyz";

        let url = URL_BASE.to_owned() + proper_currency;
        let mut origin = String::new();
        let mut target = String::new();

        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => {
                            let json: Value =  serde_json::from_str(&body).unwrap();
                            let rates: &Value = json.get("rates").unwrap();
                            let rates = serde_json::to_string(rates).unwrap().replace("\"", "\'");
                            origin = format!("Rates for {}:\n{:?}", proper_currency, rates);
                        }
                        Err(err) => origin = format!("Error: {:?}", err)
                    }
                } else {
                    origin = format!("Error: {:?}", response.status());
                }
            }
            Err(err) => origin = format!("Error: {:?}", err),
        }

        target = currency_details(proper_currency);
        assert_eq!(origin, target);

        origin = format!("Currency code \"{bad_currency}\" not in the list.");
        target = currency_details(bad_currency);
        assert_eq!(origin, target);
    }

    #[test]
    fn test_exchange_manager() {
        let bad_source = "xyz";
        let bad_destiny = "xyz";
        let bad_quantity = "xyz";
        let proper_source = "pln";
        let proper_destiny = "eur";
        let proper_quantity = "2.0";

        let mut origin:String = String::new();
        let mut target:String = String::new();

        origin = represent_error_template(bad_source, proper_destiny, proper_quantity);
        target = exchange_manager(bad_source, proper_destiny, proper_quantity);
        assert_eq!(origin, target);

        origin = represent_error_template(proper_source, bad_destiny, &proper_quantity);
        target = exchange_manager(proper_source, bad_destiny, &proper_quantity);
        assert_eq!(origin, target);

        origin = represent_error_template(proper_source, proper_destiny, &bad_quantity);
        target = exchange_manager(proper_source, proper_destiny, &bad_quantity);
        assert_eq!(origin, target);

        origin = represent_error_template(proper_source, proper_destiny, &bad_quantity);
        target = exchange_manager(proper_source, proper_destiny, &bad_quantity);
        assert_eq!(origin, target);


        let url = URL_BASE.to_owned() + proper_source;
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => {
                            let json: Value =  serde_json::from_str(&body).unwrap();
                            let rates: f64 = json.get("rates").unwrap().get(proper_destiny.to_uppercase()).unwrap().as_f64().unwrap();
                            if !rates.is_nan() {
                                origin = format!("Converted value: {} for rates: {}", rates * proper_quantity.parse::<f64>().unwrap() , rates);
                            } else {
                                origin = format!("There is no currency code as: {}", proper_destiny);
                            }
                        }
                        Err(err) => origin = format!("Error: {:?}", err)
                    }
                } else {
                    origin = format!("Error: {:?}", response);
                }
            }
            Err(err) => origin = format!("Error: {:?}", err.to_string()),
        }
        target = exchange_manager(proper_source, proper_destiny, &proper_quantity);
        assert_eq!(origin, target)
    }
}