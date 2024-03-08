use std::collections::HashMap;


pub fn json_response_single(code:String) -> Result<(), Box<dyn std::error::Error>> {
    let endpoint = format_args!("https://www.flake8api.com/api/rules/{code}/").to_string();
    let resp = reqwest::blocking::get(endpoint)?
        .json::<HashMap<String, String>>()?;
    println!("{:#?}", resp);
    Ok(())
}
