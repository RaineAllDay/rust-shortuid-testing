mod generate;
use std::{thread, time, sync::Arc};
use rand::{thread_rng, Rng};
use boxcar::Vec;

fn main() {
    let vec:Arc<Vec<String>> = Arc::new(boxcar::Vec::new());
    let threads = (0..200)
    .map(|t| {
        let vec = vec.clone();
        thread::spawn(move || {
            let mut i = 0;
            while i < 100000 {
                println!("id: {:?}, Iter: {:?}", t, i);
                let mut rng = thread_rng();
                let n: u64= rng.gen_range(0..100);
                let dur = time::Duration::from_millis(n);
                vec.push(generate::generate_id());
                thread::sleep(dur);
                i = i + 1;
            }
        })
    }).collect::<Vec<_>>();

    for thread in threads {
        thread.join().unwrap();
    }
    println!("Threads Finished Total Count: {:?}", vec.count());
    let mut s: std::vec::Vec<String> = std::vec![];
    vec.iter().for_each(|sid| {
            s.push(sid.1.to_string());
    });
    s.sort();
    s.dedup();
    let dupes = vec.count() - s.len();
    println!("Completed! Total Ids_Generated: {:?}, Dupes: {:?}", vec.count(), dupes);
        /*let id = generate_id();
        println!("Id: {:?}", id);*/
}



/*fn timestamp_prefix() {

}*/
