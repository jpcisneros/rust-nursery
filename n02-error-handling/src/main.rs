use error_chain::error_chain;
use std::net::IpAddr;
use std::str;

error_chain! {
    foreign_links {
        Utf8(std::str::Utf8Error);
        AddrParse(std::net::AddrParseError);
    }
} 


fn main() -> Result<()> {
    let bytes = b"2001:db9::1";

    let s = str::from_utf8(bytes)?;

    let addr: IpAddr = s.parse()?;

    println!("{:?}", addr);
    Ok(())
}
