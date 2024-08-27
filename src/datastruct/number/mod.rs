


#[derive(Debug, PartialEq)]
pub enum NumberError {
    InvalidCharacter(char),    
    InvalidLength,
    InvalidBits,
}

#[derive(Debug, Clone)]
pub struct Number {
    bin: String,
    bits: usize,
}

impl Number {
    pub fn from_dec(number: &str, bits: usize) -> Result<Self, NumberError> {
        if bits == 0 {
            return Err(NumberError::InvalidBits);
        }
        if number.len() == 0 {
            return Err(NumberError::InvalidLength);
        }
        Ok(Self {
            bin: Self::dec_to_bin(number, bits)?,
            bits,
        })
    }
    pub fn from_bin(number: &str, bits: usize) -> Result<Self, NumberError> {
        if bits == 0 {
            return Err(NumberError::InvalidBits);
        }
        if number.len() == 0 {
            return Err(NumberError::InvalidLength);
        }
        let bin = if number.len() < bits {
            format!("{}{}", "0".repeat(bits - number.len()), number)
        } else {
            number[bits - number.len()..].to_owned()
        };
        for &n in number.as_bytes() {
            if n != '0' as u8 && n != '1' as u8 {
                return Err(NumberError::InvalidCharacter(n as char));
            } 
        }
        Ok(Self { 
            bin, 
            bits, 
        })
    }  
    pub fn dec(&self) -> String {
        let mut dec = String::from("0");
        let mut bin = self.bin.clone();
        let sign = if self.bin.as_bytes()[0] == '1' as u8 {
            bin = Self::to_negtive(&bin, self.bits).unwrap();
            true
        } else {
            false
        };
        if bin != "0".repeat(self.bits) {
            let mut adder = String::from("1");
            let bin = bin.as_bytes();
            for &b in bin.iter().rev() {
                if b == '1' as u8 {
                    dec = Self::udec_add(&dec, &adder);
                }
                adder = Self::dec_mul_two(&adder);
            }
        } 
        if sign {
            dec.insert(0, '-');
        }
        dec
    }
    pub fn extend(&mut self, bits: usize) {
        let mut bin = String::new();
        std::mem::swap(&mut self.bin, &mut bin);
        bin = Self::extend_bin(&bin, bits);
        self.bin = bin;
        self.bits = usize::max(self.bits, bits);
    }
    pub fn test(&self) -> bool {
        if let Some(&n) = self.bin.as_bytes().last() {
            n == '1' as u8 
        } else {
            false
        }
    }
    fn dec_to_bin(dec: &str, bits: usize) -> Result<String, NumberError> {
        let (mut dividen, negtive) = if dec.as_bytes()[0] == '-' as u8 {
            (dec[1..].to_owned(), true)
        } else {
            (dec.to_owned(), false)
        };
        let mut answer = String::new();
        loop {
            let (div, rem) = Self::dec_divide_by_two(&dividen)?;
            dividen = div;
            if rem {
                answer = format!("{}{}", "1", answer);
            } else {
                answer = format!("{}{}", "0", answer);
            }
            if dividen == "0" || answer.len() == bits {
                let answer = format!("{}{}", "0".repeat(bits - answer.len()), answer); 
                if negtive {
                    return Ok(Self::to_negtive(&answer, bits)?);
                } else {
                    return Ok(answer);
                }
            }
        }
    }
    fn dec_divide_by_two(dec: &str) -> Result<(String, bool), NumberError> {
        if dec.len() == 0 {
            return Err(NumberError::InvalidLength);
        }
        let mut answer = String::new();
        let dec = dec.as_bytes();
        let mut index = 0;
        if !Self::is_valid_number(dec[index]) {
            return Err(NumberError::InvalidCharacter(dec[index] as char));
        }
        let mut val = dec[index] - '0' as u8;
        if index + 1 < dec.len() && val < 2 {
            index += 1;
            if !Self::is_valid_number(dec[index]) {
                return Err(NumberError::InvalidCharacter(dec[index] as char));
            }
            val = val * 10 + dec[index] - '0' as u8;
        }
        while index < dec.len() {
            answer += &(val / 2).to_string(); 
            index += 1;
            if index < dec.len() {
                if !Self::is_valid_number(dec[index]) {
                    return Err(NumberError::InvalidCharacter(dec[index] as char));
                }
                val = (val % 2) * 10 + dec[index] - '0' as u8;
            } else {
                val = val % 2;
            }
        }
        if answer.len() == 0 {
            Ok((String::from("0"), false))
        } else {
            Ok((answer, val == 1))
        }
    }
    fn to_negtive(bin: &str, bits: usize) -> Result<String, NumberError> {
        let flip = Self::flip(bin)?;
        let mut answer = Self::add_one(&flip)?;
        if answer.len() > bits {
            answer.remove(0);
        }
        Ok(answer)
    }
    fn flip(bin: &str) -> Result<String, NumberError> {
        let mut answer = String::new();
        for &n in bin.as_bytes() {
            if n == '1' as u8 {
                answer.push('0' as char);
            } else if n == '0' as u8 {
                answer.push('1' as char);
            } else {
                return Err(NumberError::InvalidCharacter(n as char));
            }
        }
        Ok(answer)
    }
    fn add_one(bin: &str) -> Result<String, NumberError> {
        let mut answer = Vec::new();
        let bin = bin.as_bytes();
        let mut carry = 1;
        for i in (0..bin.len()).rev() {
            let bit = if bin[i] == '0' as u8 {
                0
            } else if bin[i] == '1' as u8 {
                1 
            } else {
                return Err(NumberError::InvalidCharacter(bin[i] as char));
            };
            carry += bit;
            answer.push(carry % 2 + '0' as u8);
            carry /= 2;
        }
        answer.reverse();
        match String::from_utf8(answer) {
            Ok(answer) => Ok(answer),
            Err(_) => Err(NumberError::InvalidCharacter(0 as char)),
        }
    }
    fn to_count(&self) -> Vec<usize> {
        let mut count = Vec::new();
        for index in (0..self.bits).step_by(usize::BITS as usize) {
            let start = self.bits - index - 1;
            let mut val = 0;
            let bytes = self.bin.as_bytes();
            for j in 0..usize::BITS as usize {
                if j > start {
                    break;
                }
                if bytes[start - j] == '1' as u8 {
                    val += 1 << j;
                } 
            }
            count.push(val);
        }
        while let Some(val) = count.pop() {
            if val != 0 {
                count.push(val);
                break;
            }
        }
        count
    }
    fn is_valid_number(n: u8) -> bool {
        n >= '0' as u8 && n - '0' as u8 <= 9
    }
    fn zero(bits: usize) -> Self {
        Self { 
            bin: "0".repeat(bits).to_string(), 
            bits, 
        }
    }
    fn one(bits: usize) -> Self {
        Self { 
            bin: format!("{}{}", "0".repeat(bits - 1), "1"), 
            bits,
        }
    }
    fn extend_bin(bin: &str, bits: usize) -> String {
        if bin.len() >= bits {
            bin.to_string()
        } else {
            let sign = String::from_utf8(vec![bin.as_bytes()[0]]).unwrap();
            format!("{}{}", sign.as_str().repeat(bits - bin.len()), bin)
        }
    }
    fn shl(mut bin: String, bits: usize, count: &mut Vec<usize>, index: usize) -> String {
        if index == 0 {
            if count[0] != 0 {
                bin += &"0".repeat(count[0]);
                count[0] = 0;
            } else {
                bin += &"0".repeat(usize::MAX);
            }
        } else {
            if count[index] != 0 {
                for _ in 0..count[index] {
                    return Self::shl(bin, bits, count, index - 1);
                }
                count[index] = 0;
            } else {
                for _ in 0..usize::MAX {
                    return Self::shl(bin, bits, count, index - 1)
                }
            }
        }
        bin.split_off(bin.len() - bits)
    }
    fn sshr(mut bin: String, bits: usize, count: &mut Vec<usize>, index: usize) -> String {
        let sign = if bin.as_bytes()[0] == '1' as u8 {
            "1"
        } else {
            "0"
        };
        if index == 0 {
            if count[0] != 0 {
                bin = sign.repeat(count[0]) + &bin;
                count[0] = 0;
            } else {
                bin = sign.repeat(usize::MAX) + &bin;
            }
        } else {
            if count[index] != 0 {
                for _ in 0..count[index] {
                    return Self::sshr(bin, bits, count, index - 1);
                }
                count[index] = 0;
            } else {
                for _ in 0..usize::MAX {
                    return Self::sshr(bin, bits, count, index - 1)
                }
            }
        }
        bin.truncate(bits);
        bin
    }
    fn ushr(mut bin: String, bits: usize, count: &mut Vec<usize>, index: usize) -> String {
        if index == 0 {
            if count[0] != 0 {
                bin = "0".repeat(count[0]) + &bin;
                count[0] = 0;
            } else {
                bin = "0".repeat(usize::MAX) + &bin;
            }
        } else {
            if count[index] != 0 {
                for _ in 0..count[index] {
                    return Self::sshr(bin, bits, count, index - 1);
                }
                count[index] = 0;
            } else {
                for _ in 0..usize::MAX {
                    return Self::sshr(bin, bits, count, index - 1)
                }
            }
        }
        bin.truncate(bits);
        bin
    }
    fn udec_add(dec1: &str, dec2: &str) -> String {
        let mut dec = String::new();
        let (longer, shorter) = if dec1.len() > dec2.len() {
            (dec1.as_bytes(), dec2.as_bytes())
        } else {
            (dec2.as_bytes(), dec1.as_bytes())
        };
        let mut carry = 0;
        for i in (0..shorter.len()).rev() {
            let val = longer[longer.len() - shorter.len() + i] + shorter[i] + carry - '0' as u8 * 2;
            dec.insert(0, ((val % 10) + '0' as u8) as char);
            carry = val / 10;
        }
        for i in (0..longer.len() - shorter.len()).rev() {
            let val = longer[i] + carry - '0' as u8;
            dec.insert(0, ((val % 10) + '0' as u8) as char);
        }
        if carry != 0 {
            dec.insert(0, (carry + '0' as u8) as char);
        }
        dec
    }
    fn dec_mul_two(dec: &str) -> String {
        let mut answer = String::new();
        let dec = dec.as_bytes();
        let mut carry = 0;
        for &n in dec.iter().rev() {
            let val = (n - '0' as u8) * 2 + carry;
            answer.insert(0, ((val % 10) + '0' as u8) as char);
            carry = val / 10;
        }
        while carry != 0 {
            answer.insert(0, ((carry % 10) + '0' as u8) as char);
            carry /= 10;
        }
        answer
    }
    fn ushr_number(&mut self) {
        let mut bin = String::new();
        std::mem::swap(&mut bin, &mut self.bin);
        self.bin = Self::ushr(bin, self.bits, &mut vec![1], 0);
    }
}

impl PartialEq for Number {
    fn eq(&self, other: &Self) -> bool {
        let (longer, shorter) = if self.bits > other.bits {
            (&self.bin, &other.bin)
        } else {
            (&other.bin, &self.bin)
        };
        let shorter_extend = Number::extend_bin(shorter, longer.len());
        &shorter_extend == longer
    }
}


impl std::ops::BitAnd for Number {
    type Output = Number;
    fn bitand(self, rhs: Self) -> Self::Output {
        let bits = usize::max(self.bits, rhs.bits);
        let (longer, shorter) = if self.bits > rhs.bits {
            (self.bin.as_bytes(), rhs.bin.as_bytes())
        } else {
            (rhs.bin.as_bytes(), self.bin.as_bytes())
        };
        let mut bin = Vec::from_iter(longer.iter().map(|x| *x - '0' as u8)); 
        let len = longer.len() - shorter.len();
        for i in 0..shorter.len() {
            let s = shorter[i] - '0' as u8;
            bin[len + i] &= s;
        }
        for n in bin.iter_mut() {
            *n += '0' as u8;
        }
        Number::from_bin(&String::from_utf8(bin).ok().unwrap(), bits).ok().unwrap()
    }
}

impl std::ops::BitOr for Number {
    type Output = Number;
    fn bitor(self, rhs: Self) -> Self::Output {
        let bits = usize::max(self.bits, rhs.bits);
        let (longer, shorter) = if self.bits > rhs.bits {
            (self.bin.as_bytes(), rhs.bin.as_bytes())
        } else {
            (rhs.bin.as_bytes(), self.bin.as_bytes())
        };
        let mut bin = Vec::from_iter(longer.iter().map(|x| *x - '0' as u8)); 
        let len = longer.len() - shorter.len();
        for i in 0..shorter.len() {
            let s = shorter[i] - '0' as u8;
            bin[len + i] |= s;
        }
        for n in bin.iter_mut() {
            *n += '0' as u8;
        }       
        Number::from_bin(&String::from_utf8(bin).unwrap(), bits).unwrap()
    }
}

impl std::ops::BitXor for Number {
    type Output = Number;
    fn bitxor(self, rhs: Self) -> Self::Output {
        let bits = usize::max(self.bits, rhs.bits);
        let (longer, shorter) = if self.bits > rhs.bits {
            (self.bin.as_bytes(), rhs.bin.as_bytes())
        } else {
            (rhs.bin.as_bytes(), self.bin.as_bytes())
        };
        let mut bin = Vec::from_iter(longer.iter().map(|x| *x - '0' as u8)); 
        let len = longer.len() - shorter.len();
        for i in 0..shorter.len() {
            let s = shorter[i] - '0' as u8;
            bin[len + i] ^= s;
        }
        for n in bin.iter_mut() {
            *n += '0' as u8;
        }       
        Number::from_bin(&String::from_utf8(bin).unwrap(), bits).unwrap()       
    }
}

impl std::ops::BitAndAssign for Number {
    fn bitand_assign(&mut self, rhs: Self) {
        let bits = usize::max(self.bits, rhs.bits);
        let (longer, shorter) = if self.bits > rhs.bits {
            (self.bin.as_bytes(), rhs.bin.as_bytes())
        } else {
            (rhs.bin.as_bytes(), self.bin.as_bytes())
        };
        let mut bin = Vec::from_iter(longer.iter().map(|x| *x - '0' as u8)); 
        let len = longer.len() - shorter.len();
        for i in 0..shorter.len() {
            let s = shorter[i] - '0' as u8;
            bin[len + i] &= s;
        }
        for n in bin.iter_mut() {
            *n += '0' as u8;
        }       
        self.bits = bits;
        self.bin = String::from_utf8(bin).unwrap();
    }
}

impl std::ops::BitOrAssign for Number {
    fn bitor_assign(&mut self, rhs: Self) {
        let bits = usize::max(self.bits, rhs.bits);
        let (longer, shorter) = if self.bits > rhs.bits {
            (self.bin.as_bytes(), rhs.bin.as_bytes())
        } else {
            (rhs.bin.as_bytes(), self.bin.as_bytes())
        };
        let mut bin = Vec::from_iter(longer.iter().map(|x| *x - '0' as u8)); 
        let len = longer.len() - shorter.len();
        for i in 0..shorter.len() {
            let s = shorter[i] - '0' as u8;
            bin[len + i] |= s;
        }
        for n in bin.iter_mut() {
            *n += '0' as u8;
        }       
        self.bits = bits;
        self.bin = String::from_utf8(bin).unwrap();       
    }
}

impl std::ops::BitXorAssign for Number {
    fn bitxor_assign(&mut self, rhs: Self) {
        let bits = usize::max(self.bits, rhs.bits);
        let (longer, shorter) = if self.bits > rhs.bits {
            (self.bin.as_bytes(), rhs.bin.as_bytes())
        } else {
            (rhs.bin.as_bytes(), self.bin.as_bytes())
        };
        let mut bin = Vec::from_iter(longer.iter().map(|x| *x - '0' as u8)); 
        let len = longer.len() - shorter.len();
        for i in 0..shorter.len() {
            let s = shorter[i] - '0' as u8;
            bin[len + i] ^= s;
        }
        for n in bin.iter_mut() {
            *n += '0' as u8;
        }       
        self.bits = bits;
        self.bin = String::from_utf8(bin).unwrap();       
    }
}

impl std::ops::Neg for Number {
    type Output = Number;
    fn neg(mut self) -> Self::Output {
        self.bin = Number::to_negtive(&self.bin, self.bits).unwrap();
        self
    }
}

impl std::ops::Not for Number {
    type Output = Number;
    fn not(mut self) -> Self::Output {
        self.bin = Number::flip(&self.bin).unwrap();
        self
    }
}

impl std::ops::Shl for Number {
    type Output = Number;
    fn shl(self, rhs: Self) -> Self::Output {
        let mut bin = self.bin;
        let bits = self.bits;
        let mut count = rhs.to_count();
        for i in 0..count.len() {
            bin = Self::shl(bin, bits, &mut count, i);
        }
        Self {
            bin,
            bits,
        }
    }
}

impl std::ops::ShlAssign for Number {
    fn shl_assign(&mut self, rhs: Self) {
        let mut bin = String::new();
        std::mem::swap(&mut self.bin, &mut bin);
        let bits = self.bits;
        let mut count = rhs.to_count();
        for i in 0..count.len() {
            bin = Self::shl(bin, bits, &mut count, i);
        }       
        self.bin = bin;
    }
}

impl std::ops::Shr for Number {
    type Output = Number;
    fn shr(self, rhs: Self) -> Self::Output {
        let mut bin = self.bin;
        let bits = self.bits;
        let mut count = rhs.to_count();
        for i in 0..count.len() {
            bin = Self::sshr(bin, bits, &mut count, i);
        }
        Self {
            bin,
            bits,
        }
    }
}

impl std::ops::ShrAssign for Number {
    fn shr_assign(&mut self, rhs: Self) {
        let mut bin = String::new();
        std::mem::swap(&mut self.bin, &mut bin);
        let bits = self.bits;
        let mut count = rhs.to_count();
        for i in 0..count.len() {
            bin = Self::sshr(bin, bits, &mut count, i);
        }       
        self.bin = bin;
    }
}


impl std::ops::Add for Number {
    type Output = Number;
    fn add(self, rhs: Self) -> Self::Output {
        if rhs == Self::zero(rhs.bits) {
            return self;
        }
        let bits = usize::max(self.bits, rhs.bits);
        let carry = (self.clone() & rhs.clone()) << Self::one(bits);
        let xor = self ^ rhs;
        xor + carry
    }
}

impl std::ops::AddAssign for Number {
    fn add_assign(&mut self, rhs: Self) {
        if rhs == Self::zero(rhs.bits) {
            return ;
        }
        let bits = usize::max(self.bits, rhs.bits);
        let carry = (self.clone() & rhs.clone()) << Self::one(bits);
        let xor = self.clone() ^ rhs;
        std::mem::swap(self, &mut (xor + carry));
    }
}

impl std::ops::Sub for Number {
    type Output = Number;
    fn sub(self, rhs: Self) -> Self::Output {
        if rhs == Self::zero(rhs.bits) {
            return self;
        }
        let rhs = -rhs;
        let bits = usize::max(self.bits, rhs.bits);
        let carry = (self.clone() & rhs.clone()) << Self::one(bits);
        let xor = self ^ rhs;
        xor + carry
    }
}

impl std::ops::SubAssign for Number {
    fn sub_assign(&mut self, rhs: Self) {
        if rhs == Self::zero(rhs.bits) {
            return ;
        }
        let bits = usize::max(self.bits, rhs.bits);
        let rhs = -rhs;
        let carry = (self.clone() & rhs.clone()) << Self::one(bits);
        let xor = self.clone() ^ rhs;
        std::mem::swap(self, &mut (xor + carry));       
    }
}

impl std::ops::Mul for Number {
    type Output = Number;
    fn mul(mut self, mut rhs: Self) -> Self::Output {
        let bits = usize::max(self.bits, rhs.bits);
        self.extend(bits);
        rhs.extend(bits);
        let mut answer = Self::zero(bits);
        while rhs != Self::zero(rhs.bits) {
            if rhs.test() {
                answer = answer + self.clone();
            }
            self <<= Self::one(self.bits);
            rhs.ushr_number();
        }
        answer
    }
}

impl std::ops::MulAssign for Number {
    fn mul_assign(&mut self, mut rhs: Self) {
        let bits = usize::max(self.bits, rhs.bits);
        let mut lhs = self.clone();
        lhs.extend(bits);
        rhs.extend(bits);
        let mut answer = Self::zero(bits);
        while rhs != Self::zero(rhs.bits) {
            if rhs.test() {
                answer = answer + lhs.clone();
            }
            lhs <<= Self::one(lhs.bits);
            rhs.ushr_number();
        }
        std::mem::swap(self, &mut lhs); 
    }
}

#[cfg(test)]
mod test_number {
}
