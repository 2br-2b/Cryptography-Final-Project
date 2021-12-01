use std::io;
use rand::Rng;
use primes;
use modinverse::modinverse;
use num::BigInt;

fn generate_prime() -> i128 {
    loop{
        let base: i128 = 2;
        let number = rand::thread_rng().gen_range(1..=base.pow(32));

        if primes::is_prime(number.try_into().unwrap()){
            return number;
        }else{
            continue;
        }
        
    }
}


fn main(){

    let p : i128= generate_prime();
    let q : i128 = generate_prime();

    let n : i128 = p * q;

    let phi : i128 = (p - 1) * (q - 1);

    let e: i128 = 2_i128.pow(16) + 1;

    let mut d:i128 = 0;

    let does_exist = modinverse(e, phi);

    match does_exist {
        Some(x) => d = x,
        None => panic!("modinverse() didn't work as expected"),
    }


    
    println!("p = {}", p);
    println!("q = {}", q);
    println!("e = {}", e);
    println!("d = {}", d);
    println!("n = {}", n);

    println!("Input a number to encrypt:");
    let p:i128 = input_int();
    let c:u64 = mod_pow(p, e, n);

    println!("{} ^ {} mod {} = {}", p, e, n, c);

    let decrypted:u64 = mod_pow(c.into(), d, n).into();

    println!("{} ^ {} mod {} = {}", c, d, n, decrypted);

}

fn mod_pow(base: i128, exp: i128, modulus: i128) -> u64 {
   
    // **Warning**: Converting from BigInts to u64, so there may be some slight data loss

    let result = BigInt::modpow(&BigInt::from(base), &BigInt::from(exp), &BigInt::from(modulus));
    
    let (_a, b) = result.to_u64_digits();
    
    b[0]
}


fn input_int() -> i128 {
    loop{
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        match guess.trim().parse(){
            Ok(num) => return num,
            Err(_) => {
                println!("Please enter a valid number!\n");
                continue;
            },
        };
    }
}