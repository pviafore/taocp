use crate::chartable; 
pub fn convert_to_punch_cards(vec: Vec<u8>, start_location: i16) -> String {
    vec.chunks(6)
        .map(|x| (x[0], (x[1] as i32) * 64_i32.pow(4) + (x[2] as i32) * 64_i32.pow(3) + (x[3] as i32) * 64_i32.pow(2) + (x[4] as i32) * 64 + (x[5] as i32)))
        .map(|x| format!("{}{:0>10}", x.0 as char, x.1.to_string()))
        .collect::<Vec<String>>()
        .chunks(7)
        .map(|x| format!("+PRGRM{}{:04}{}", x.len(), start_location, x.join("")))
        .collect::<Vec<String>>()
        .join("\n") + "\n"
}


pub fn translate_data_card(text: &str) -> Vec<u8> {
    let data: Vec<u8> = chartable::translate(text);
    let mut output_data: Vec<u8> = vec![];
    for i in 0..(data.len()/11) {
        for j in i*11+1..(i+1)*11 {
            output_data.push(data[j])
        }
        if data[i*11] == 45 { // - after a translate
            output_data[i*10 + 9] -= 20;
        }
    }
    output_data
}
