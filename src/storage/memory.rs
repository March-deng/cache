use crate::{Cache, CacheError, Stat};
use std::{collections::HashMap, sync::RwLock};

pub struct InMemoryCache {
    container: RwLock<cache_container>,
}

struct cache_container {
    c: HashMap<String, Vec<u8>>,
    stat: Stat,
}

impl cache_container {
    fn new() -> Self {
        cache_container {
            c: HashMap::new(),
            stat: Stat::default(),
        }
    }
}

impl InMemoryCache {
    pub fn new() -> Self {
        InMemoryCache {
            container: RwLock::new(cache_container::new()),
        }
    }
}

impl Cache for InMemoryCache {
    fn del(&mut self, key: &str) -> Result<(), CacheError> {
        let mut container = self.container.write().unwrap();
        match container.c.remove(key) {
            Some(v) => {
                container.stat.del(key, &v);
            }
            None => {}
        }
        Ok(())
    }

    fn get(&mut self, key: &str) -> Result<Vec<u8>, CacheError> {
        let container = self.container.read().unwrap();
        match container.c.get(key) {
            Some(v) => Ok(v.to_owned()),
            None => Err(CacheError::NotFound(key.into())),
        }
    }

    fn set(&mut self, key: &str, value: &[u8]) -> Result<(), CacheError> {
        let mut container = self.container.write().unwrap();
        match container.c.insert(key.into(), value.to_owned()) {
            Some(old_value) => {
                container.stat.update(key, &old_value, &value);
            }
            None => {
                container.stat.add(key, value);
            }
        }

        Ok(())
    }

    fn get_stat(&self) -> Stat {
        let stat = self.container.read().unwrap();
        stat.stat.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Stat;
    #[test]
    fn test_in_memory_cache() {
        let mut cache = InMemoryCache::new();

        let init_stat = cache.get_stat();
        assert_eq!(
            init_stat,
            Stat {
                count: 0,
                key_size: 0,
                value_size: 0,
            }
        );
        assert!(cache.get("test_key_1").is_err());
        assert!(cache
            .set("test_key_1", "test_key_value_1".as_bytes())
            .is_ok());

        let cut_stat = cache.get_stat();
        assert_eq!(
            cut_stat,
            Stat {
                count: 1,
                key_size: "test_key_1".len() as u64,
                value_size: "test_key_value_1".as_bytes().len() as u64,
            }
        );

        let cur_value = cache.get("test_key_1").unwrap();

        assert_eq!(cur_value, "test_key_value_1".as_bytes());
    }
}
