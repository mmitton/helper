use crate::Error;
use std::str::FromStr;

const S: [u32; 64] = [
    7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9,
    14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15,
    21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
];

const K: [u32; 64] = [
    0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
    0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
    0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
    0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
    0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
    0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05, 0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
    0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
    0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
];

const A: u32 = 0x67452301;
const B: u32 = 0xefcdab89;
const C: u32 = 0x98badcfe;
const D: u32 = 0x10325476;

#[derive(Copy, Clone)]
pub struct MD5String {
    bytes: [u8; 64],
    len: usize,
}

impl Default for MD5String {
    fn default() -> Self {
        Self {
            bytes: [
                0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            len: 0,
        }
    }
}

impl MD5String {
    const MAX_LEN: usize = 55;

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn truncate_without_zero(&mut self, len: usize) {
        use std::cmp::Ordering;
        match len.cmp(&self.len) {
            Ordering::Equal => {}
            Ordering::Greater => {
                panic!("New len({len}) is greater than existing len({})", self.len)
            }
            Ordering::Less => {
                self.len = len;
            }
        }
    }

    pub fn truncate(&mut self, len: usize) {
        use std::cmp::Ordering;
        match len.cmp(&self.len) {
            Ordering::Equal => {}
            Ordering::Greater => {
                panic!("New len({len}) is greater than existing len({})", self.len)
            }
            Ordering::Less => {
                self.bytes[len..=self.len].iter_mut().for_each(|b| *b = 0);
                self.len = len;
            }
        }
    }

    pub fn push_bytes(&mut self, bytes: &[u8]) -> Result<(), Error> {
        let new_len = self.len + bytes.len();
        if new_len > Self::MAX_LEN {
            Err(Error::MD5StringOverrun)
        } else {
            self.bytes[self.len..new_len].copy_from_slice(bytes);
            self.len = new_len;
            Ok(())
        }
    }

    pub fn push_str(&mut self, s: &str) -> Result<(), Error> {
        self.push_bytes(s.as_bytes())
    }

    pub fn push(&mut self, ch: char) -> Result<(), Error> {
        match ch.len_utf8() {
            1 => {
                let new_len = self.len + 1;
                if new_len > Self::MAX_LEN {
                    Err(Error::MD5StringOverrun)
                } else {
                    self.bytes[self.len] = ch as u8;
                    self.len = new_len;
                    Ok(())
                }
            }
            _ => {
                let mut buf = [0; 4];
                let bytes = ch.encode_utf8(&mut buf).as_bytes();
                self.push_bytes(bytes)
            }
        }
    }

    pub fn digest(&mut self) -> [u8; 16] {
        self.bytes[self.len] = 0x80;
        self.bytes[57] = (self.len >> 5) as u8;
        self.bytes[56] = (self.len << 3) as u8;

        let mut a0 = A;
        let mut b0 = B;
        let mut c0 = C;
        let mut d0 = D;

        let b = &self.bytes;
        let chunks: [u32; 16] = [
            u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
            u32::from_le_bytes([b[4], b[5], b[6], b[7]]),
            u32::from_le_bytes([b[8], b[9], b[10], b[11]]),
            u32::from_le_bytes([b[12], b[13], b[14], b[15]]),
            u32::from_le_bytes([b[16], b[17], b[18], b[19]]),
            u32::from_le_bytes([b[20], b[21], b[22], b[23]]),
            u32::from_le_bytes([b[24], b[25], b[26], b[27]]),
            u32::from_le_bytes([b[28], b[29], b[30], b[31]]),
            u32::from_le_bytes([b[32], b[33], b[34], b[35]]),
            u32::from_le_bytes([b[36], b[37], b[38], b[39]]),
            u32::from_le_bytes([b[40], b[41], b[42], b[43]]),
            u32::from_le_bytes([b[44], b[45], b[46], b[47]]),
            u32::from_le_bytes([b[48], b[49], b[50], b[51]]),
            u32::from_le_bytes([b[52], b[53], b[54], b[55]]),
            u32::from_le_bytes([b[56], b[57], b[58], b[59]]),
            u32::from_le_bytes([b[60], b[61], b[62], b[63]]),
        ];

        md5_mix(&chunks, &mut a0, &mut b0, &mut c0, &mut d0);

        let mut res = [0; 16];
        res[0..4].copy_from_slice(&a0.to_le_bytes());
        res[4..8].copy_from_slice(&b0.to_le_bytes());
        res[8..12].copy_from_slice(&c0.to_le_bytes());
        res[12..16].copy_from_slice(&d0.to_le_bytes());
        res
    }
}

impl FromStr for MD5String {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > Self::MAX_LEN {
            Err(Error::MD5StringOverrun)
        } else {
            let bytes = s.as_bytes();
            let mut s = Self::default();
            s.bytes[0..bytes.len()].copy_from_slice(bytes);
            s.len = bytes.len();
            Ok(s)
        }
    }
}

impl std::fmt::Write for MD5String {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.push_str(s).map_err(|_| std::fmt::Error)
    }

    fn write_char(&mut self, ch: char) -> std::fmt::Result {
        self.push(ch).map_err(|_| std::fmt::Error)
    }
}

pub struct MD5 {}

enum PayloadBytes<'a> {
    Bytes(&'a [u8]),
    LenPayload,
    Done,
}

struct Payload<'a> {
    payload: PayloadBytes<'a>,
    bytes: [u8; 64],
    chunks: [u32; 16],
    len: [u8; 8],
}

impl<'a> Payload<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        let len = bytes.len() as u64 * 8;
        Self {
            payload: PayloadBytes::Bytes(bytes),
            bytes: [0; 64],
            chunks: [0; 16],
            len: len.to_le_bytes(),
        }
    }

    #[inline]
    fn fill(&mut self) -> bool {
        match &mut self.payload {
            PayloadBytes::Bytes(bytes) => {
                if bytes.len() >= 64 {
                    for i in 0..16 {
                        self.chunks[i] = u32::from_le_bytes([
                            bytes[i * 4],
                            bytes[i * 4 + 1],
                            bytes[i * 4 + 2],
                            bytes[i * 4 + 3],
                        ]);
                    }
                    // self.bytes.copy_from_slice(&bytes[0..64]);
                    *bytes = &bytes[64..];
                    return true;
                } else {
                    self.bytes.as_mut_slice()[..bytes.len()].copy_from_slice(bytes);
                    self.bytes[bytes.len()] = 0x80;
                    let last = bytes.len() + 1;
                    if last <= 56 {
                        self.bytes.as_mut_slice()[last..56]
                            .iter_mut()
                            .for_each(|b| *b = 0);
                        self.bytes.as_mut_slice()[56..].copy_from_slice(&self.len);
                        self.payload = PayloadBytes::Done;
                    } else {
                        self.bytes.as_mut_slice()[last..]
                            .iter_mut()
                            .for_each(|b| *b = 0);
                        *bytes = &bytes[bytes.len()..];
                        self.payload = PayloadBytes::LenPayload;
                    }
                }
            }
            PayloadBytes::LenPayload => {
                // Marker was placed in the last payload
                self.bytes.as_mut_slice()[..56]
                    .iter_mut()
                    .for_each(|b| *b = 0);
                self.bytes.as_mut_slice()[56..].copy_from_slice(&self.len);
                self.payload = PayloadBytes::Done;
            }
            PayloadBytes::Done => return false,
        }

        let b = &self.bytes;
        self.chunks[0] = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        self.chunks[1] = u32::from_le_bytes([b[4], b[5], b[6], b[7]]);
        self.chunks[2] = u32::from_le_bytes([b[8], b[9], b[10], b[11]]);
        self.chunks[3] = u32::from_le_bytes([b[12], b[13], b[14], b[15]]);
        self.chunks[4] = u32::from_le_bytes([b[16], b[17], b[18], b[19]]);
        self.chunks[5] = u32::from_le_bytes([b[20], b[21], b[22], b[23]]);
        self.chunks[6] = u32::from_le_bytes([b[24], b[25], b[26], b[27]]);
        self.chunks[7] = u32::from_le_bytes([b[28], b[29], b[30], b[31]]);
        self.chunks[8] = u32::from_le_bytes([b[32], b[33], b[34], b[35]]);
        self.chunks[9] = u32::from_le_bytes([b[36], b[37], b[38], b[39]]);
        self.chunks[10] = u32::from_le_bytes([b[40], b[41], b[42], b[43]]);
        self.chunks[11] = u32::from_le_bytes([b[44], b[45], b[46], b[47]]);
        self.chunks[12] = u32::from_le_bytes([b[48], b[49], b[50], b[51]]);
        self.chunks[13] = u32::from_le_bytes([b[52], b[53], b[54], b[55]]);
        self.chunks[14] = u32::from_le_bytes([b[56], b[57], b[58], b[59]]);
        self.chunks[15] = u32::from_le_bytes([b[60], b[61], b[62], b[63]]);

        true
    }
}

#[inline(always)]
fn md5_mix(chunks: &[u32; 16], a0: &mut u32, b0: &mut u32, c0: &mut u32, d0: &mut u32) {
    let mut a = *a0;
    let mut b = *b0;
    let mut c = *c0;
    let mut d = *d0;

    macro_rules! tail {
        ($f:ident, $g:ident, $i:ident) => {{
            let f = $f
                .wrapping_add(a)
                .wrapping_add(K[$i])
                .wrapping_add(chunks[$g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[$i]));
        }};
    }

    for i in 0..16 {
        let f = (b & c) | (!b & d);
        let g = i;
        tail!(f, g, i);
    }
    for i in 16..32 {
        let f = (d & b) | (!d & c);
        let g = (5 * i + 1) % 16;
        tail!(f, g, i);
    }
    for i in 32..48 {
        let f = b ^ c ^ d;
        let g = (3 * i + 5) % 16;
        tail!(f, g, i);
    }
    for i in 48..64 {
        let f = c ^ (b | !d);
        let g = (7 * i) % 16;
        tail!(f, g, i);
    }

    *a0 = a0.wrapping_add(a);
    *b0 = b0.wrapping_add(b);
    *c0 = c0.wrapping_add(c);
    *d0 = d0.wrapping_add(d);
}

impl MD5 {
    #[inline]
    pub fn digest(bytes: &[u8]) -> [u8; 16] {
        let mut a0 = A;
        let mut b0 = B;
        let mut c0 = C;
        let mut d0 = D;
        let mut payload = Payload::new(bytes);
        while payload.fill() {
            md5_mix(&payload.chunks, &mut a0, &mut b0, &mut c0, &mut d0);
        }

        let mut res = [0; 16];
        res[0..4].copy_from_slice(&a0.to_le_bytes());
        res[4..8].copy_from_slice(&b0.to_le_bytes());
        res[8..12].copy_from_slice(&c0.to_le_bytes());
        res[12..16].copy_from_slice(&d0.to_le_bytes());
        res
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use crate::{MD5String, MD5};

    #[test]
    fn md5_known_digests() {
        let tests = [
            ("1", "c4ca4238a0b923820dcc509a6f75849b"),
            (
                "The quick brown fox jumps over the lazy dog",
                "9e107d9d372bb6826bd81d3542a419d6",
            ),
            (
                "The quick brown fox jumps over the lazy dog.",
                "e4d909c290d0fb1ca068ffaddf22cbd0",
            ),
            ("", "d41d8cd98f00b204e9800998ecf8427e"),
        ];
        for (s, e) in tests.iter() {
            use std::fmt::Write;
            let digest = MD5::digest(s.as_bytes());
            let digest_str: String = digest.iter().fold(String::new(), |mut s, b| {
                let _ = write!(s, "{b:02x}");
                s
            });
            assert_eq!(digest_str, *e, "{s:?} did not producted expected MD5");

            let mut md5string = MD5String::from_str(s).unwrap();
            let new_digest = md5string.digest();
            let new_digest_str: String = new_digest.iter().fold(String::new(), |mut s, b| {
                let _ = write!(s, "{b:02x}");
                s
            });
            assert_eq!(
                digest_str, new_digest_str,
                "{s:?} did not producted expected MD5 with MD5String {:02x?}",
                md5string.bytes
            );
        }
    }

    #[test]
    fn md5_compare() {
        const TOP: usize = 4 * 1024;
        let mut buf = Vec::with_capacity(TOP);
        for i in 0..TOP {
            println!();
            println!("{i}");
            let a = MD5::digest(&buf);
            let b = md5::compute(&buf);
            assert_eq!(a, b.0, "{buf:?}");
            buf.push((i % 0xFF) as u8);
        }
    }

    #[test]
    fn md5_bench_internal() {
        const ITERS: usize = 50_000;
        let payload: [u8; 4 * 1024] = std::array::from_fn(|i| i as u8);
        println!();

        // Benchmark internal MD5
        let start = std::time::Instant::now();
        for _ in 0..ITERS {
            MD5::digest(&payload);
        }
        let elapsed = start.elapsed();
        println!(
            "Internal MD5 processed {ITERS} in {elapsed:?}.  {} iters per second",
            ITERS as f64 / elapsed.as_secs_f64()
        );

        // Benchmark md5 crate
        let start = std::time::Instant::now();
        for _ in 0..ITERS {
            md5::compute(payload);
        }
        let elapsed = start.elapsed();
        println!(
            "MD5 crate processed {ITERS} in {elapsed:?}.  {} iters per second",
            ITERS as f64 / elapsed.as_secs_f64()
        );
    }
}
