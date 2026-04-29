// (c) 2026 : https://github.com/NMoroney + Gemini CLI
// MIT License
// https://github.com/NMoroney/Struse
// https://nmoroney.github.io/notes/2026/260423_struse/index.html
//

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_random_seed() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        // Simple LCG parameters
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.state
    }

    fn gen_range(&mut self, min: u32, max: u32) -> u32 {
        let range = max - min + 1;
        (self.next() % range as u64) as u32 + min
    }
}

pub fn to_struse(s1: &str) -> String {
    let mut rng = Lcg::new(get_random_seed());
    
    let m1: HashMap<char, &str> = [
        ('a', "&#x10300;"), ('b', "&#x1041A;"), ('c', "&#x1103;"), ('d', "&#x1095;"),
        ('e', "&#x0F5B;"), ('f', "&#x0CEF;"), ('g', "&#x0D6D;"), ('h', "&#x0A2E;"),
        ('i', "&#x1109;"), ('j', "&#x10408;"), ('k', "&#x051E;"), ('l', "&#x10539;"),
        ('m', "&#x1328;"), ('n', "&#x0E17;"), ('o', "&#x07DB;"), ('p', "&#x10502;"),
        ('q', "&#x09ED;"), ('r', "&#x053B;"), ('s', "&#x0D17;"), ('t', "&#x10B5;"),
        ('u', "&#x053F;"), ('v', "&#x13C9;"), ('w', "&#x0E1F;"), ('x', "&#x0514;"),
        ('y', "&#x080A;"), ('z', "&#x10350;"),
    ].iter().cloned().collect();

    let m2: HashMap<char, &str> = [
        ('a', "&#x1D70;"), ('b', "&#x266D;"), ('c', "&#x122D;"), ('d', "&#x1D1CE;"),
        ('e', "&#x096C;"), ('f', "&#x10346;"), ('g', "&#x1B9D;"), ('h', "&#x10AC;"),
        ('i', "&#x10382;"), ('j', "&#x1D4A5;"), ('k', "&#x20AD;"), ('l', "&#x1D4DB;"),
        ('m', "&#x0BF1;"), ('n', "&#x0E81;"), ('o', "&#x0C20;"), ('p', "&#xA6E0;"),
        ('q', "&#x1007;"), ('r', "&#x10342;"), ('s', "&#x13A6;"), ('t', "&#x1D366;"),
        ('u', "&#x07CE;"), ('v', "&#x2A54;"), ('w', "&#x0E9C;"), ('x', "&#x0CF1;"),
        ('y', "&#x10284;"), ('z', "&#x2F04;"),
    ].iter().cloned().collect();

    let m3: HashMap<char, &str> = [
        ('a', "&#x10001;"), ('b', "&#x2CD2;"), ('c', "&#x1573;"), ('d', "&#x1D507;"),
        ('e', "&#x0B3D;"), ('f', "&#x2A0D;"), ('g', "&#x01E4;"), ('h', "&#x12E0;"),
        ('i', "&#x1703;"), ('j', "&#x3057;"), ('k', "&#xA018;"), ('l', "&#x2827;"),
        ('m', "&#x0D28;"), ('n', "&#x12AC;"), ('o', "&#x0DA7;"), ('p', "&#xAA6D;"),
        ('q', "&#x10E2;"), ('r', "&#x053B;"), ('s', "&#x1511;"), ('t', "&#x2020;"),
        ('u', "&#x00B5;"), ('v', "&#xAA9C;"), ('w', "&#x0429;"), ('x', "&#x102C4;"),
        ('y', "&#x03AB;"), ('z', "&#xA903;"),
    ].iter().cloned().collect();

    let mut s2 = String::new();
    for c in s1.chars() {
        let c1 = c.to_ascii_lowercase();
        if m1.contains_key(&c1) {
            let rn = rng.gen_range(1, 3);
            let val = match rn {
                1 => m1.get(&c1).unwrap(),
                2 => m2.get(&c1).unwrap(),
                _ => m3.get(&c1).unwrap(),
            };
            s2.push_str(val);
        } else {
            if c == ' ' {
                s2.push_str("&ensp; ");
            } else {
                s2.push(c1);
            }
        }
    }
    s2
}

fn main() {
    let s1 = "rhymes with juice";
    for _ in 0..4 {
        let t1 = to_struse(s1);
        println!("{}", t1);
    }
}
