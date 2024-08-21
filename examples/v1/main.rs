fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let output = the_letter_a("abcdef")?;
    dbg!(output);
    Ok(())
}
