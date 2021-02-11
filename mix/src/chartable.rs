
use std::collections::HashMap;

fn get_chartable() -> [(char, u8); 56] {
    let char_table = [
        (' ', 0u8),
        ('A', 1u8),
        ('B', 2u8),
        ('C', 3u8),
        ('D', 4u8),
        ('E', 5u8),
        ('F', 6u8),
        ('G', 7u8),
        ('H', 8u8),
        ('I', 9u8),
        ('∆', 10u8),
        ('J', 11u8),
        ('K', 12u8),
        ('L', 13u8),
        ('M', 14u8),
        ('N', 15u8),
        ('O', 16u8),
        ('P', 17u8),
        ('Q', 18u8),
        ('R', 19u8),
        ('∑', 20u8),
        ('Π', 21u8),
        ('S', 22u8),
        ('T', 23u8),
        ('U', 24u8),
        ('V', 25u8),
        ('W', 26u8),
        ('X', 27u8),
        ('Y', 28u8),
        ('Z', 29u8),
        ('0', 30u8),
        ('1', 31u8),
        ('2', 32u8),
        ('3', 33u8),
        ('4', 34u8),
        ('5', 35u8),
        ('6', 36u8),
        ('7', 37u8),
        ('8', 38u8),
        ('9', 39u8),
        ('.', 40u8),
        (',', 41u8),
        ('(', 42u8),
        (')', 43u8),
        ('+', 44u8),
        ('-', 45u8),
        ('*', 46u8),
        ('/', 47u8),
        ('=', 48u8),
        ('$', 49u8),
        ('<', 50u8),
        ('>', 51u8),
        ('@', 52u8),
        (';', 53u8),
        (':', 54u8),
        ('\'', 55u8),
    ];
    char_table
}

pub fn translate(text: &str) -> Vec<u8> {
    let char_table: HashMap<char, u8> = get_chartable().iter().cloned().collect();
    text.chars().map(|x| char_table[&x] as u8).collect()
}

pub fn to_char(values: Vec<u8>) -> String {
    let char_table: HashMap<u8, char> = get_chartable()
                                        .to_vec()
                                        .iter()
                                        .map(|x| (x.1, x.0))
                                        .collect();
    values.iter().map(|x| String::from(char_table[&x])).collect::<Vec<String>>().join("")
}
