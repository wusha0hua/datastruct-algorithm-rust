
pub fn brute_force(text: &str, patten: &str) -> Vec<usize> {
    let mut result = Vec::new();
    if patten.len() > text.len() {return result;}
    let text = text.as_bytes();
    let patten = patten.as_bytes();
    for i in 0..text.len() - patten.len() + 1 {
        let mut j = 0;
        while j < patten.len() {
            if patten[j] != text[i + j] {
                break;
            } else {
                j += 1;
            }
        }
        if j == patten.len() {
            result.push(i);
        }
    } 
    result
}

pub fn rk(text: &str, patten: &str) -> Vec<usize> {
    let mut result = Vec::new();
    const MERSENNE_PRIME: u64 = 2147483647;
    const ASCII_COUNTS: u64 = 256;
    let quick_power_modular = |base: u64, exp: u64, modulu: u64| -> u64 {
        let mut result = 1;
        let mut base = base % modulu;
        let mut exp = exp % modulu;
        while exp > 0 {
            if exp & 1 == 1 {
                result = result * base % modulu;
            }
            base = base * base % modulu;
            exp >>= 1;
        }
        result % modulu
    };
    let text = text.as_bytes();
    let patten = patten.as_bytes();
    let patten_len = patten.len();
    let text_len = text.len();
    let mut patten_hash: u64 = 0;
    for (index, value) in patten.iter().enumerate() {
        patten_hash = (*value as u64 * ((patten_hash + quick_power_modular(ASCII_COUNTS, (patten_len - index) as u64 - 1, MERSENNE_PRIME)) % MERSENNE_PRIME)) % MERSENNE_PRIME; 
    }
    let mut text_hash: u64 = 0;
    for i in 0..patten_len {
        text_hash = (text[i] as u64 * ((text_hash + quick_power_modular(ASCII_COUNTS, (patten_len - i) as u64 - 1, MERSENNE_PRIME)) % MERSENNE_PRIME)) % MERSENNE_PRIME;
    }
    if text_hash == patten_hash && patten == &text[0..patten_len] {
        result.push(0);
    }
    for i in 1..text_len - patten_len + 1 {
        text_hash = text_hash + MERSENNE_PRIME;
        text_hash = text_hash - (quick_power_modular(ASCII_COUNTS, patten_len as u64 - 1, MERSENNE_PRIME) * text[i - 1] as u64 % MERSENNE_PRIME);
        text_hash = ((text_hash % MERSENNE_PRIME) * ASCII_COUNTS) % MERSENNE_PRIME;
        text_hash = (text_hash + text[i + patten_len - 1] as u64) % MERSENNE_PRIME;
        if text_hash == patten_hash && patten == &text[i..i + patten_len] {
            result.push(i);
        }
    } 
    result
}

pub fn kmp(text: &str, patten: &str) -> Vec<usize> {
    let mut result = Vec::new();
    if patten.len() > text.len() {return result;}
    let text = text.as_bytes();
    let patten = patten.as_bytes();
    let get_next = |s: &[u8]| -> Vec<usize> {
        let mut next = vec![0; s.len()];
        let mut left = 0;
        for right in 1..s.len() {
            while left > 0 && s[left] != s[right] {
                left = next[left - 1];
            }
            if s[left] == s[right] {
                left += 1;
            }
            next[right] = left;
        }
        next
    };
    let mut j = 0;
    let next = get_next(patten);
    for i in 0..text.len() {
        while j > 0 && text[i] != patten[j] {
            j = next[j - 1];
        }
        if text[i] == patten[j] {
            j += 1;
        }
        if j == patten.len() {
            result.push(i + 1 - j);
            j = 0;
        }
    }
    result
}
