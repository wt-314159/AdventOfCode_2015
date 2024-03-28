use md5::{Md5, Digest};

fn main() {
    let input = "ckczppom";
    let mut counter = 0;

    loop {
        counter += 1;
        let mut hasher = Md5::new();
        let input_str = format!("{}{}", input, counter.to_string());

        hasher.update(input_str.as_bytes());
        let result = hasher.finalize();

        let hex = format!("{:02x}", result);

        // use five 0's for first test, six for second test
        if hex.starts_with("000000") {
            println!("Answer found! {}", counter);
            break;
        }
    }
}
