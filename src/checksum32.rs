// Bech32 Code Playground
// Written in 2023 by
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

/// Checksums
///
/// This file defines the various checksums that we care about for the codex32
/// project. This includes bech32 and bech32m (mostly for sanity checking),
/// codex32 itself, and "long codex32" which is the checksum used for strings
/// longer than 93 characters.
///
/// To define a new code, the process is roughly:
///     1. Run Pieter's gen_bech.py code in his pychar repo. You may want to
///        edit the final line that sets some default parameters.
///     2. This will output a checksum with something like
///        `gen=[23,4,22,5,6,21,23,6,21,25,9,26,25,10,15,1]`
///     3. Obtain its "string" representation by copying any of the `get_mod_*`
///        unit tests below and replacing the `genbch_str` variable.
///     4. This is your `MODULUS_STRING`. For the `RESIDUE_STRING` just make something up.
///     5. Modify the 'get_checksums()' function to add your new checksum.
///
/// To use the checksum with the codex32 PostScript code, replace the polymodulus
/// variable with the `gen=` string, replacing the commas with spaces and IMPORTANTLY
/// dropping the final 1, which is implicit in the Python code.
///
use crate::base32::{u5, u5String};
use std::{collections::HashMap, str::FromStr};

/// Returns the master list of checksums supported by this tool
pub fn get_checksums() -> HashMap<&'static str, Checksum> {
    vec![
        ("bech32", Checksum::new("ja45kap", "qqqqqp")),
        ("codex32", Checksum::new("sscmleeeqg3mep", "secretshare32")),
        (
            "long-codex32",
            Checksum::new("hyk9x4hx4ef6e20p", "secretshare32ex"),
        ),
    ]
    .into_iter()
    .collect()
}

/// Generic checksum trait
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Checksum {
    /// Stringified version of the modulus
    modulus: u5String,
    /// Stringified version of the
    residue: u5String,
}

impl Checksum {
    /// Construct a new checksum. This should only be called from the `get_checksums` function
    /// in this file.
    fn new(modulus_str: &str, residue_str: &str) -> Checksum {
        assert!(
            modulus_str.is_ascii(),
            "Modulus string \"{modulus_str}\" must be ASCII"
        );
        assert!(
            residue_str.is_ascii(),
            "Residue string \"{residue_str}\" must be ASCII"
        );
        assert_eq!(
            modulus_str.as_bytes()[modulus_str.len() - 1],
            b'p',
            "Modulus string \"{modulus_str}\" should end in 'p'.",
        );
        assert_eq!(modulus_str.len(), residue_str.len() + 1,);

        let modulus = match u5String::from_str(modulus_str) {
            Ok(s) => s,
            Err(e) => panic!("Modulus string \"{modulus_str}\" was not a u5 string: {e}"),
        };
        let residue = match u5String::from_str(residue_str) {
            Ok(s) => s,
            Err(e) => panic!("Residue string \"{residue_str}\" was not a u5 string: {e}"),
        };
        Checksum { modulus, residue }
    }

    /// Compute the residue of a string, plus the target residue
    fn polymod(&self, input: &u5String) -> u5String {
        /// Helper function to multiply the current remainder by x
        fn shift(checksum: &Checksum, result: &mut [u5]) {
            // Store current coefficient of x^{n-1}, which will become
            // x^n (and get reduced)
            let xn = result[0];
            // Simply shift x^0 through x^{n-1} up one, and set x^0 to 0
            for i in 1..result.len() {
                result[i - 1] = result[i];
            }
            result[result.len() - 1] = u5::from(0);
            // Then reduce x^n mod the generator. We need to read the generator
            // backward for endianness reasons (well, because the generator is
            // a polynomial stored with the ith coefficient in position i, while
            // our target string sa the ith coefficient in position (n-i). Also
            // we need to skip the final 1 coefficient, which is implicit in
            // our algorithm.
            let mod_iter = checksum.modulus[..checksum.modulus.len() - 1].iter().rev();
            for (i, ch) in mod_iter.enumerate() {
                result[i] += *ch * xn;
            }
        }

        // 3. Loop through the string, interpreting it as a polynomial in
        // GF(32). Continually mod it out by the checksum generator
        // Here {n} represents the GF(32) element whose binary encoding
        // is the same as that for the 5-bit big-endian integer n.
        let mut ret = vec![u5::from(0); self.residue.len()];
        let residue_len = ret.len();
        ret[residue_len - 1] = u5::from(1); // start with the polynomial 1
        for ch in &input[..] {
            shift(self, &mut ret[..]);
            ret[residue_len - 1] += *ch;
        }
        // 4. Add the residue to it
        for (i, ch) in self.residue[..].iter().enumerate() {
            ret[i] += *ch;
        }
        // 5. Return
        u5String::from(ret)
    }

    /// Compute the checksum of a string (with HRP) and tack it onto the end
    pub fn checksum(&self, s: &str) -> String {
        // 1. Parse the string from ASCII into u5
        let mut input = match u5String::from_hrpstring(s) {
            Ok(s) => s,
            Err(e) => panic!("String to checksum \"{s}\" was not a u5 string: {e}"),
        };
        // 2. Suffix some 0s onto the end, which we will replace by the checksum
        let pre_checksum_len = input.len();
        println!(
            "len {pre_checksum_len}   residue len {}",
            self.residue.len()
        );
        for _ in 0..self.residue.len() {
            input.push(u5::from(0));
        }
        // 3. Compute its checksum
        let checksum = self.polymod(&input);
        // 4. Tack it onto the end
        for i in 0..checksum.len() {
            input[pre_checksum_len + i] = checksum[i];
        }
        // 5. Tack it onto the original string and return
        let mut ret = String::with_capacity(s.len() + checksum.len());
        ret.push_str(s);
        ret.push_str(&checksum.to_string());
        ret
    }

    /// Check whether an already-checksummed string is valid
    pub fn validate_checksum(&self, s: &str) -> bool {
        // 1. Parse the string from ASCII into u5
        let input = match u5String::from_hrpstring(s) {
            Ok(s) => s,
            Err(e) => panic!("String to checksum \"{s}\" was not a u5 string: {e}"),
        };
        // 2. Compute its checksum and confirm the residue is 0
        self.polymod(&input).is_all_zero()
    }
}

/*
/// The codex32 checksum
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Codex32;
impl Checksum for Codex32 {
}

/// The bech32 checksum
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Bech32;
impl Checksum for Bech32 {
    const MODULUS_STRING: &'static str = "ja45kap";
    const RESIDUE_STRING: &'static str = "qqqqqp";
}
*/

#[cfg(test)]
mod tests {
    use crate::base32;

    #[test]
    fn get_mod_string_long_codex32() {
        let genbch_str = vec![23, 4, 22, 5, 6, 21, 23, 6, 21, 25, 9, 26, 25, 10, 15, 1];
        assert_eq!(
            base32::u5String::from(genbch_str).to_string(),
            "hyk9x4hx4ef6e20p",
        );
    }

    #[test]
    fn get_mod_string_codex32() {
        let genbch_str = vec![16, 16, 24, 27, 31, 25, 25, 25, 0, 8, 17, 27, 25, 1];
        assert_eq!(
            base32::u5String::from(genbch_str).to_string(),
            "sscmleeeqg3mep",
        );
    }

    #[test]
    fn get_mod_string_bech32() {
        let genbch_str = vec![18, 29, 21, 20, 22, 29, 1];
        assert_eq!(base32::u5String::from(genbch_str).to_string(), "ja45kap",);
    }
}
