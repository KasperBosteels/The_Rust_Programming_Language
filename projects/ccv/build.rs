fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("C:/Users/kaspe/Downloads/10803116_digital_key_security_bitcoin_trader_icon.ico");
    res.compile().unwrap();
}
