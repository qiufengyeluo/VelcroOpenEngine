#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]


pub struct Sha1 {
    _h: [u32; 5],
    _block: [u8; 64],
    _block_byte_index: usize,
    _byte_count: usize,
}

impl Sha1 {
    pub fn reset(&mut self) {
        self._h[0] = 0x6745_2301;
        self._h[1] = 0xEFCD_AB89;
        self._h[2] = 0x98BA_DCFE;
        self._h[3] = 0x1032_5476;
        self._h[4] = 0xC3D2_E1F0;

        self._block_byte_index = 0;
        self._byte_count = 0;
    }

    pub fn process_byte(&mut self, b: u8) {
        self._block[self._block_byte_index] = b;
        self._block_byte_index += 1;
        self._byte_count += 1;

        if self._block_byte_index == 64 {
            self._block_byte_index = 0;
            self.process_block();
        }
    }

    fn process_block(&mut self) {
    
        let mut w: [u32; 80] = [0; 80];
        for i in 0..16 {
            w[i]  = (self._block[i.wrapping_mul(4) as usize] as u32) << 24;
            w[i] |= (self._block[(i.wrapping_mul(4) as usize).wrapping_add(1)] as u32) << 16;
            w[i] |= (self._block[(i.wrapping_mul(4) as usize).wrapping_add(2)] as u32) << 8;
            w[i] |= self._block[(i.wrapping_mul(4) as usize).wrapping_add(3)] as u32;
        }

        for i in 16..80 {
            w[i] = Self::left_rotate(w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16], 1);
        }

        let mut a = self._h[0];
        let mut b = self._h[1];
        let mut c = self._h[2];
        let mut d = self._h[3];
        let mut e = self._h[4];

        for i in 0..80 {
            let mut f: u32 = 0;
            let mut k: u32 = 0;

            if i < 20 {
                f = b & c | (!b) & d;
                k = 0x5A82_7999;
                
            } else if i < 40 {
                f = b ^ c ^ d;
                k = 0x6ED9_EBA1;
            } else if i < 60 {
                f = (b & c) | (b & d) | (c & d);
                k = 0x8F1B_BCDC;
            } else if i >= 60 {
                f = b ^ c ^ d;
                k = 0xCA62_C1D6;
            }

            let temp = Self::left_rotate(a, 5) + f + e + k + w[i];

            e = d; 
            d = c;
            c = Self::left_rotate(b, 30);
            b = a;
            a = temp;
        }

        self._h[0] += a;
        self._h[1] += b;
        self._h[2] += c;
        self._h[3] += d;
        self._h[4] += e;
    }

    fn left_rotate(x: u32, n: usize) -> u32 {
        return (x << n) ^ (x >> (32 - n));
    }
}
