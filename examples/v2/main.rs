fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input),
    }
}

// Instead of doing the work in the function body, we return a closure that does the work, and that matches our type signature for a parser from previously
fn match_literal_v1(expected: &'static str)
    -> impl Fn(&str) -> Result<(&str, ()), &str>
{
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => {
            Ok((&input[expected.len()..], ()))
        }
        _ => Err(input),
    }
}

fn match_literal_v2(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.contains(expected) {
        true => Ok((&input[expected.len()..], ())),
        false => Err(input),
    }
}

#[test]
fn literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(
        Ok(("", ())),
        parse_joe("Hello Joe!")
    );
    assert_eq!(
        Ok((" Hello Robert!", ())),
        parse_joe("Hello Joe! Hello Robert!")
    );
    assert_eq!(
        Err("Hello Mike!"),
        parse_joe("Hello Mike!")
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let output = the_letter_a("abcdef")?;
    dbg!(output);
    Ok(())
}
