pub fn NewCache(cache_type: CacheType) -> Box<dyn Cache> {
    todo!()
}

pub enum CacheType {
    InMemory,
}

pub trait Cache {
    fn set(&mut self, key: &str, value: &[u8]) -> Result<(), CacheError>;
    fn get(&mut self, key: &str) -> Result<Vec<u8>, CacheError>;
    fn del(&mut self, key: &str) -> Result<(), CacheError>;
    fn get_stat(&self) -> Stat;
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Stat {
    pub count: u64,
    pub key_size: u64,
    pub value_size: u64,
}

impl Stat {
    pub fn add(&mut self, key: &str, value: &[u8]) {
        self.count += 1;
        self.key_size += key.len() as u64;
        self.value_size += value.len() as u64;
    }

    pub fn del(&mut self, key: &str, value: &[u8]) {
        self.count -= 1;
        self.key_size -= key.len() as u64;
        self.value_size -= value.len() as u64;
    }

    pub fn update(&mut self, key: &str, old_value: &[u8], new_value: &[u8]) {
        self.value_size -= old_value.len() as u64;
        self.value_size += new_value.len() as u64;
    }
}

#[derive(Debug)]
pub enum CacheError {
    NotFound(String),
}

#[cfg(test)]
pub trait TestSuite {
    fn open() -> Box<dyn Cache> {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
