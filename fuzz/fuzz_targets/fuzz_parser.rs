use honggfuzz::fuzz;
use sqllogictest::{parse, DefaultColumnType};

fn main() {
    loop {
        fuzz!(|data: &[u8]| {
            if let Ok(data_str) = std::str::from_utf8(data) {
                let _ = parse::<DefaultColumnType>(data_str);
            }
        });
    }
}