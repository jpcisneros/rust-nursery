use rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};

fn first() {
    let mut rng = rand::thread_rng();

    let n1: u8 = rng.gen();
    let n2: u64 = rng.gen();

    println!("Random u8 {:?}", n1);
    println!("Random u64 {:?}", n2);
    println!("Random u32 {:?}", rng.gen::<u32>());
    println!("Random float{:?}", rng.gen::<f64>());
}

fn second() {
    let mut rng = rand::thread_rng();

    println!("Integer is {}", rng.gen_range(0..100));
    println!("Float is {}", rng.gen_range(0.0..10.0));
}

fn normal_distribution() -> Result<(),NormalError>{
    let mut rng = rand::thread_rng();

    let normal = Normal::new(2.0,3.0)?;
    let v = normal.sample(&mut rng);
    println!("{} is from a N(2,9) distribution", v);
    Ok(())
}

fn password_generator(passwd_len: u8) {
    const CHARSET: &[u8] = b"QWERTYUIOPASDFGHJKLZXCVBNM\
                            qwertyuiopasdfghjklzxcvbnm\
                            1234567890/*-+!@#~{[]}][}{";
    
    if passwd_len < 5 {
        eprintln!("Password too short");
    }
    let mut rng = rand::thread_rng();

    let password:String = (0..passwd_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    println!("{:?}",password);
}


fn main() {
    first();
    second();
    normal_distribution().unwrap();
    password_generator(3);
    password_generator(30);
    password_generator(60);
}
