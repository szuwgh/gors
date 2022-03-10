pub struct Builder {
    data: Vec<u8>,
}

impl Builder {
    pub fn new() -> Builder {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, key: &[u8]) {
        self.data.extend_from_slice(key)
    }

    pub fn get(&self) -> &[u8] {
        &self.data
    }

    pub fn len(&self) -> u32 {
        self.data.len() as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut b = Builder::new();
        b.add("aaa".as_bytes());
        b.add("bbb".as_bytes());
        println!("{:?}", b.get())
    }
}
