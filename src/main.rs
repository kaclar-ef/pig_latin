use std::io;
fn main() {
    let mut input_text = String::new();
    while !is_ascii_alphabetic_text(&input_text[..]) {
        //英単語を入力させる
        input_text = String::new(); //前回の入力を初期化
        println!("英単語を入力してください");
        io::stdin()
            .read_line(&mut input_text)
            .expect("入力に失敗しました");
        input_text.pop(); //改行コードを取り除く
    }

    if is_vowel(input_text[..].chars().nth(0).unwrap()) {
        input_text.push_str("-hay");
    } else {
        let first_char = input_text.remove(0);
        input_text = format!("{}-{}ay", input_text, first_char);
    }
    println!("{}", input_text);
}

fn is_ascii_alphabetic_text(text: &str) -> bool {
    //入力された英単語のチェック
    //何も入力されていないor半角英字以外が含まれている場合は再度入力させる
    let mut is_alphabetic_text: bool = true;
    if text.len() == 0 {
        is_alphabetic_text = false;
    }
    for c in text.chars() {
        if !c.is_ascii_alphabetic() {
            println!("不正な値です。半角英字で入力してください。");
            is_alphabetic_text = false;
            break;
        }
    }
    is_alphabetic_text
}

fn is_vowel(text: char) -> bool {
    let vowel = "aiueo";
    vowel.contains(text)
}
