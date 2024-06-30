use rand::prelude::SliceRandom;
use zxcvbn::zxcvbn;

const UPPERCASE: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijgkmpqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    uppercase: bool,
    lowercase: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<()> {
    let mut rng = rand::thread_rng();
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if uppercase {
        chars.extend_from_slice(UPPERCASE);
        password.push(
            *UPPERCASE
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }
    if lowercase {
        chars.extend_from_slice(LOWERCASE);
        password.push(
            *LOWERCASE
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }
    if number {
        chars.extend_from_slice(NUMBER);
        password.push(
            *NUMBER
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }
    if symbol {
        chars.extend_from_slice(SYMBOL);
        password.push(
            *SYMBOL
                .choose(&mut rng)
                .expect("chars won't be empty in this context"),
        );
    }

    for _ in 0..(length - password.len() as u8) {
        //let idx = rng.gen_range(0..chars.len());
        //password.push(chars[idx] as char);
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in this context");
        password.push(*c);
    }

    password.shuffle(&mut rng);

    let password = String::from_utf8(password)?;

    println!("{}", password);
    let estimate = zxcvbn(&password, &[]);
    eprintln!("Password strength: {}", estimate.score());

    Ok(())
}
