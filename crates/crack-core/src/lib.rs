mod platform;
mod utils;
mod common;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use libc::{c_int, c_long, c_void};
    const PTRACE_WORD_SIZE: usize = 8;

    #[test]
    fn read_memory() {
        let data = [1, 2, 3, 4, 5, 6, 7];
        let aligned_index = 0;
        let mut bytes: [u8; PTRACE_WORD_SIZE] = (data
            [aligned_index..aligned_index + PTRACE_WORD_SIZE])
            .try_into()
            .unwrap();
        bytes[1] = 123;

        println!("{:?}", data);
        println!("{:?}", bytes);
    }
}
