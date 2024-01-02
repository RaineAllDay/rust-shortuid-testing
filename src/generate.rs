
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{thread_rng, Rng};


pub fn generate_id() -> String {
    let start = SystemTime::now();
    let since_the_epoch = start
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards").as_micros();
    let time_as_bytes = since_the_epoch.to_be_bytes();
    let mut out: Vec<u8> = vec![];
    for b in time_as_bytes {
        let mut rng = thread_rng();
        if b == 0 || b < 48 {
            continue;
        }
        if is_in_range(b) {
            out.push(b);
        }
        else {
            let n: u8= rng.gen_range(48..122);
            if is_in_range(n) {
                out.push(n);
            } else {
                let nn: u8 = rng.gen_range(7..13);
                let new_val = n + nn;
                out.push(new_val);
            }
        } 
    }
    while out.len() < 8 {
        let mut rng = thread_rng();
        let n: u8= rng.gen_range(48..122);
        if is_in_range(n) {
            out.push(n);
        } else {
            let nn: u8 = rng.gen_range(7..13);
            let new_val = n + nn;
            out.push(new_val);
        }
    }
    return String::from_utf8(out).unwrap()
}

fn is_in_range(num: u8) -> bool {
    if num >= 48 && num <= 57 {
        return true;
    } if num >= 65 && num <= 90 {
        return true;
    } if num >= 97 && num <= 122 {
        return true;
    } return false;
}