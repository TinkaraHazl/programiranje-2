use crate::sequence::models::Sequence;

// Implementirajte konstantno zaporedje
pub struct Constant<T> {
    vrednost : T,
}

impl Constant<i64> {
    pub fn new(c : i64) -> Constant<i64> {
        Constant {
            vrednost,
        }
    }

    pub fn name(&self) -> String {
        let ime = format!("{}", self.vrednost);
        return ime
    }

    fn start(&self) -> i64 {
        return self.vrednost
    }

    fn k_th(&self, k: usize) -> i64 {
        return self.vrednost
    }

    fn contains(&self, item: i64) -> bool {
        return self.vrednost == item
    }
    
}