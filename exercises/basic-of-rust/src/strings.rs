// Exercise 1
#[allow(dead_code)]
fn exercise1(color: &str) -> String {
    return color.to_string();
}

// Exercise 2
// Fix all errors without adding newline
fn exercise2() -> String {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str("world!");
    return s;
}
// Exercise 3
// Fix errors without removing any line
fn exercise3() -> String {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3: String = s1 + &s2;
    print!("{}", s3);
    return s3;
}

// Exercise 4
// Reverse a string

fn reverse_string(input: &str) -> String {
    return input.to_string();
}

// Exercise 5
// Check if a string is a palindrome
fn is_palindrome(word: &str) -> bool {
    word.char_indices()
        .filter(|&(_, c)| c.is_alphabetic())
        // zip with the second half...
        .zip(
            word.char_indices()
                // which needs to be reversed...
                .rev()
                // and filter out bad cars
                .filter(|&(_, c)| c.is_alphabetic()),
        )
        // accept all input until the indexes have crossed
        .take_while(|&((first_count, _), (last_count, _))| first_count < last_count)
        // check that all the chars from the begining and end match
        .all(|((_, first_char), (_, last_char))| {
            first_char.to_ascii_lowercase() == last_char.to_ascii_lowercase()
        })
}

// Exercise 6
// Count the occurrences of a character in a string
fn count_char_occurrences(string: &str, ch: char) -> usize {
    let mut count: usize = 0;
    let to_string = string.to_string();
    let to_chars = to_string.chars();
    for c in to_chars {
        if c == ch {
            count = count + 1 as usize;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test for exercise 1
    #[test]
    fn exercise1_work() {
        assert_eq!("white".to_string(), exercise1("white"));
    }

    // Test for exercise 2
    #[test]
    fn exercise2_work() {
        assert_eq!("hello, world!".to_string(), exercise2());
    }

    // Test for exercise 3
    #[test]
    fn exercise3_work() {
        assert_eq!("hello, world!".to_string(), exercise3());
    }

    // Test for exercise 4
    #[test]
    fn test_reverse_string() {
        assert_eq!(reverse_string("hello"), "olleh");
        assert_eq!(reverse_string("rust"), "tsur");
        assert_eq!(reverse_string("world"), "dlrow");
        assert_eq!(reverse_string(""), "");
    }

    // Test for exercise 5
    #[test]
    fn test_palindrome() {
        assert_eq!(is_palindrome("level"), true);
        assert_eq!(is_palindrome("deed"), true);
        assert_eq!(is_palindrome("Rotor"), true);
    }
    // Test for exercise 5
    #[test]
    fn test_non_palindrome() {
        assert_eq!(is_palindrome("hello"), false);
        assert_eq!(is_palindrome("world"), false);
    }

    // Test for exercise 6

    #[test]
    fn test_count_char_occurrences() {
        assert_eq!(count_char_occurrences("Hello", 'l'), 2);
        assert_eq!(count_char_occurrences("Rust is fun", 'u'), 1);
        assert_eq!(count_char_occurrences("Mississippi", 's'), 4);
    }
}
