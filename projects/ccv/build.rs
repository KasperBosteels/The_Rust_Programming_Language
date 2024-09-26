fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("C:/Users/kaspe/OneDrive/Afbeeldingen/icons/ccv serice code.ico");
    res.compile().unwrap();
}
