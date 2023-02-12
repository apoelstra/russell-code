
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

#![allow(clippy::suspicious_arithmetic_impl)] // this is the shittiest lint ever

pub mod base32;
pub mod checksum32;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 4 {
        println!("Usage: {} <sum|validate> <checksum> <string>", args[0]);
        return;
    }

    let s = real_main(&args[1], &args[2], &args[3]);
    println!("{s}");
}

fn real_main(action_s: &str, checksum_s: &str, s: &str) -> String {
    let checksums = checksum32::get_checksums();
    let checksum = match checksums.get(checksum_s) {
        Some(checksum) => checksum,
        None => {
            println!("Unknown checksum {}. Available checksums:", checksum_s);
            for checksum in checksums.keys() {
                println!("     {checksum}");
            }
            return "ERROR".into();
        }
    };

    match action_s {
        "sum" => checksum.checksum(s),
        "validate" => {
            if checksum.validate_checksum(s) {
                "OK".into()
            } else {
                "BAD".into()
            }
        }
        x => panic!("unknown action {x}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bech32() {
        assert_eq!(
            real_main("sum", "bech32", "bc1qar0srrr7xfkvy5l643lydnw9re59gtzz"),
            "bc1qar0srrr7xfkvy5l643lydnw9re59gtzzwf5mdq",
        );
    }

    #[test]
    fn test_codex32() {
        assert_eq!(
            real_main("sum", "codex32", "ms10testsxxxxxxxxxxxxxxxxxxxxxxxxxx"),
            "ms10testsxxxxxxxxxxxxxxxxxxxxxxxxxx4nzvca9cmczlw",
        );
        assert_eq!(
            real_main(
                "validate",
                "codex32",
                "ms10testsxxxxxxxxxxxxxxxxxxxxxxxxxx4nzvca9cmczlw"
            ),
            "OK",
        );
    }
}
