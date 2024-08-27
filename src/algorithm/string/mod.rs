pub mod single_pattern_match;
pub mod multiple_pattern_match;

#[cfg(test)]
mod test_single_pattern_match {
    use crate::algorithm::string::single_pattern_match::*;
    fn gen_text_and_patten() -> (String, String, Vec<usize>) {
        use rand::{Rng, thread_rng};
        let gen_character = || {
            let offset = thread_rng().gen::<u8>();
            let r = thread_rng().gen::<u8>() % 3 ;
            if r % 3 == 0 {
                'a' as u8 + (offset % 26) 
            } else if r % 3 == 1 {
                'A' as u8 + (offset % 26)
            } else {
                '0' as u8 + (offset % 10)
            }
        };
        const BASE: usize = 1000;
        let mut patten = Vec::<u8>::new();
        let mut text = Vec::<u8>::new();
        let mut index = Vec::new();
        let patten_len = thread_rng().gen_range(BASE..BASE * 10);
        for _ in 0..patten_len {
            patten.push(gen_character());
        } 
        let round = 3 + thread_rng().gen::<usize>() % 10;
        for _ in 0..round {
            if thread_rng().gen::<usize>() % round == 0 {
                for u in patten.iter() {
                    text.push(*u);
                } 
            } else {
                for _ in 0..thread_rng().gen_range(BASE * 100..BASE * 1000) {
                    text.push(gen_character());
                }
            }
        }
        let text = String::from_utf8(text).unwrap();
        let patten = String::from_utf8(patten).unwrap();
        let mut start = 0;
        while let Some(i) = text[start..].find(&patten) {
            index.push(start + i); 
            start += i + 1;
        }
        (text, patten, index)
    }
    #[test]
    fn test_brute_force() {
        let (text, pattern, index) = gen_text_and_patten();
        let result = brute_force(&text, &pattern);
        assert_eq!(result, index);
    }
    #[test]
    fn test_rk() {

    }
    #[test]
    fn test_kmp() {
        let (text, pattern, index) = gen_text_and_patten();
        let result = kmp(&text, &pattern);
        assert_eq!(result, index);
    }
}

#[cfg(test)]
mod test_multiple_pattern_match{

}

