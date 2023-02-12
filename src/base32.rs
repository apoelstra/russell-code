// Bech32 Code Playground
// Written in 2021 by
//   Andrew Poelstra <apoelstra@wpsoftware.net>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the CC0 Public Domain Dedication
// along with this software.
// If not, see <http://creativecommons.org/publicdomain/zero/1.0/>.
//

use std::{fmt, ops, str};

/// Character set in lexicographic order
pub const CHARSET: &[u8; 32] = b"qpzry9x8gf2tvdw0s3jn54khce6mua7l";

/// An element of GF(32) constructed as GF(2)[x] mod x^5 + x^3 + 1
///
/// Elements are represented using the bech32 alphabet
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Default, PartialEq, Eq, Hash)]
pub struct u5(u8);

impl u5 {
    fn mul_alpha(&mut self) {
        self.0 <<= 1;
        if self.0 & 0x20 == 0x20 {
            self.0 ^= 0x29;
        }
    }

    /// Construct a u5 from a character
    pub fn from_char(c: char) -> Result<Self, String> {
        match c {
            'q' => Ok(u5(0x00)),
            'p' => Ok(u5(0x01)),
            'z' => Ok(u5(0x02)),
            'r' => Ok(u5(0x03)),
            'y' => Ok(u5(0x04)),
            '9' => Ok(u5(0x05)),
            'x' => Ok(u5(0x06)),
            '8' => Ok(u5(0x07)),
            'g' => Ok(u5(0x08)),
            'f' => Ok(u5(0x09)),
            '2' => Ok(u5(0x0a)),
            't' => Ok(u5(0x0b)),
            'v' => Ok(u5(0x0c)),
            'd' => Ok(u5(0x0d)),
            'w' => Ok(u5(0x0e)),
            '0' => Ok(u5(0x0f)),
            's' => Ok(u5(0x10)),
            '3' => Ok(u5(0x11)),
            'j' => Ok(u5(0x12)),
            'n' => Ok(u5(0x13)),
            '5' => Ok(u5(0x14)),
            '4' => Ok(u5(0x15)),
            'k' => Ok(u5(0x16)),
            'h' => Ok(u5(0x17)),
            'c' => Ok(u5(0x18)),
            'e' => Ok(u5(0x19)),
            '6' => Ok(u5(0x1a)),
            'm' => Ok(u5(0x1b)),
            'u' => Ok(u5(0x1c)),
            'a' => Ok(u5(0x1d)),
            '7' => Ok(u5(0x1e)),
            'l' => Ok(u5(0x1f)),
            x => Err(format!("invalid bech32 character {}", x)),
        }
    }
}

impl fmt::Display for u5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let idx: usize = self.0.into();
        fmt::Write::write_char(f, CHARSET[idx] as char)
    }
}

impl fmt::Debug for u5 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let idx: usize = self.0.into();
        write!(f, "{}[{:05b}]", CHARSET[idx] as char, self.0)
    }
}

impl From<u8> for u5 {
    fn from(u: u8) -> u5 {
        if u < 32 {
            u5(u)
        } else {
            panic!("Tried to construct u5 from too-large number {}", u);
        }
    }
}

impl ops::Add<u5> for u5 {
    type Output = u5;
    fn add(self, other: u5) -> u5 {
        u5(self.0 ^ other.0)
    }
}
impl ops::AddAssign<u5> for u5 {
    fn add_assign(&mut self, other: u5) {
        *self = *self + other;
    }
}

impl ops::Mul for u5 {
    type Output = u5;
    fn mul(self, mut other: u5) -> u5 {
        let mut res: u5 = u5(0);
        if self.0 & 0x01 != 0 {
            res += other;
        }
        other.mul_alpha();
        if self.0 & 0x02 != 0 {
            res += other;
        }
        other.mul_alpha();
        if self.0 & 0x04 != 0 {
            res += other;
        }
        other.mul_alpha();
        if self.0 & 0x08 != 0 {
            res += other;
        }
        other.mul_alpha();
        if self.0 & 0x10 != 0 {
            res += other;
        }
        res
    }
}
impl ops::MulAssign<u5> for u5 {
    fn mul_assign(&mut self, other: u5) {
        *self = *self * other;
    }
}

/// A GF(32) "bech32" string
#[allow(non_camel_case_types)]
#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct u5String(Vec<u5>);

impl u5String {
    /// Construct a u5 string from a bech32 string (which will have a HRP)
    pub fn from_hrpstring(s: &str) -> Result<Self, String> {
        // Split out the HRP
        let (hrp, real_string) = match s.rsplit_once('1') {
            Some((s1, s2)) => (s1, s2),
            None => ("", s),
        };

        // Expand the HRP into base 32
        let mut res = Vec::with_capacity(hrp.len() * 2 + 1 + real_string.len());
        for ch in hrp.bytes() {
            res.push(u5(ch >> 5));
        }
        res.push(u5(0));
        for ch in hrp.bytes() {
            res.push(u5(ch & 0x1f));
        }

        // Append the actual string
        res.extend(<u5String as str::FromStr>::from_str(real_string)?.0);
        Ok(u5String(res))
    }

    /// Pushes a u5 character onto the end of a string
    pub fn push(&mut self, x: u5) {
        self.0.push(x)
    }

    /// Return the length of the string, in u5 characters
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Return whether or not this string is the null string
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Return whether or not this string contains only 0 characters
    pub fn is_all_zero(&self) -> bool {
        self.0.iter().all(|ch| *ch == u5(0))
    }
}

impl From<Vec<u5>> for u5String {
    fn from(v: Vec<u5>) -> u5String {
        u5String(v)
    }
}

impl From<Vec<u8>> for u5String {
    fn from(v: Vec<u8>) -> u5String {
        u5String(v.into_iter().map(From::from).collect())
    }
}

impl fmt::Display for u5String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &ch in &self.0 {
            let idx: usize = ch.0.into();
            fmt::Write::write_char(f, CHARSET[idx] as char)?;
        }
        Ok(())
    }
}

impl fmt::Debug for u5String {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("u5(")?;
        for &ch in &self.0 {
            let idx: usize = ch.0.into();
            fmt::Write::write_char(f, CHARSET[idx] as char)?;
        }
        f.write_str("u5(")
    }
}

impl str::FromStr for u5String {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, String> {
        let mut ret = Vec::with_capacity((s.len() * 8 + 4) / 5);
        for ch in s.chars() {
            ret.push(u5::from_char(ch)?);
        }
        Ok(u5String(ret))
    }
}

impl<I> ops::Index<I> for u5String
where
    Vec<u5>: ops::Index<I>,
{
    type Output = <Vec<u5> as ops::Index<I>>::Output;
    fn index(&self, idx: I) -> &Self::Output {
        &self.0[idx]
    }
}

impl<I> ops::IndexMut<I> for u5String
where
    Vec<u5>: ops::IndexMut<I>,
{
    fn index_mut(&mut self, idx: I) -> &mut Self::Output {
        &mut self.0[idx]
    }
}
