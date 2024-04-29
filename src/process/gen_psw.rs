use rand::{seq::SliceRandom, thread_rng};

pub fn process_genpass(length: u8, upper:bool,lower:bool,number:bool,symbol:bool) ->anyhow::Result<()>{
    let mut psw =String::new();
    let mut chars=Vec::new();
    if upper{
        chars.extend_from_slice(b"ABCDEFGHJKLMNPQRSTUVWXYZ");
    }
    if lower{
        chars.extend_from_slice(b"abcdefghijkmnopqrstuvwxyz");
    }
    if number{
        chars.extend_from_slice(b"123456789");

    }
    if symbol{
        chars.extend_from_slice(b"!@#$%^&*_");
    }
    let mut rng=thread_rng();

    for _ in 0..length{
        let c= chars.choose(&mut rng).expect("msg");
        psw.push(*c as char);

    }
    println!("{}",psw);
    Ok(())
}
