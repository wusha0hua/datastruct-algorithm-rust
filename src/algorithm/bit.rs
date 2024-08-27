
fn zero<T>(a: T) -> T where T: std::ops::BitXor<Output = T> + Copy {
    a ^ a

}

fn one_with_zeros<T>(a: T) -> Option<T> where 
T: std::ops::BitXor<Output = T> + std::ops::Shl<usize, Output = T> + Copy + PartialEq {
    if a == zero(a) {
        return None;
    }
    let mut one_with_zeros = a;
    loop {
        if one_with_zeros << 1 == zero(a) {
            break Some(one_with_zeros);
        }
        one_with_zeros = one_with_zeros << 1;
    }
}

fn one<T>(mut a: T) -> Option<T> where 
T: std::ops::BitXor<Output = T> + std::ops::BitAnd<Output = T> + std::ops::Shr<usize, Output = T> + std::ops::Shl<usize, Output = T> + std::ops::Not<Output = T> + Copy + PartialEq {
    let zero_with_ones = match one_with_zeros(a) {
        Some(one_with_zeros) => !one_with_zeros,
        None => return None,
    };
    loop {
        if (a >> 1) & zero_with_ones == zero(a) {
            break Some(a);
        }
        a = (a >> 1) & zero_with_ones;
    }
}


pub fn swap<T: std::ops::BitXor<Output = T> + Copy>(a: &mut T, b: &mut T) {
    *a = *a ^ *b;
    *b = *a ^ *b;
    *a = *a ^ *b;
}

pub fn max(a: i64, b: i64) -> i64 {
    let sign = |a: i64| -> i64 {
        a >> 63 
    };
    let flip = |a: i64| -> i64 {
        (a & 1) ^ 1
    };
    let c = a - b;
    let rb = sign(c);
    let ra = flip(rb);
    ra * a + rb * b
}

pub fn add<T>(a: T, b: T) -> T where 
T: std::ops::BitAnd<Output = T> + std::ops::BitXor<Output = T> + std::ops::Shl<usize, Output = T> + Copy + PartialEq {
    let carry = |a: T, b: T| -> T {
        (a & b) << 1
    };
    let xor = |a: T, b: T| -> T {
        a ^ b
    };
    let carry = carry(a, b);
    let xor = xor(a, b);
    if carry != zero(carry) {
        add(xor, carry)
    } else {
        xor
    }
}

pub fn sub<T>(a: T, b: T) -> T where 
T: std::ops::BitAnd<Output = T> + std::ops::BitXor<Output = T> + std::ops::Shl<usize, Output = T> + std::ops::Shr<usize, Output = T> + std::ops::Not<Output = T> + Copy + PartialEq {
    match one(b) {
        Some(one) => add(add(!b, one), a),
        None => a,
    }
}

pub fn mul<T>(mut a: T, mut b: T) -> T where 
T: std::ops::BitAnd<Output = T> + std::ops::BitXor<Output = T> + std::ops::Shl<usize, Output = T> + std::ops::Shr<usize, Output = T> + std::ops::Not<Output = T> + Copy + PartialEq {
    let (one_with_zeros, one) = match (one_with_zeros(b), one(b)) {
        (Some(one_with_zeros), Some(one)) => (one_with_zeros, one),
        _ => return zero(b),
    };
    let zero_with_ones = !one_with_zeros;
    let mut ans = b ^ b;
    loop {
        if b == b ^ b {
            break ans;
        }
        if b & one != zero(b) {
            ans = add(ans, a);
        }
        a = a << 1;
        b = (b >> 1) & zero_with_ones;
    }
}

pub fn div<T>(mut a: T, mut b: T) -> Option<(T, T)> where 
T: std::ops::BitAnd<Output = T> + std::ops::BitXor<Output = T> + std::ops::Shl<usize, Output = T> + std::ops::Shr<usize, Output = T> + std::ops::Not<Output = T> + Copy + PartialEq {
    
    None
}
