struct AritmeticnoZaporedje {
    zacetno: i64,
    skok: i64,
    trenutno: i64,
}

//sestevanje, mnozenje, nastevanje clenov

impl AritmeticnoZaporedje {
    fn new(zacetno: i64, skok: i64) -> AritmeticnoZaporedje {
        Self {
        zacetno,
        skok,
        trenutno : zacetno,
        }
    }

    fn next(&mut self) -> i64 {
        self.trenutno += self.skok;
        return self.trenutno
    }

    fn n_th(&self, n: i64) -> i64 {
        return self.zacetno + n * self.skok
    }

    fn reset(&mut self) {
        self.trenutno = 0
        }   

    fn current(&self) -> i64 {
        return self.trenutno
    }

    fn sum(&self, n:i64) -> i64 {
        let mut suma = 0;
        for i in 0..n {
            suma += self.zacetno + i * self.skok
        }
        return suma
    }

    fn vsota(&self, other : &Self) -> Self {
        Self {
            zacetno : self.zacetno + other.zacetno,
            skok : self.skok + other.skok,
            trenutno : self.trenutno + self.skok
        }
    }

    //fn produkt(&self, other : &Self) {
    //    Self {
    //        zacetno : self.zacetno * other.zacetno,
    //        skok : self.skok * other.skok
    //        trenutno : self.trenutno * other.trenutno
    //    }
    //}
    // ne gre, imamo kvadratni člen: a1*a2 + n(d1a2 + a1d2) + n^2(d1d2)
}

enum BinOperacija {
    Plus,
    Minus,
    Times,
}

enum Izraz {
    Konstanta (u32),
    Operacija (Box<Izraz>, BinOperacija, Box<Izraz>),
}
//brez Box je izraz rekurziven brez "dereferencinga"

//const sedem : Izraz::Operacija (Box<Izraz>::Konstanta (1 : <u32>) , BinOperacija::Plus, 
            //Box<Izraz>::Operacija (Box<Izraz>::Konstanta(2), BinOperacija::Times, Box<Izraz>::Konstanta (3)))

impl Izraz {
    fn eval(&self) -> u32 {
        match self {
            Izraz::Konstanta(k) => *k,
            Izraz::Operacija(levo, operacija, desno) => match operacija {
                BinOperacija::Plus => levo.eval() + desno.eval(),
                BinOperacija::Minus => levo.eval() - desno.eval(),
                BinOperacija::Times => levo.eval() * desno.eval()
            }
        }
    }

    fn collect(&self) -> u32 {
        let mut st = 0;
        match self {
            Izraz::Konstanta(k) => 1
            Izraz::Operacija(levo, operacija, desno) => levo.collect() + desno.collect()
        }
    }

    fn izpis(&self) -> String {
        Izraz::Konstanta(k) => format!("{}", *k),
            Izraz::Operacija(levo, operacija, desno) => match operacija {
                BinOperacija::Plus => format!("({} + {})", levo.izpis(), desno.izpis())
                BinOperacija::Minus => format!("({} - {})", levo.izpis(), desno.izpis()),
                BinOperacija::Times => format!("({} * {})", levo.izpis(), desno.izpis())
        }
    }
 }

fn main() {
    let zap = AritmeticnoZaporedje::new(1, 1);
    println!("test test {} {}", (&zap).sum(4), (&zap).sum(5))
}