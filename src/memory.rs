pub mod memory {
    use std::collections::HashMap;

    pub trait IMemory {
        fn get(&mut self, address: u16) -> &mut u16;
    }

    pub struct Memory {
        data: HashMap<u16, u16>,
    }

    impl Memory {
        pub fn new(start_address: u16, size_in_bytes: u16) -> Self {
            let mut mem = HashMap::<u16, u16>::new();

            for n in start_address..start_address + size_in_bytes {
                mem.insert(n, 0);
            }

            Self { data: mem }
        }
    }

    impl IMemory for Memory {
        fn get(&mut self, address: u16) -> &mut u16 {
            return self.data.get_mut(&address).unwrap(); // [&address];
        }
    }
}
