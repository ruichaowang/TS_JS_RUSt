extern crate test_type_script;
use std::io::Write;
use test_type_script::ts_to_js;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Current directory: {}", std::env::current_dir()?.display());
    let ts_code = std::fs::read_to_string("./examples/hello2.ts")?;
    let js_converted = ts_to_js("activity_hello.ts", &ts_code)?;
    let output_path = "./examples/converted_hello2.js";
    let mut file = std::fs::File::create(output_path)?;
    file.write_all(js_converted.as_bytes())?;
    println!(
        "JavaScript converted content has been saved to {}",
        output_path
    );

    Ok(())
}