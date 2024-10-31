use advent::prelude::*;

fn parse_input(input: &str) -> Vec<i64> {
    todo!("parsing")
}

fn default_input() -> Vec<i64> {
    parse_input(include_input!(2016 / 05))
}

fn part1(input: Vec<i64>) -> i64 {
    todo!("part 1")
}

fn part2(input: Vec<i64>) -> i64 {
    todo!("part 2")
}

pub fn buffer_size(n: usize) -> usize {
    (n + 9).next_multiple_of(64)
}

pub fn hash(mut buffer: &mut [u8], size: usize) -> (u32, u32, u32, u32) {
    let end = buffer.len() - 8;
    let bits = size * 8;

    buffer[size] = 0x80;
    buffer[end..].copy_from_slice(&bits.to_le_bytes());

    let mut m = [0; 16];
    let mut a0: u32 = 0x67452301;
    let mut b0: u32 = 0xefcdab89;
    let mut c0: u32 = 0x98badcfe;
    let mut d0: u32 = 0x10325476;

    while !buffer.is_empty() {
        let (prefix, suffix) = buffer.split_at_mut(64);
        buffer = suffix;

        for (i, chunk) in prefix.chunks_exact(4).enumerate() {
            m[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        }

        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        a = round1(a, b, c, d, m[0], 7, 0xd76aa478);
        d = round1(d, a, b, c, m[1], 12, 0xe8c7b756);
        c = round1(c, d, a, b, m[2], 17, 0x242070db);
        b = round1(b, c, d, a, m[3], 22, 0xc1bdceee);
        a = round1(a, b, c, d, m[4], 7, 0xf57c0faf);
        d = round1(d, a, b, c, m[5], 12, 0x4787c62a);
        c = round1(c, d, a, b, m[6], 17, 0xa8304613);
        b = round1(b, c, d, a, m[7], 22, 0xfd469501);
        a = round1(a, b, c, d, m[8], 7, 0x698098d8);
        d = round1(d, a, b, c, m[9], 12, 0x8b44f7af);
        c = round1(c, d, a, b, m[10], 17, 0xffff5bb1);
        b = round1(b, c, d, a, m[11], 22, 0x895cd7be);
        a = round1(a, b, c, d, m[12], 7, 0x6b901122);
        d = round1(d, a, b, c, m[13], 12, 0xfd987193);
        c = round1(c, d, a, b, m[14], 17, 0xa679438e);
        b = round1(b, c, d, a, m[15], 22, 0x49b40821);

        a = round2(a, b, c, d, m[1], 5, 0xf61e2562);
        d = round2(d, a, b, c, m[6], 9, 0xc040b340);
        c = round2(c, d, a, b, m[11], 14, 0x265e5a51);
        b = round2(b, c, d, a, m[0], 20, 0xe9b6c7aa);
        a = round2(a, b, c, d, m[5], 5, 0xd62f105d);
        d = round2(d, a, b, c, m[10], 9, 0x02441453);
        c = round2(c, d, a, b, m[15], 14, 0xd8a1e681);
        b = round2(b, c, d, a, m[4], 20, 0xe7d3fbc8);
        a = round2(a, b, c, d, m[9], 5, 0x21e1cde6);
        d = round2(d, a, b, c, m[14], 9, 0xc33707d6);
        c = round2(c, d, a, b, m[3], 14, 0xf4d50d87);
        b = round2(b, c, d, a, m[8], 20, 0x455a14ed);
        a = round2(a, b, c, d, m[13], 5, 0xa9e3e905);
        d = round2(d, a, b, c, m[2], 9, 0xfcefa3f8);
        c = round2(c, d, a, b, m[7], 14, 0x676f02d9);
        b = round2(b, c, d, a, m[12], 20, 0x8d2a4c8a);

        a = round3(a, b, c, d, m[5], 4, 0xfffa3942);
        d = round3(d, a, b, c, m[8], 11, 0x8771f681);
        c = round3(c, d, a, b, m[11], 16, 0x6d9d6122);
        b = round3(b, c, d, a, m[14], 23, 0xfde5380c);
        a = round3(a, b, c, d, m[1], 4, 0xa4beea44);
        d = round3(d, a, b, c, m[4], 11, 0x4bdecfa9);
        c = round3(c, d, a, b, m[7], 16, 0xf6bb4b60);
        b = round3(b, c, d, a, m[10], 23, 0xbebfbc70);
        a = round3(a, b, c, d, m[13], 4, 0x289b7ec6);
        d = round3(d, a, b, c, m[0], 11, 0xeaa127fa);
        c = round3(c, d, a, b, m[3], 16, 0xd4ef3085);
        b = round3(b, c, d, a, m[6], 23, 0x04881d05);
        a = round3(a, b, c, d, m[9], 4, 0xd9d4d039);
        d = round3(d, a, b, c, m[12], 11, 0xe6db99e5);
        c = round3(c, d, a, b, m[15], 16, 0x1fa27cf8);
        b = round3(b, c, d, a, m[2], 23, 0xc4ac5665);

        a = round4(a, b, c, d, m[0], 6, 0xf4292244);
        d = round4(d, a, b, c, m[7], 10, 0x432aff97);
        c = round4(c, d, a, b, m[14], 15, 0xab9423a7);
        b = round4(b, c, d, a, m[5], 21, 0xfc93a039);
        a = round4(a, b, c, d, m[12], 6, 0x655b59c3);
        d = round4(d, a, b, c, m[3], 10, 0x8f0ccc92);
        c = round4(c, d, a, b, m[10], 15, 0xffeff47d);
        b = round4(b, c, d, a, m[1], 21, 0x85845dd1);
        a = round4(a, b, c, d, m[8], 6, 0x6fa87e4f);
        d = round4(d, a, b, c, m[15], 10, 0xfe2ce6e0);
        c = round4(c, d, a, b, m[6], 15, 0xa3014314);
        b = round4(b, c, d, a, m[13], 21, 0x4e0811a1);
        a = round4(a, b, c, d, m[4], 6, 0xf7537e82);
        d = round4(d, a, b, c, m[11], 10, 0xbd3af235);
        c = round4(c, d, a, b, m[2], 15, 0x2ad7d2bb);
        b = round4(b, c, d, a, m[9], 21, 0xeb86d391);

        a0 = a0.wrapping_add(a);
        b0 = b0.wrapping_add(b);
        c0 = c0.wrapping_add(c);
        d0 = d0.wrapping_add(d);
    }

    (a0.to_be(), b0.to_be(), c0.to_be(), d0.to_be())
}

#[inline]
fn round1(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = (b & c) | (!b & d);
    common(f, a, b, m, s, k)
}

#[inline]
fn round2(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = (b & d) | (c & !d);
    common(f, a, b, m, s, k)
}

#[inline]
fn round3(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = b ^ c ^ d;
    common(f, a, b, m, s, k)
}

#[inline]
fn round4(a: u32, b: u32, c: u32, d: u32, m: u32, s: u32, k: u32) -> u32 {
    let f = c ^ (b | !d);
    common(f, a, b, m, s, k)
}

#[inline]
fn common(f: u32, a: u32, b: u32, m: u32, s: u32, k: u32) -> u32 {
    f.wrapping_add(a).wrapping_add(k).wrapping_add(m).rotate_left(s).wrapping_add(b)
}

#[cfg(feature = "simd")]
pub mod simd {
    use std::array;
    use std::simd::num::SimdUint;
    use std::simd::{LaneCount, Simd, SupportedLaneCount};

    #[inline]
    #[expect(clippy::too_many_lines)]
    pub fn hash<const N: usize>(
        buffers: &mut [[u8; 64]],
        size: usize,
    ) -> ([u32; N], [u32; N], [u32; N], [u32; N])
        where
            LaneCount<N>: SupportedLaneCount,
    {
        // Assume all buffers are the same size.
        let end = 64 - 8;
        let bits = size * 8;

        for buffer in buffers.iter_mut() {
            buffer[size] = 0x80;
            buffer[end..].copy_from_slice(&bits.to_le_bytes());
        }

        let mut a0: Simd<u32, N> = Simd::splat(0x67452301);
        let mut b0: Simd<u32, N> = Simd::splat(0xefcdab89);
        let mut c0: Simd<u32, N> = Simd::splat(0x98badcfe);
        let mut d0: Simd<u32, N> = Simd::splat(0x10325476);

        let mut a = a0;
        let mut b = b0;
        let mut c = c0;
        let mut d = d0;

        let m0 = message(buffers, 0);
        a = round1(a, b, c, d, m0, 7, 0xd76aa478);
        let m1 = message(buffers, 1);
        d = round1(d, a, b, c, m1, 12, 0xe8c7b756);
        let m2 = message(buffers, 2);
        c = round1(c, d, a, b, m2, 17, 0x242070db);
        let m3 = message(buffers, 3);
        b = round1(b, c, d, a, m3, 22, 0xc1bdceee);
        let m4 = message(buffers, 4);
        a = round1(a, b, c, d, m4, 7, 0xf57c0faf);
        let m5 = message(buffers, 5);
        d = round1(d, a, b, c, m5, 12, 0x4787c62a);
        let m6 = message(buffers, 6);
        c = round1(c, d, a, b, m6, 17, 0xa8304613);
        let m7 = message(buffers, 7);
        b = round1(b, c, d, a, m7, 22, 0xfd469501);
        let m8 = message(buffers, 8);
        a = round1(a, b, c, d, m8, 7, 0x698098d8);
        let m9 = message(buffers, 9);
        d = round1(d, a, b, c, m9, 12, 0x8b44f7af);
        let m10 = message(buffers, 10);
        c = round1(c, d, a, b, m10, 17, 0xffff5bb1);
        let m11 = message(buffers, 11);
        b = round1(b, c, d, a, m11, 22, 0x895cd7be);
        let m12 = message(buffers, 12);
        a = round1(a, b, c, d, m12, 7, 0x6b901122);
        let m13 = message(buffers, 13);
        d = round1(d, a, b, c, m13, 12, 0xfd987193);
        let m14 = message(buffers, 14);
        c = round1(c, d, a, b, m14, 17, 0xa679438e);
        let m15 = message(buffers, 15);
        b = round1(b, c, d, a, m15, 22, 0x49b40821);

        a = round2(a, b, c, d, m1, 5, 0xf61e2562);
        d = round2(d, a, b, c, m6, 9, 0xc040b340);
        c = round2(c, d, a, b, m11, 14, 0x265e5a51);
        b = round2(b, c, d, a, m0, 20, 0xe9b6c7aa);
        a = round2(a, b, c, d, m5, 5, 0xd62f105d);
        d = round2(d, a, b, c, m10, 9, 0x02441453);
        c = round2(c, d, a, b, m15, 14, 0xd8a1e681);
        b = round2(b, c, d, a, m4, 20, 0xe7d3fbc8);
        a = round2(a, b, c, d, m9, 5, 0x21e1cde6);
        d = round2(d, a, b, c, m14, 9, 0xc33707d6);
        c = round2(c, d, a, b, m3, 14, 0xf4d50d87);
        b = round2(b, c, d, a, m8, 20, 0x455a14ed);
        a = round2(a, b, c, d, m13, 5, 0xa9e3e905);
        d = round2(d, a, b, c, m2, 9, 0xfcefa3f8);
        c = round2(c, d, a, b, m7, 14, 0x676f02d9);
        b = round2(b, c, d, a, m12, 20, 0x8d2a4c8a);

        a = round3(a, b, c, d, m5, 4, 0xfffa3942);
        d = round3(d, a, b, c, m8, 11, 0x8771f681);
        c = round3(c, d, a, b, m11, 16, 0x6d9d6122);
        b = round3(b, c, d, a, m14, 23, 0xfde5380c);
        a = round3(a, b, c, d, m1, 4, 0xa4beea44);
        d = round3(d, a, b, c, m4, 11, 0x4bdecfa9);
        c = round3(c, d, a, b, m7, 16, 0xf6bb4b60);
        b = round3(b, c, d, a, m10, 23, 0xbebfbc70);
        a = round3(a, b, c, d, m13, 4, 0x289b7ec6);
        d = round3(d, a, b, c, m0, 11, 0xeaa127fa);
        c = round3(c, d, a, b, m3, 16, 0xd4ef3085);
        b = round3(b, c, d, a, m6, 23, 0x04881d05);
        a = round3(a, b, c, d, m9, 4, 0xd9d4d039);
        d = round3(d, a, b, c, m12, 11, 0xe6db99e5);
        c = round3(c, d, a, b, m15, 16, 0x1fa27cf8);
        b = round3(b, c, d, a, m2, 23, 0xc4ac5665);

        a = round4(a, b, c, d, m0, 6, 0xf4292244);
        d = round4(d, a, b, c, m7, 10, 0x432aff97);
        c = round4(c, d, a, b, m14, 15, 0xab9423a7);
        b = round4(b, c, d, a, m5, 21, 0xfc93a039);
        a = round4(a, b, c, d, m12, 6, 0x655b59c3);
        d = round4(d, a, b, c, m3, 10, 0x8f0ccc92);
        c = round4(c, d, a, b, m10, 15, 0xffeff47d);
        b = round4(b, c, d, a, m1, 21, 0x85845dd1);
        a = round4(a, b, c, d, m8, 6, 0x6fa87e4f);
        d = round4(d, a, b, c, m15, 10, 0xfe2ce6e0);
        c = round4(c, d, a, b, m6, 15, 0xa3014314);
        b = round4(b, c, d, a, m13, 21, 0x4e0811a1);
        a = round4(a, b, c, d, m4, 6, 0xf7537e82);
        d = round4(d, a, b, c, m11, 10, 0xbd3af235);
        c = round4(c, d, a, b, m2, 15, 0x2ad7d2bb);
        b = round4(b, c, d, a, m9, 21, 0xeb86d391);

        a0 += a;
        b0 += b;
        c0 += c;
        d0 += d;

        (
            a0.swap_bytes().to_array(),
            b0.swap_bytes().to_array(),
            c0.swap_bytes().to_array(),
            d0.swap_bytes().to_array(),
        )
    }

    #[inline]
    fn message<const N: usize>(buffers: &mut [[u8; 64]], i: usize) -> Simd<u32, N>
        where
            LaneCount<N>: SupportedLaneCount,
    {
        let start = 4 * i;
        let end = start + 4;
        Simd::from_array(array::from_fn(|lane| {
            let slice = &buffers[lane][start..end];
            u32::from_le_bytes(slice.try_into().unwrap())
        }))
    }

    #[inline]
    fn round1<const N: usize>(
        a: Simd<u32, N>,
        b: Simd<u32, N>,
        c: Simd<u32, N>,
        d: Simd<u32, N>,
        m: Simd<u32, N>,
        s: u32,
        k: u32,
    ) -> Simd<u32, N>
        where
            LaneCount<N>: SupportedLaneCount,
    {
        let f = (b & c) | (!b & d);
        common(f, a, b, m, s, k)
    }

    #[inline]
    fn round2<const N: usize>(
        a: Simd<u32, N>,
        b: Simd<u32, N>,
        c: Simd<u32, N>,
        d: Simd<u32, N>,
        m: Simd<u32, N>,
        s: u32,
        k: u32,
    ) -> Simd<u32, N>
        where
            LaneCount<N>: SupportedLaneCount,
    {
        let f = (b & d) | (c & !d);
        common(f, a, b, m, s, k)
    }

    #[inline]
    fn round3<const N: usize>(
        a: Simd<u32, N>,
        b: Simd<u32, N>,
        c: Simd<u32, N>,
        d: Simd<u32, N>,
        m: Simd<u32, N>,
        s: u32,
        k: u32,
    ) -> Simd<u32, N>
        where
            LaneCount<N>: SupportedLaneCount,
    {
        let f = b ^ c ^ d;
        common(f, a, b, m, s, k)
    }

    #[inline]
    fn round4<const N: usize>(
        a: Simd<u32, N>,
        b: Simd<u32, N>,
        c: Simd<u32, N>,
        d: Simd<u32, N>,
        m: Simd<u32, N>,
        s: u32,
        k: u32,
    ) -> Simd<u32, N>
        where
            LaneCount<N>: SupportedLaneCount,
    {
        let f = c ^ (b | !d);
        common(f, a, b, m, s, k)
    }

    #[inline]
    fn common<const N: usize>(
        f: Simd<u32, N>,
        a: Simd<u32, N>,
        b: Simd<u32, N>,
        m: Simd<u32, N>,
        s: u32,
        k: u32,
    ) -> Simd<u32, N>
        where
            LaneCount<N>: SupportedLaneCount,
    {
        let k = Simd::splat(k);
        let first = f + a + k + m;
        let second = (first << s) | (first >> (32 - s));
        second + b
    }
}

fn main() {
    let solution = advent::new(default_input).part(part1).part(part2).build();
    solution.cli()
}

#[test]
fn default() {
    let input = default_input();
    assert_eq!(part1(input.clone()), 1);
    assert_eq!(part2(input), 2);
}