use std::io;
use reqwest;
use serde_json::Value;
fn main() {
    let currency_codes: [&str; 4]=  ["pln", "usd", "eur", "fjd"];

    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer);
        let mut input = buffer.trim().to_lowercase();
        let mut input: Vec<&str> = input.split(" ").collect();

        match input[..] {
            ["list"] => {println!("{:?}", currency_codes);}
            ["exit"] => {break;}
            [source, destiny, quantity] => {
                if currency_codes.contains(&source) && currency_codes.contains(&destiny) && &quantity.parse::<f64>().unwrap() > &0.0 {
                    let url = format!("{}{}","https://open.er-api.com/v6/latest/", &source);
                    match reqwest::blocking::get(url) {
                        Ok(response) => {
                            if response.status().is_success() {
                                match response.text() {
                                    Ok(body) => {
                                        let mut json: Value =  serde_json::from_str(&body).unwrap();
                                        let mut rates: Option<f64> = json.get("rates").unwrap().get(destiny.to_uppercase()).unwrap().as_f64();
                                        if !rates.is_none() {
                                            println!("Converted value: {} for rates: {:?}", &rates.unwrap() * &quantity.parse::<f64>().unwrap(), rates);
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
                    println!("Something wrong");
                    println!("Source: {:?}", source);
                    println!("Destiny: {:?}", destiny);
                    println!("Quantity: {:?}", quantity);
                }
            }
            _ => {println!("I don't recognize: \n{:?}", input.join(" "));}
        }
    }
}
