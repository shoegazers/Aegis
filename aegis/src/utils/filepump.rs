use std::io::Write;

pub fn pump(size: usize) {
    if cfg!(feature = "pump_file") {
        let mut file = std::fs::File::create("pump.bin").unwrap();
        let mut data = vec![0u8; size * 1024 * 1024];
        file.write_all(&data).unwrap();
    }
}
