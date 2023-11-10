#[allow(unused)]
fn break_continue() {
    let v = vec![10, 20, 30];
    let mut iter = v.into_iter();
    // println!("v is {v}"); error v move to iter
    // v.iter() borrow
    'outer: while let Some(x) = iter.next() {
        println!("x: {x}");
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}

#[allow(unused)]
fn loop_keyword() {
    let mut x = 10;
    loop {
        x = if x % 2 == 0 { x / 2 } else { 3 * x + 1 };
        if x == 1 {
            break;
        }
    }
    println!("x: {x}");
}

enum WebEvent {
    PageLoad,                 // Variant without payload
    KeyPress(char),           // Tuple struct variant
    Click { x: i64, y: i64 }, // Full struct variant
}

#[rustfmt::skip]
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad       => println!("page loaded"),
        WebEvent::KeyPress(c)    => println!("pressed '{c}'"),
        WebEvent::Click { x, y } => println!("clicked at x={x}, y={y}"),
    }
}

#[allow(unused)]
fn enum_print() {
    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('x');
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(press);
    inspect(click);
}

fn second_word_to_upper(s: &str) -> Option<String> {
    let mut it = s.split(' ');
    let (Some(_), Some(item)) = (it.next(), it.next()) else {
        return None;
    };
    Some(item.to_uppercase())
}

#[allow(unused)]
fn if_control() {
    println!("{:?}", second_word_to_upper("foo bar"));
}

#[allow(unused)]
fn match_control(val: &str) {
    match Some(val) {
        Some("cat") => println!("Will do cat things"),
        Some("ls") => println!("Will ls some files"),
        Some("mv") => println!("Let's move some files"),
        Some("rm") => println!("Uh, dangerous!"),
        None => println!("Hmm, no program name?"),
        _ => println!("Unknown program name!"),
    }

    let pair = (2, -2);
    println!("Tell me about {pair:?}");
    match pair {
        (x, y) if x == y => println!("These are twins"),
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_break_continue() {
        break_continue()
    }

    #[test]
    fn test_loop_keyword() {
        loop_keyword()
    }

    #[test]
    fn test_enum_print() {
        enum_print()
    }

    #[test]
    fn test_if_control() {
        if_control()
    }

    #[test]
    fn test_match_control() {
        match_control("cat")
    }
}
