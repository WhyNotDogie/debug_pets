pub trait Printer {
    fn print(dialogue: &str);
}

pub struct DefaultPrinter;

impl Printer for DefaultPrinter {
    fn print(dialogue: &str) {
        println!("{}", dialogue)
    }
}