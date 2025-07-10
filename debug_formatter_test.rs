use cursed::formatter::simple::SimpleCursedFormatter;

fn main() {
    let formatter = SimpleCursedFormatter::default();
    let source = "sus result=x+y*z\nsus compare=a==b&&c!=d";
    
    let formatted = formatter.format(source).unwrap();
    println!("Original:");
    println!("{}", source);
    println!("\nFormatted:");
    println!("{}", formatted);
    
    println!("\nChecking for expected strings:");
    println!("Contains 'sus result = x + y * z': {}", formatted.contains("sus result = x + y * z"));
    println!("Contains 'sus compare = a == b && c != d': {}", formatted.contains("sus compare = a == b && c != d"));
}
