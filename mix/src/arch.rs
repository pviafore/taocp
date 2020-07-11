// Define a byte to hold up to 64 bits 
const BYTE_MASK: u8 = 0x3F;
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Byte {
    value: u8,
}
impl Byte {

    pub fn read(&self) -> u8{
        return self.value & BYTE_MASK 
    }

    pub fn new(value: u8) -> Byte {
        Byte{value: value & BYTE_MASK}
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Word{
    pub is_positive: bool,
    pub bytes: [Byte; 5]
}

impl Word {
    pub fn new() -> Word {
        Word::from_value(0)
    }

    pub fn from_values(is_positive: bool, val1: u8, val2: u8, val3: u8, val4: u8, val5: u8) -> Word {
        Word{
            is_positive: is_positive,
            bytes: [Byte::new(val1),Byte::new(val2),Byte::new(val3),Byte::new(val4),Byte::new(val5)]
        }
    }

    pub fn from_value(val: i32) -> Word {
        Word::from_values(val.is_positive(),
                          ((val.abs() >> 24) % 64) as u8,
                          ((val.abs()>> 18) % 64) as u8,
                          ((val.abs() >> 12) % 64) as u8,
                          ((val.abs() >> 6) % 64) as u8,
                          (val.abs() % 64) as u8  ) 
    }

    pub fn from_half_word(val: HalfWord) -> Word {
        Word{
            is_positive: val.is_positive,
            bytes: [Byte::new(0),Byte::new(0),Byte::new(0), val.bytes[0], val.bytes[1]]
        }
    }

    pub fn read_partial_as_word(&self, l: u8, r: u8) -> Word {
        let is_positive = if l == 0 { self.is_positive } else { true };
        if r > 5 || l > 5 || r < l {
            panic!("Invalid field specifier");
        }
        let val5 = self.bytes[(r-1) as usize].read();
        let val4 = if r >= l+1 && r > 1 { self.bytes[(r-2) as usize].read() } else {0};
        let val3 = if r >= l+2 && r > 2 { self.bytes[(r-3) as usize].read() } else {0};
        let val2 = if r >= l+3 && r > 3{ self.bytes[(r-4) as usize].read() } else {0};
        let val1 = if r >= l+4 && r > 4 { self.bytes[(r-5) as usize].read() } else {0};
        Word::from_values(is_positive, val1, val2, val3, val4, val5)
    }

    pub fn invert_sign(&mut self) {
        self.is_positive = ! self.is_positive
    }
    
    pub fn read(self) -> i32 {
        let modifier: i8 = if self.is_positive { 1 } else { -1 };
        i32::from(modifier) * (((i64::from(self.bytes[0].read()) << 24) + 
                                (i64::from(self.bytes[1].read()) << 18) +
                                (i64::from(self.bytes[2].read()) << 12) +
                                (i64::from(self.bytes[3].read()) << 6) +
                                 i64::from(self.bytes[4].read())) as i32)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HalfWord{
    pub is_positive: bool,
    pub bytes: [Byte; 2]
}

impl HalfWord {
    pub fn new() -> HalfWord {
        HalfWord::from_values(true, 0, 0)
    }

    pub fn from_value(value: i16) -> HalfWord {
        HalfWord::from_values(value.is_positive(), (value.abs() / 64) as u8, (value.abs() % 64) as u8)
    }

    pub fn from_values(is_positive:bool, val1: u8, val2: u8) -> HalfWord {
        HalfWord {
            is_positive: is_positive,
            bytes: [Byte{value: val1}, Byte{value: val2}]
        }
    }

    pub fn read(self) -> i16 {
        let modifier: i8 = if self.is_positive { 1 } else { -1 };
        i16::from(modifier) * (i16::from(self.bytes[0].read()) * 64 + i16::from(self.bytes[1].read()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partial_word_reads() {
        let word = Word::from_values(false, 1, 2, 3, 4, 5);
        assert_eq!(word.read_partial_as_word(0,5), word); 
        assert_eq!(word.read_partial_as_word(1,5), Word::from_values(true, 1,2,3,4,5)); 
        assert_eq!(word.read_partial_as_word(1,2), Word::from_values(true, 0,0,0,1,2)); 
        assert_eq!(word.read_partial_as_word(4,4), Word::from_values(true, 0,0,0,0,4)); 
        assert_eq!(word.read_partial_as_word(0,2), Word::from_values(false, 0,0,0,1,2)); 
    }

}
