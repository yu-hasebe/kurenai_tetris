use derive_new::new;

#[derive(Clone, Copy, Debug, Eq, new, PartialEq)]
pub struct Count(i32);

impl Count {
    pub fn add(&mut self) {
        self.0 = (self.0 + 1) % 128;
    }

    pub fn beat(&self, tempo: i32) -> bool {
        self.0 % tempo == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add1() {
        let mut count = Count(0);
        count.add();
        assert_eq!(Count(1), count);
    }

    #[test]
    fn test_add2() {
        let mut count = Count(127);
        count.add();
        assert_eq!(Count(0), count);
    }

    #[test]
    fn test_beat() {
        let count = Count(64);
        assert_eq!(true, count.beat(2));
        assert_eq!(false, count.beat(3));
    }
}
