use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    // value: Option<u32>,
    cacheMap: HashMap<u32, u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            // value: None,
            cacheMap: HashMap::new(),
        }
    }
    pub fn value(&mut self, arg: u32) -> u32 {
        match self.cacheMap.get(&arg) {
            Some(v) => *v,
            None => {
                let v = (self.calculation)(arg);
                self.cacheMap.insert(arg, v);
                v
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_value() {
        let mut cacher = Cacher::new(|num| { num });

        assert_eq!(0, cacher.value(0));
    }
    #[test]
    fn many_values() {
        let mut cacher = Cacher::new(|num| { num });

        assert_eq!(0, cacher.value(0));
        assert_eq!(1, cacher.value(1));
        assert_eq!(2, cacher.value(2));
    }
}
