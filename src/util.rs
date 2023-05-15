fn get_header_value<K: reqwest::header::AsHeaderName>(
    key: K,
    response: &reqwest::blocking::Response,
) -> Option<&str> {
    let value = if let Some(x) = response.headers().get(key) {
        x
    } else {
        return None;
    };

    match value.to_str() {
        Ok(x) => Some(x),
        Err(_) => None,
    }
}

fn handle_response<T>(response: &reqwest::blocking::Response) -> Result<T, ()> {
    let content_type = match get_header_value(reqwest::header::CONTENT_TYPE, &response) {
        Some(x) => x,
        None => return Err(()),
    };

    println!("{}", content_type);

    match content_type {
        "application/json" => todo!(),
        "text/html; charset=UTF-8" => todo!(),
        "text/html" => todo!(),
        _ => Err(()),
    }
}
