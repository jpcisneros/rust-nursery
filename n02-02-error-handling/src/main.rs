use error_chain::error_chain;
use url::{Url, Position};

error_chain!{
    foreign_links {
        UrlParse(url::ParseError);
    }
}

fn main() -> Result<()> {
    let parsed = Url::parse("https://httpbin.org/cookies/set?k3=v4")?;
    let cleaned: &str = &parsed[..Position::AfterPath];

    println!("cleaned: {:?}", cleaned);
    Ok(())
}

