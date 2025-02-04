use std::{fmt::Display, ops::{Add, AddAssign, Deref, DerefMut, Div, Mul, Not, Rem, Shl, Shr, Sub, SubAssign}};

const BIG_INT_SIZE: usize = 128;

#[derive(Clone, Copy, PartialEq)]
struct BigInt([u8; BIG_INT_SIZE]);

impl BigInt {
    pub const MIN: Self = BigInt([0x00; BIG_INT_SIZE]);
    pub const MAX: Self = BigInt([0xFF; BIG_INT_SIZE]);

    fn new(bytes: [u8; BIG_INT_SIZE]) -> Self {
        Self(bytes)
    }
}

impl std::fmt::Debug for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::from("0x");
        
        for byte in (**self).iter().rev() {
            string += format!("{byte:02X}").as_str();
        }

        f.debug_tuple("BigInt").field(&string).finish()
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = String::new();
        let mut value = self.to_owned();

        while value != 0.into() {
            let n = value % 10.into();
            string += format!("{}", n[0]).as_str();
            value = value / 10.into();
        }
        let string: String = if string.is_empty() {
            "0".to_string()
        } else {
            string.chars().rev().collect()
        };
        
        write!(f, "{}", string)
    }
}

impl Deref for BigInt {
    type Target = [u8; BIG_INT_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BigInt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        let mut data = [0; BIG_INT_SIZE];
        for (i, byte) in value.to_le_bytes().iter().enumerate() {
            data[i] = *byte;
        }
        BigInt(data)
    }
}

impl Not for BigInt {
    type Output = BigInt;

    fn not(self) -> Self::Output {
        let mut result = self.to_owned();
        not_bytes(&mut result.0);
        result
    }
}

impl Shr<usize> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: usize) -> Self::Output {
        let mut result = self.to_owned();
        shr_bytes(&mut *result, rhs);
        result
    }
}

impl Shl<usize> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: usize) -> Self::Output {
        let mut result = self.to_owned();
        shl_bytes(&mut *result, rhs);
        result
    }
}

impl Add for BigInt {
    type Output = BigInt;

    fn add(self, rhs: Self) -> Self::Output {
        let (result, _) = add_bytes(*self, *rhs);
        BigInt::new(result)
    }
}

impl AddAssign for BigInt {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: Self) -> Self::Output {
        let (result, _) = sub_bytes(*self, *rhs);
        BigInt::new(result)
    }
}

impl SubAssign for BigInt {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl Div for BigInt {
    type Output = BigInt;

    fn div(self, rhs: Self) -> Self::Output {
        let (result, _) = div_bytes(*self, *rhs);
        BigInt::new(result)
    }
}

impl Rem for BigInt {
    type Output = BigInt;

    fn rem(self, rhs: Self) -> Self::Output {
        let (_, result) = div_bytes(*self, *rhs);
        BigInt::new(result)
    }
}

impl Mul for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: Self) -> Self::Output {
        let (result, _) = mul_bytes(*self, *rhs);
        BigInt::new(result)
    }
}

fn not_bytes(arr: &mut [u8]) {
    for byte in arr {
        *byte = !*byte;
    }
}

fn shr_bytes(arr: &mut [u8], rhs: usize)  {
    for _ in 0..rhs {
        for i in 0..arr.len() {
            let carry = arr[i] & 0b00000001 == 0b00000001;
            arr[i] = arr[i] >> 1;
            if carry && i != 0 {
                arr[i - 1] = arr[i - 1] | 1 << 7;
            }
        }
    }
}

fn shl_bytes(arr: &mut [u8], rhs: usize)  {
    for _ in 0..rhs {
        for i in (0..arr.len()).rev() {
            let carry = arr[i] & 0b10000000 == 0b10000000;
            arr[i] = arr[i] << 1;
            if carry && i != arr.len() - 1 {
                arr[i + 1] = arr[i + 1] | 1;
            }
        }
    }
}

fn add_bytes<const S: usize>(lhs: [u8; S], rhs: [u8; S]) -> ([u8; S], bool) {
    let mut result = [0u8; S];

    let mut carry = false;
    for i in 0..S {
        for shift in 0..8 {
            let bit_a = lhs[i] >> shift & 1 == 1;
            let bit_b = rhs[i] >> shift & 1 == 1;
            
            let mut set_bit = || {
                result[i] = 1 << shift | result[i];
            };

            match (bit_a, bit_b, carry) {
                (true, true, true) => {
                    set_bit();
                    carry = true;
                },
                (true, true, false) => carry = true,
                (true, false, true) => carry = true,
                (true, false, false) => set_bit(),
                (false, true, true) => carry = true,
                (false, true, false) => set_bit(),
                (false, false, true) => {
                    set_bit();
                    carry = false;
                },
                (false, false, false) => (),
            }
        }
    }
    
    (result, carry)
}

// https://www.youtube.com/watch?v=tKvOwKC1T3w
fn sub_bytes<const S: usize>(lhs: [u8; S], rhs: [u8; S]) -> ([u8; S], bool) {
    let mut rhs = rhs.to_owned();
    not_bytes(&mut rhs);
    let mut one = [0u8; S];
    one[0] = 1;
    let (inverted, overflow_1) = add_bytes(rhs, one);
    let (result, overflow_2) = add_bytes(lhs, inverted);
    (result, !overflow_1 && !overflow_2)
}

// https://www.youtube.com/watch?v=tdg2jXEAClE paper method implemented here
fn mul_bytes<const S: usize>(lhs: [u8; S], rhs: [u8; S]) -> ([u8; S], bool) {
    let mut lhs = lhs;
    let mut result = [0u8; S];
    let mut overflow = false;

    for i in 0..S {
        for shift in 0..8 {
            let bit_b = rhs[i] >> shift & 1 == 1;
            
            if bit_b {
                let x = add_bytes(result, lhs);
                result = x.0;
                overflow = overflow || x.1;
            }

            shl_bytes(&mut lhs, 1);
        }
    }
    
    (result, overflow)
}

// https://www.youtube.com/watch?v=56by3-gAcXU
fn div_bytes<const S: usize>(lhs: [u8; S], rhs: [u8; S]) -> ([u8; S], [u8; S]) {
    let mut quocient = [0u8; S];
    let mut remainder_twice_sized = [0u8; BIG_INT_SIZE * 2];
    let mut divisor = [0u8; BIG_INT_SIZE * 2];
    
    // Twice as big shifted by the size higher
    for i in 0..S {
        divisor[i + S] = rhs[i];
        remainder_twice_sized[i] = lhs[i];
    }

    for _ in 0..S * 8 + 1 {
        let (result, overflow) = sub_bytes(remainder_twice_sized, divisor);
        if !overflow {
            remainder_twice_sized = result;
        }
        shr_bytes(&mut divisor, 1);
        shl_bytes(&mut quocient, 1);
        if !overflow {
            quocient[0] = quocient[0] | 1;
        }
    }

    let mut remainder = [0u8; S];

    for i in 0..S {
        remainder[i] = remainder_twice_sized[i];
    }

    (quocient, remainder)
}

#[derive(Debug)]
struct BitIterator<'a> {
    data: &'a mut [u8],
    index: usize
}

impl<'a> BitIterator<'a> {
    fn new(data: &'a mut [u8]) -> Self {
        Self { data, index: 0 }
    }
}

impl<'a> Iterator for BitIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let byte_index = self.index / 8;
        let Some(byte) = self.data.get(byte_index) else {
            return None;
        };

        let bit_index =  self.index % 8;
        let bit = *byte >> bit_index & 0b00000001;
        
        self.index += 1;
        Some(bit)
    }
}

fn main() {
    let mut n = BigInt::from(2);
    let power = 1000;

    println!("Computing started.");
    // Technically no need to compute since 2^1000 is 1 followed by 1000 zeroes in binary or 250 zeroes in hex
    /*
    BigInt(
        "0x0000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    )
    */
    for _ in 0..power-1 {
        n = n * BigInt::from(2);
    }
    // Printing takes the majority of time, should improve that
    dbg!(n);
    println!("Computing done, printing...");
    let ciphers = n.to_string();
    println!("{ciphers}");

    let sum = ciphers.chars().map(|c| c.to_digit(10).unwrap()).sum::<u32>();
    println!("Sum of ciphers:");
    dbg!(sum);
}