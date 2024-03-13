use std::io;
use reqwest;
use serde_json::Value;
static CURRENCY_CODES: [&str; 4]=  ["pln", "usd", "eur", "fjd"];
fn request_manager(source:&str, destiny:&str, quantity:&str) {
    let quantity: f64 = quantity.parse::<f64>().unwrap();
    if CURRENCY_CODES.contains(&source) && CURRENCY_CODES.contains(&destiny) && quantity > 0.0 {
        let url = format!("{}{}","https://open.er-api.com/v6/latest/", &source);
        match reqwest::blocking::get(url) {
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(body) => {
                            let json: Value =  serde_json::from_str(&body).unwrap();
                            let rates: f64 = json.get("rates").unwrap().get(destiny.to_uppercase()).unwrap().as_f64().unwrap();
                            if !rates.is_nan() {
                                println!("Converted value: {} for rates: {:?}", &rates * quantity, rates);
                            } else {
                                println!("There is no currency code as: {}", &destiny);
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
        println!("At least one of the parameters is incorrect. For all currencies type \"list\".\n\
                  Source: {:?}\n\
                  Destiny: {:?}\n\
                  Quantity: {:?}\n",
                  source, destiny, quantity);
    }
}
fn main() {
    loop {
        let mut buffer = String::new();
        let _ = io::stdin().read_line(&mut buffer);
        let input = buffer.trim().to_lowercase();
        let input: Vec<&str> = input.split(" ").collect();

        match input[..] {
            ["list"] => println!("{:?}", CURRENCY_CODES),
            ["exit"] => break,
            [source, destiny, quantity] => request_manager(source, destiny, quantity),
            _ => println!("I don't recognize: \n{:?}", input.join(" "))
        }
    }
}
