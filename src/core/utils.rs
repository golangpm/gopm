use neofiglet::FIGfont;
pub fn logo() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("GOPM");
    assert!(figure.is_some());
    let out = format!("{}\nInvalid command. Use 'gopm --help' for usage information.", figure.unwrap());
    eprintln!("{out}");
}