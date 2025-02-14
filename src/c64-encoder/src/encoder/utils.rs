pub fn print_hexdump(bytes: &[u8]) {
    bytes.chunks(16).for_each(|chunk| {
        let mut line = Vec::new();
        for byte in chunk {
            line.push(format!("${:02X}", byte));
        }
        println!("  byte {}", line.join(", ").trim_end());
    });
}

pub fn print_vechex(bytes: &[u8]) {
    let mut line = Vec::new();
    for byte in bytes {
        line.push(format!("0x{:02X}", byte));
    }
    println!("  vec![{}]", line.join(", ").trim_end());
}
