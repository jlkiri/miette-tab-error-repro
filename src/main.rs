use miette::{Diagnostic, SourceSpan, Result};
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
enum Error {
    #[error("Whoops. Not enough coffee.")]
    #[diagnostic(code(::coffee))]
    NotEnoughCoffee(
        #[source_code] String,
        #[label = "Here."] (usize, usize),
    )
}

fn main() -> Result<()> {
    let source = "Cups of coffee:\t\t\t\t\t\t\t\t\t\t3 or less.";
    // Uncomment next line to panic.
    // let source = "Cups of coffee:\t\t\t\t\t\t\t\t\t\t\t\t\t3 or less."; // Panics
    let tab_count = source.matches('\t').count();
    let tabs = tab_count * 2;
    let offset = source.find("3").unwrap_or(0);
    dbg!(offset + tabs - tab_count);
    Err(Error::NotEnoughCoffee(source.to_string(), (offset + tabs - tab_count, 0)).into())
}
