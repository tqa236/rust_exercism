use num_bigint::BigUint;
use num_traits::ToPrimitive;
use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p - 1)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    let p_big = BigUint::from(p);
    let g_big = BigUint::from(g);
    let a_big = BigUint::from(a);

    let public_key = mod_exp(g_big, a_big, p_big);

    public_key.to_u64().unwrap_or(0)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    let p_big = BigUint::from(p);
    let b_pub_big = BigUint::from(b_pub);
    let a_big = BigUint::from(a);

    let secret_key = mod_exp(b_pub_big, a_big, p_big);

    secret_key.to_u64().unwrap_or(0)
}

fn mod_exp(base: BigUint, exp: BigUint, p: BigUint) -> BigUint {
    let mut result = BigUint::from(1u64);
    let mut base = base % &p;
    let mut exp = exp;

    while exp > BigUint::from(0u64) {
        if &exp % 2u64 == BigUint::from(1u64) {
            result = (result * &base) % &p;
        }
        base = (&base * &base) % &p;
        exp >>= 1;
    }

    result
}
