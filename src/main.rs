
fn main() {
    let url = "https://www.worldometers.info/coronavirus/";
    let response = reqwest::blocking::get(url).expect("Could not load URL");

    
    let raw_html_string = response.text().unwrap();
}
