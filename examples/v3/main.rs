fn the_letter_a(input: &str) -> Result<(&str, ()), &str> {
    match input.chars().next() {
        Some('a') => Ok((&input['a'.len_utf8()..], ())),
        _ => Err(input),
    }
}

// Instead of doing the work in the function body, we return a closure that does the work, and that matches our type signature for a parser from previously
fn match_literal(expected: &'static str) -> impl Fn(&str) -> Result<(&str, ()), &str> {
    move |input| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

// Thee element name identifier, it's as follows: 
// one alphabetical character, followed by zero or more of either an alphabetical character, a number, or a dash -
fn identifier(input: &str) -> Result<(&str, String), &str> {
    let mut matched = String::new();
    let mut chars = input.chars();

    match chars.next() {
        Some(next) if next.is_alphabetic() =>  matched.push(next),
        _ => return Err(input),
    }

    while let Some(next) = chars.next()  {
        if next.is_alphanumeric() || next ==  '-' {
            matched.push(next)
        }  else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}


#[test]
fn literal_parser() {
    let parse_joe = match_literal("Hello Joe!");
    assert_eq!(Ok(("", ())), parse_joe("Hello Joe!"));
    assert_eq!(
        Ok((" Hello Robert!", ())),
        parse_joe("Hello Joe! Hello Robert!")
    );
    assert_eq!(Err("Hello Mike!"), parse_joe("Hello Mike!"));
}

#[test]
fn identifier_parser() {
    assert_eq!(
        Ok(("", "i-am-an-identifier".to_string())),
        identifier("i-am-an-identifier")
    );
    assert_eq!(
        Ok((" entirely an identifier", "not".to_string())),
        identifier("not entirely an identifier")
    );
    assert_eq!(
        Err("!not at all an identifier"),
        identifier("!not at all an identifier")
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    let output = the_letter_a("abcdef")?;
    dbg!(output);
    Ok(())
}
