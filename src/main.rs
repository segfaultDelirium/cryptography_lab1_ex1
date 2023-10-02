fn get_modulo_max() ->  i32{
    'z' as i32 as i32 - 'a' as u8 as i32
}

fn shift_letter(letter: char, n: i32) -> char{
    let letter_as_u8 = (letter as u8) - ('a' as u8);
    let new_letter = letter_as_u8 as i32 + n;
    let modulo_res = new_letter.rem_euclid(get_modulo_max());
    let res = ( modulo_res + ('a' as u8 as i32) ) as u8 as char;
    return res;
}

fn shift_cypher(text: &String, n: i32) -> String{
    let res = text.chars().into_iter().map(|c| shift_letter(c, n))
        .map(|c| format!("{}", c))
        .reduce(|acc, c| format!("{}{}", acc, c)).unwrap();
    return res
}


fn main() {
    // let x = shift_letter('a', 3);
    // let y = shift_letter('d', -3);
    // println!("x = {x}");
    // println!("y = {y}");

    let plaintext = "attackatonce".to_string();
    // let plaintext = "a".to_string();
    println!("Encryption");
    println!("Plaintext: {plaintext}");
    let cyphertext = shift_cypher(&plaintext, 3);
    println!("Cyphertext: {cyphertext}");
    let decrypted_text =  shift_cypher(&cyphertext, -3);
    println!("");
    println!("Decryption");
    println!("Plaintext: {decrypted_text}");
    println!("Cyphertext: {cyphertext}");

}
