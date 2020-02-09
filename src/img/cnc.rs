pub struct CRC {
    crc_table: [u32; 256],
    crc_table_computed: i32
}

impl CRC {
    pub fn new() -> Self {

        return Self {crc_table: [0; 256], crc_table_computed: 0};
    }

    /* Make the table for a fast CRC. */
    fn make_crc_table (&mut self)
    {
        let mut c: u32 = 0;

        for n in 0..256 {
            c ^= n << 24;
            for _k in 0..8 {
                if (c & 1) != 0 {
                    c = 0xEDB88320u32 ^ ((c >> 1) & 0x7FFFFFFF);
                } else {
                    c = (c >> 1) & 0x7FFFFFFF;
                }
            }
            self.crc_table[n as usize] = c;
        }
        self.crc_table_computed = 1;
    }

    /* Update a running CRC with the bytes buf[0..len-1]--the CRC
        should be initialized to all 1's, and the transmitted value
        is the 1's complement of the final running CRC (see the
        crc() routine below). */

    fn update_crc(&mut self, crc: u32, buf: &mut Vec<u8>, len: i32) -> u32 {
        let mut c: u32 = crc;

        if self.crc_table_computed == 0 {
            CRC::make_crc_table(self);
        }
        for n in 0..len {
            c = self.crc_table[((c ^ (buf[n as usize] as u32)) & 0xff) as usize] ^ ((c >> 8) & 0xffffffffu32);
        }
        return c;
    }

    /* Return the CRC of the bytes buf[0..len-1]. */
    pub fn crc(mut self, buf: &mut Vec<u8>) -> u32 {
        return CRC::update_crc(&mut self, 0xffffffffu32, buf, buf.len() as i32) ^ 0xffffffffu32;
    }
}