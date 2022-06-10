use regex::Regex;
use std::io::stdin;

fn main() {
    let input = input("Enter something to search", None);
    let given_sentence: &str = "This is foo and FoO foo as well as Foo.";
    let res = number_of_occurrences(given_sentence, &input);

    println!("count {}", res);
}

fn number_of_occurrences(given_sentence: &str, regex: &str) -> u32 {
    let _given_sentence = given_sentence.to_lowercase();
    let _str_regex = regex.to_lowercase();
    let _regex: Regex = Regex::new(&_str_regex).unwrap();
    _regex.find_iter(&_given_sentence).count() as u32
}

fn input(message: &str, instruction: Option<&str>) -> String {
    let mut line: String = String::new();
    println!("{}:", message);

    match instruction {
        Some(ins) => println!("{}", ins),
        None => (),
    }

    stdin().read_line(&mut line).unwrap();
    line.pop();

    line
}

#[test]
fn given_sentence_include_input() {
    let given_sentence: &str = "This is foo and FoO foo as well as Foo.";
    let input: &str = "FoO";

    assert_eq!(number_of_occurrences(given_sentence, input), 4);
}
