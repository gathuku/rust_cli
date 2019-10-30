extern crate reqwest;

use std::collections::HashMap;

pub fn test () -> Result<(), Box<dyn std::error::Error>> {
    let resp: HashMap<String, String> = reqwest::get("https://httpbin.org/ip")?
        .json()?;
    println!("{:#?}", resp);
    Ok(())
}
