
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
        "to_hrp_u5" => {
            let input = match base32::u5String::from_hrpstring(s) {
                Ok(inp) => inp,
                Err(e) => panic!("Could not parse input {s} as HRP string: {e}"),
            };
            let mut ret = String::with_capacity(input.len() * 4);
            // lol i'll optimize this later
            ret.push_str("[");
            for b in &input[..] {
                ret.push_str(&format!("0x{:02x}, ", u8::from(*b)));
            }
            ret.push_str("]");
            ret
        },
        "to_hrp_hex" => {
            let input = match base32::u5String::from_hrpstring(s) {
                Ok(inp) => inp,
                Err(e) => panic!("Could not parse input {s} as HRP string: {e}"),
            };
            let mut ret = String::with_capacity(input.len() * 2);
            // lol i'll optimize this later
            for b in input.to_bytes() {
                ret.push_str(&format!("{b:02x}"));
            }
            ret
        },
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
        // grabbed from sipa's demo site
        assert_eq!(
            real_main("validate", "bech32", "bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4"),
            "OK",
        );
    }

    fn codex32_valid(s: &str) {
        assert_eq!(real_main("validate", "codex32", s), "OK");
    }

    #[test]
    fn test_codex32() {
        // Vector 1 from BIP draft 2023-02
        assert_eq!(
            real_main("sum", "codex32", "ms10testsxxxxxxxxxxxxxxxxxxxxxxxxxx"),
            "ms10testsxxxxxxxxxxxxxxxxxxxxxxxxxx4nzvca9cmczlw",
        );
        codex32_valid("ms10testsxxxxxxxxxxxxxxxxxxxxxxxxxx4nzvca9cmczlw");
        // Vector 2 from BIP draft 2023-02
        codex32_valid("MS12NAMEA320ZYXWVUTSRQPNMLKJHGFEDCAXRPP870HKKQRM");
        codex32_valid("MS12NAMECACDEFGHJKLMNPQRSTUVWXYZ023FTR2GDZMPY6PN");
        codex32_valid("MS12NAMEDLL4F8JLH4E5VDVULDLFXU2JHDNLSM97XVENRXEG");
        codex32_valid("MS12NAMES6XQGUZTTXKEQNJSJZV4JV3NZ5K3KWGSPHUH6EVW");
        // Vector 3 from BIP draft 2023-02
        codex32_valid("ms13cashsllhdmn9m42vcsamx24zrxgs3qqjzqud4m0d6nln");
        codex32_valid("ms13casha320zyxwvutsrqpnmlkjhgfedca2a8d0zehn8a0t");
        codex32_valid("ms13cashcacdefghjklmnpqrstuvwxyz023949xq35my48dr");
        codex32_valid("ms13cashd0wsedstcdcts64cd7wvy4m90lm28w4ffupqs7rm");
        codex32_valid("ms13casheekgpemxzshcrmqhaydlp6yhms3ws7320xyxsar9");
        codex32_valid("ms13cashf8jh6sdrkpyrsp5ut94pj8ktehhw2hfvyrj48704");

        codex32_valid("ms13cashsllhdmn9m42vcsamx24zrxgs3qqjzqud4m0d6nln"); // repeat of above
        codex32_valid("ms13cashsllhdmn9m42vcsamx24zrxgs3qpte35dvzkjpt0r");
        codex32_valid("ms13cashsllhdmn9m42vcsamx24zrxgs3qzfatvdwq5692k6");
        codex32_valid("ms13cashsllhdmn9m42vcsamx24zrxgs3qrsx6ydhed97jx2");
        // Vector 4 from BIP draft 2023-02
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqqtum9pgv99ycma");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqpj82dp34u6lqtd");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqzsrs4pnh7jmpj5");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqrfcpap2w8dqezy");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqy5tdvphn6znrf0");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyq9dsuypw2ragmel");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqx05xupvgp4v6qx");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyq8k0h5p43c2hzsk");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqgum7hplmjtr8ks");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqf9q0lpxzt5clxq");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyq28y48pyqfuu7le");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqt7ly0paesr8x0f");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqvrvg7pqydv5uyz");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqd6hekpea5n0y5j");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyqwcnrwpmlkmt9dt");
        codex32_valid("ms10leetsllhdmn9m42vcsamx24zrxgs3qrl7ahwvhw4fnzrhve25gvezzyq0pgjxpzx0ysaam");
    }

    #[test]
    fn test_to_hrp_hex() {
        assert_eq!(real_main("to_hrp_hex", "bech32", "SECRETSHARE32"), "043381e570bf4798");
        assert_eq!(
            real_main("to_hrp_hex", "bech32", "ms1"),
            "18c0d9",
        );
        // Test vector 1 has the hex 318c6318c6318c6318c6318c6318c631 which you can
        // confirm is a substring of this output, except for the final 1 which becomes
        // 5, presumably because part of that byte becomes checksum data.
        assert_eq!(
            real_main("to_hrp_hex", "bech32", "ms10testsxxxxxxxxxxxxxxxxxxxxxxxxxx4nzvca9cmczlw"),
            "18c0d9bd7982e06318c6318c6318c6318c6318c6318c635662663a5c6f02fb",
        );
        assert_eq!(
            real_main("to_hrp_u5", "bech32", "ms10testsxxxxxxxxxxxxxxxxxxxxxxxxxx4nzvca9cmczlw"),
            "[0x03, 0x03, 0x00, 0x0d, 0x13, 0x0f, 0x0b, 0x19, 0x10, 0x0b, 0x10, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x15, 0x13, 0x02, 0x0c, 0x18, 0x1d, 0x05, 0x18, 0x1b, 0x18, 0x02, 0x1f, 0x0e, ]",
        );
    }
}
