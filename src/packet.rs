
const SEGMENT_BITS: u32 = 0x7F;
const CONTINUE_BIT: u32 = 0x80;

pub struct PacketByteBuf {
    raw: Box<[u8]>,
    reader_idx: usize,
}

impl PacketByteBuf {
    pub fn new(data: &[u8]) -> Self {
        PacketByteBuf {
            raw: Box::from(data),
            reader_idx: 0
        }
    }

    pub fn read_byte(&mut self) -> u8 {
        let a = self.raw[self.reader_idx];
        self.reader_idx += 1;
        a
    }

    pub fn read_varint(&mut self) -> u32 {
        let mut value = 0u32;
        let mut position: u8 = 0;

        loop {
            let current_byte = self.read_byte() as u32;
            value |= ((current_byte & SEGMENT_BITS) << position) as u32;

            if (current_byte & CONTINUE_BIT) == 0 {
                break
            }

            position += 7;

            if position >= 32 {
                panic!("Too big VarInt");
            }
        }

        return value;
    }

    pub fn read_string(&mut self) -> Vec<u8> {
        let length = self.read_varint();
        let mut buff: Vec<u8> = Vec::new();

        for i in 0..length {
            buff.insert(i as usize, self.read_byte())
        }

        buff
    }

    pub fn get_reader_idx(&self) -> u32 {
        self.reader_idx as u32
    }

    pub fn read_ushort(&mut self) -> u16 {
        let first = self.read_byte() as u16;
        let second = self.read_byte() as u16;

        (first << 8) + second
    }
}

pub struct MutablePacketByteBuf {
    bytes: Vec<u8>,
    idx: usize
}

impl MutablePacketByteBuf {
    pub fn new() -> Self {
        MutablePacketByteBuf {
            bytes: Vec::new(),
            idx: 0
        }
    }

    pub fn write_byte(&mut self, b: u8) {
        self.bytes.insert(self.idx, b);
        self.idx += 1;
    }

    pub fn write_varint(&mut self, mut value: u32) {
        loop {
            if (value & 0x80 as u32) == 0 {
                self.write_byte(value as u8);
                return;
            }

            self.write_byte(((value & 0x7f as u32) | 0x80 as u32) as u8);
            value = value >> 7;
        }
    }

    fn prepend_length(&mut self) {
        let mut size = self.bytes.len() as u32;
        let mut buff: Vec<u8> = Vec::new();
        loop {
            if (size & CONTINUE_BIT as u32) == 0 {
                buff.insert(buff.len(), size as u8);
                for b in 0..buff.len() {
                    self.bytes.insert(b, buff.as_slice()[b])
                }
                return;
            }

            buff.insert(buff.len(), ((size & SEGMENT_BITS as u32) | CONTINUE_BIT as u32) as u8);
            size = size >> 7;
        }
    }

    pub fn write_string(&mut self, val: &str) {
        let l = val.len();
        self.write_varint(l as u32);
        for b in val.as_bytes() {
            self.write_byte(*b);
        }
    }

    pub fn get_bytes(&mut self) -> Vec<u8> {
        self.prepend_length();
        self.bytes.clone()
    }
}
