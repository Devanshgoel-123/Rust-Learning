fn main() {
    let sentence: String = String::from("my name is Devnash");
    let first_word: String = get_first_word(sentence);
    println!("The first word is: {}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut answer: String = String::from("");
    for char in sentence.chars() {
        if char == ' ' {
            break;
        }
        answer.push(char);
    }
    answer
}
