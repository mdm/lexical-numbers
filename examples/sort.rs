use lexical_numbers::LexicalNumber;

fn main() {
    let n = 100;

    let mut numbers = (0..n).collect::<Vec<usize>>();
    numbers.sort_by_cached_key(|i| {
        let lexical_number = LexicalNumber::from(*i);
        format!("{}", lexical_number)
    });

    for number in numbers {
        println!("{}", LexicalNumber::from(number));
    }
}
