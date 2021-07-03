use crate::arch;
use crate::chartable;

struct TapeUnit {
    tape: Vec<arch::Word>,
    position: u16
}

impl TapeUnit { 
    pub fn new() -> TapeUnit{
        TapeUnit { 
            tape: vec![],
            position: 0
        }
    }

    pub fn read(&mut self) -> Vec<arch::Word> {
        use std::iter::FromIterator;
        let values = Vec::from_iter(self.tape[self.position as usize..(self.position as usize + 100)].iter().cloned());
        self.position += 100;
        values
    }
    
    pub fn write(&mut self, values: &[arch::Word]) {
        for index in 0..100 {
            if self.position < self.tape.len() as u16{
                self.tape[self.position as usize] = values[index as usize];
            }
            else {
                self.tape.push(values[index as usize]);
            }
            self.position += 1;
        }
    }
    pub fn advance(&mut self, value: i16) {
        use std::cmp::max;
        self.position = if value == 0 { 0 } else { (self.position as i16 + max(value, -1 * self.position as i16)) as u16};
    }
}

struct Disk { 
    disk: Vec<arch::Word>,
    position: u32
}

impl Disk {
    pub fn new() -> Disk {
        use std::iter::FromIterator;
        Disk {
            disk: Vec::from_iter((0..8192).map(|_| arch::Word::new())),
            position: 0
        }
    }
    
    pub fn read(&mut self, position: u32) -> Vec<arch::Word> {
        use std::iter::FromIterator;
        self.position = position;
        Vec::from_iter(self.disk[self.position as usize..(self.position as usize + 100)].iter().cloned())
    }

    pub fn write(&mut self, position: u32, values: &[arch::Word]) {
        self.position = position;
        for index in 0..100 {
            self.disk[self.position as usize] = values[index as usize];
            self.position += 1;
        }
    }

    pub fn set_position(&mut self, position: u32) {
        self.position = position;
    }

}

struct CardReader {
    cards: Vec<[u8; 80]>
}

impl CardReader {
    pub fn new() -> CardReader {
        CardReader {
            cards: vec![]
        }
    }

    pub fn add_card(&mut self, card: [u8; 80]) {
        self.cards.push(card)
    }

    pub fn read_card(&mut self) -> [u8; 80] {
        let card = self.cards[0];
        self.cards = self.cards[1..].to_vec();
        card
    }
}


pub struct IO {
    tapes: [TapeUnit; 8],
    disks: [Disk; 8],
    card_reader: CardReader
}

impl IO {
    pub fn new() -> IO {
        IO {
            tapes: [TapeUnit::new(), TapeUnit::new(), TapeUnit::new(), TapeUnit::new(), TapeUnit::new(), TapeUnit::new(), TapeUnit::new(), TapeUnit::new()],
            disks: [Disk::new(), Disk::new(), Disk::new(), Disk::new(), Disk::new(), Disk::new(), Disk::new(), Disk::new() ],
            card_reader: CardReader::new()
        }
    }

    pub fn read(&mut self, unit: u8, position_if_disk: u32) -> Vec<arch::Word> {
        if unit <= 7 {
            self.tapes[unit as usize].read()
        }
        else if unit <= 15 {
            self.disks[(unit - 8) as usize].read(position_if_disk)
        }
        else if unit == 16 {
            self.card_reader.read_card()
                            .chunks(5)
                            .map(|b| arch::Word::from_values(true, b[0], b[1], b[2], b[3], b[4]))
                            .collect()
        }
        else {
            panic!("Unreadable unit");
        }
    }

    pub fn write(&mut self, unit: u8, position_if_disk: u32, values: &[arch::Word]) {
        if unit <= 7 {
            self.tapes[unit as usize].write(values)
        }
        else if unit <= 15 {
            self.disks[(unit - 8) as usize].write(position_if_disk, values)
        }
        else if unit == 18 {
            let data: Vec<Vec<u8>> = values[0..24]
                                    .iter()
                                    .map(|x| x.as_u8s())
                                    .collect();
            let bytes: Vec<u8> = data.into_iter().flatten().collect();
            println!("{:?}", chartable::to_char(bytes));
        }
        else {
            panic!("Unwriteable unit");
        }
    }

    pub fn ioctl(&mut self, unit: u8, m_value: i16, x_value: u32) {
        if unit <= 7 {
            self.tapes[unit as usize].advance(m_value);
        }
        else if unit <= 15 {
            self.disks[(unit - 8) as usize].set_position(x_value);
        }
        else {
            panic!("Unable to ioctl unit");
        }
    }

    pub fn add_card(&mut self, data: Vec<u8>) {
        assert!(data.len() <= 80);
        let mut card: [u8; 80] = [0; 80];
        for i in 0..(data.len()) {
            card[i] = data[i]
        }
        self.card_reader.add_card(card)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_tape() {
        let mut io = IO::new();
        for index in 0..100{
            io.tapes[2].tape.push(arch::Word::from_value(index));
        }
        let values = io.read(2, 0);
        assert_eq!(io.tapes[2].position, 100);
        assert_eq!(values.len(), 100);
        for index in 0..100{
            assert_eq!(values[index], arch::Word::from_value(index as i32));
        }
    }
    
    #[test]
    fn can_read_disk() {
        let mut io = IO::new();
        for index in 0..100{
            io.disks[1].disk[1000 + index] = arch::Word::from_value(index as i32);
        }
        let values = io.read(9, 1000);
        assert_eq!(io.disks[1].position, 1000);
        assert_eq!(values.len(), 100);
        for index in 0..100{
            assert_eq!(values[index], arch::Word::from_value(index as i32));
        }
    }
    
    #[test]
    fn can_write_tape() {
        let mut io = IO::new();
        use std::iter::FromIterator;
        let to_write = Vec::from_iter((0..100).map(|i| arch::Word::from_value(i)));
        io.write(7, 0, &to_write);
        assert_eq!(io.tapes[7].position, 100);
        assert_eq!(io.tapes[7].tape.len(), 100);
        for index in 0..100{
            assert_eq!(io.tapes[7].tape[index], arch::Word::from_value(index as i32));
        }
    }
    
    #[test]
    fn can_write_disk() {
        let mut io = IO::new();
        use std::iter::FromIterator;
        let to_write = Vec::from_iter((0..100).map(|i| arch::Word::from_value(i)));
        io.write(15, 10, &to_write);
        assert_eq!(io.disks[7].position, 110);
        for index in 0..100{
            assert_eq!(io.disks[7].disk[10+index], arch::Word::from_value(index as i32));
        }
    }

    #[test]
    fn ioctl_tape(){
        let mut io = IO::new();
        io.tapes[0].position=3;
        io.ioctl(0, 3, 0);
        assert_eq!(io.tapes[0].position, 6);
        io.ioctl(0, -4, 0);
        assert_eq!(io.tapes[0].position, 2);
        io.ioctl(0, 0, 0);
        assert_eq!(io.tapes[0].position, 0);
        io.ioctl(0, -4, 0);
        assert_eq!(io.tapes[0].position, 0);
    }
    
    #[test]
    fn ioctl_disk(){
        let mut io = IO::new();
        io.disks[0].position=3;
        io.ioctl(8, 0, 3);
        assert_eq!(io.disks[0].position, 3);
        io.ioctl(8, 0, 13);
        assert_eq!(io.disks[0].position, 13);
    }
    
}
