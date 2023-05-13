use lambda_http::{Response, Body, Error};
use substring::Substring;

pub fn cleanse (text: &str, extra_chars: &str, to_upper: bool) -> String {
    let mut allowed_chars = "ASDFGHJKLQWERTYUIOPZXCVBNM1234567890".to_owned();
    allowed_chars.push_str(extra_chars);
    let mut output = "".to_owned();
    for ch in text.chars() {
        let upper = ch.to_ascii_uppercase();
        if allowed_chars.contains(upper){
            output.push(if to_upper {upper} else {ch});
        }
    }
    output.to_owned()
}

pub fn error_resp(code: u16, message: &str) -> Result<Response<Body>, Error> {
    return Ok(Response::builder()
        .status(code)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?);
}

pub fn success_resp(message: &str) -> Result<Response<Body>, Error> {
    return Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(message.into())
        .map_err(Box::new)?);
}

pub trait Comparing {
    fn exists_in(self, vector: Vec<&str>) -> bool;
}
impl Comparing for &str {
    fn exists_in(self, vector: Vec<&str>) -> bool {
        return vector.contains(&self);
    }
}
impl Comparing for String {
    fn exists_in(self, vector: Vec<&str>) -> bool {
        return vector.contains(&self.as_str());
    }
}


/**
 * Remove any sabotage from the email address.
 */
pub fn clean_email(input: &str) -> String {
    if input.contains("@gmail.com"){
        let at_sign = input.find('@').unwrap();
        let mut output = input.substring(0, at_sign).to_owned();
        output = output.replace(".", "");
        if output.contains('+') {
            output = output.substring(0, output.find('+').unwrap()).to_owned();
        }
        output.push_str(input.substring(at_sign, input.len()));
        return output;
    }
    return input.to_owned();
}

