struct AritmeticnoZaporedje {
    zacetno: i64,
    skok: i64,
    trenutno: i64,
}

//sestevanje, mnozenje, nastevanje clenov

impl AritmeticnoZaporedje {
    fn new(n: i64, a: i64) -> Self {
        Self {
        zacetno : n,
        skok : a,
        trenutno : n,
        }
    }
}

impl AritmeticnoZaporedje {
    fn next(&self) -> i64 {
        Self {
            zacetno : self.zacetno,
            skok : self.skok,
            trenutno : self.skok + self.zacetno,
        };
        return self.trenutno;
    }
}

impl AritmeticnoZaporedje {
    fn n_th(n, self) -> i64 {
        let mut i = 1;
        while i <= n {
            self.next();
            i = i + 1
        };
        return self.trenutno
    }
}



fn main() {
    let zap = AritmeticnoZaporedje::new(1, 1);
    println!("test test {} {}", zap.next(), zap.next())
}