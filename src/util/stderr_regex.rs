use regex::Regex;

pub fn ge_ct_stderr_check(stderr: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let mut error_text: String = String::new();
    let patterns = vec![
        Regex::new(r": (?P<message>no matching key exchange method found)\.")?,
        Regex::new(r"(?P<message>Permission denied)")?,
        Regex::new(r": (?P<message>max-retries exceeded)")?,
        Regex::new(r"(?P<message>Connection timed out)")?,
        // Add more patterns here
    ];
    

    for re in patterns.iter() {
        if let Some(caps) = re.captures(stderr) {
            error_text = caps.name("message").unwrap().as_str().to_string();
            break;
        }
    }

    println!("\nERROR_TEXT FROM RegEx: {:?}", &error_text);

    let connection_error: Option<String> = if error_text.is_empty() {
        None
    } else {
        Some(error_text)
    };

    Ok(connection_error)
}
