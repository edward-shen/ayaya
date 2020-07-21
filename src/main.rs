use std::io::{stdout, Result, Write};

fn main() -> Result<()> {
    stdout().write_all(include_bytes!("ayaya.utf.ans"))
}
