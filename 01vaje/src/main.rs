/// Skupaj preverite in pokomentirajte kvize iz [učbenika](https://rust-book.cs.brown.edu/ch03-00-common-programming-concepts.html)

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `fib`, ki sprejme začetna člena fibbonacijevega zaporedja, število `n` in vrne `n`-ti člen zaporedja

fn fib(a0: u32, a1: u32, n: u32) -> u32 {
    if n == 0 {
        return a0;
    }
    else if n == 1 {
        return a1;
    }
    else {
        return fib(a0, a1, n - 1) + fib(a0, a1, n - 2) ;
    }
}
/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_prestopno`, ki za podano leto preveri, ali je prestopno

fn je_prestopno(l: u32) -> bool {
    if l % 400 == 0 {
        return true;
    }
    else if l % 100 == 0 {
        return false;
    }
    else if l % 4 == 0 {
        return true;
    }
    else {
        return false;
    }
}



/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `je_veljaven_datum(datum: Date) -> bool`, ki preveri, ali je datum veljaven

// Dan, mesec, leto
type Date = (u32, u32, u32);

//fn je_veljaven_datum(datum: Date) -> bool {
//    if datum.1 > 12 {
//            return false}
//    else if je_prestopno(datum.2) {
//        let d = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
//        if datum.0 <= d[datum.1 - 1] {
//            return true;
//        }
//        else {
//            return false;
//        }
//    }
//    else {
//        let d = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
//        if datum.0 <= d[datum.1 - 1] {
//            return true;
//        }
//        else {
//            return false;
//        }
//    }
//}


/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo `iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32`, ki sprejme iteracijsko funkcijo, zaustavitveni pogoj in začetno vrednost.
/// Iteracijsko funkcijo zaporedoma uporablja, dokler za rezultat ne velja zaustavitveni pogoj, in vrne prvi rezultat, ki zadošča zaustavitvenemu pogoju.

fn iteracija(mut start: u32, fun: fn(u32) -> u32, cond: fn(u32) -> bool) -> u32 {
    if cond(start) {
        return start;
    }
    else {
        return iteracija(fun(start), fun, cond)
    }
}

/// ------------------------------------------------------------------------------------------------

/// Napišite funkcijo, ki izračuna ničlo zvezne funkcije s pomočjo bisekcije.
/// Postopek bisekcije je sledeč:
/// 1. Izberemo interval [a, b], kjer je f(a) * f(b) < 0
/// 2. Izračunamo sredino intervala c = (a + b) / 2
/// 3. Če je |f(c)| < prec ali je dolžina intervala manjša od določene natančnosti, vrnemo c
/// 4. Če ni, izberemo nov interval [a, b]  glede na predznak f(c)
/// 5. Ponavljamo korake 2-4

fn bisekcija(mut a: f64, mut b: f64, fun: fn(f64) -> f64, prec: f64) -> f64 {
    let mut c  = (a + b) / 2.; 
    while (fun(c)).abs() > prec {
        if fun(a) * fun(c) <= 0. {
            b = (a + c) / 2.
        }
        else {
            a = (c + b) / 2.
        };
        c  = (a + b) / 2.; 
    }
    return c
}

/// ------------------------------------------------------------------------------------------------

/// Popravite igro ugibanja iz prejšnje naloge, da bo delovala sledeče
/// Uporabnika sprašujemo po novi številki, vse dokler so števila, ki jih vpisuje del nekega aritmetičnega zaporedja
/// Če uporabnik vpiše neveljavno število to ni napaka, program za pogoj aritmetičnega zaporedja upošteva samo veljavno vpisana števila.

fn guessing_game() {
    panic!("Not implemented");
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2]`, ki matriki `a` in `b` zmnoži in vrne rezultat

fn mat_mul(a: [[u32; 2]; 2], b: [[u32; 2]; 2]) -> [[u32; 2]; 2] {
    return [[a[1][1]*b[1][1]+a[1][2]*b[2][1], a[1][1]*b[1][2]+a[1][2]*b[2][2]], 
    [a[2][1]*b[1][1]+a[2][2]*b[2][1], a[2][1]*b[1][2]+a[2][2]*b[2][2]]];
}

/// ------------------------------------------------------------------------------------------------
/// Napišite funkcijo `ordered`, ki sprejme tabelo števil in vrne `true`, če so števila urejena (padajoče ali naraščajoče) in `false` sicer.

fn ordered(arr: &[u32]) -> bool {
    if arr[0] <= arr[1] {
    let mut i = 0;
    while i <= (arr.len() - 1) {
        if arr[i] > arr[i - 1] {
            return false
        }
        else {
            continue
        }
        };
    };
    else if arr[0] >= arr[1] {
    while i >= (arr.len() - 1) {
        if arr[i] > arr[i - 1] {
            return false
        }
        else {
            continue
        };
        };    
    }
}

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje
/// Napišite funkcijo `fn pow(mut x: u32, mut n: u32) -> u32`, ki izračuna `x` na potenco `n` v času O(log n)
/// Hitro potenciranje izgleda tako:
/// 1. Če je `n` sodo, potem je `x^n = (x^(n/2))^2`
/// 2. Če je `n` liho, potem je `x^n = (x^2)^(n/2)`
/// 3. Če je `n = 0`, potem je `x^n = 1`

/// ------------------------------------------------------------------------------------------------
/// Prepišite hitro potenciranje v iterativno obliko

/// ------------------------------------------------------------------------------------------------
/// Hitro potenciranje deluje tudi, če nas zanima samo ostanek po deljenju z nekim številom `m`
/// Napišite funkcijo `fn pow_mod(mut x: u32, mut n: u32, m: u32) -> u32`, ki izračuna `x` na potenco `n` in vrne ostanek po deljenju z `m`
/// Postopek je enak, le da pri vsakem izračunu vrnemo ostanek pri deljenju z `m`

/// ------------------------------------------------------------------------------------------------
/// Urejanje z izbiranjem
/// Napišite funkcijo `fn selection_sort(mut arr: [u32])`, ki uredi tabelo `arr` z uporabo algoritma urejanja z izbiranjem

fn selection_sort(mut arr: &[u32]) {}

fn main() {
    let r = bisekcija(0., 1., identiteta, 0.1 );
    println!("{}", r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = main();
        assert_eq!(result, ());
    }

    #[test]
    fn test_fib() {
        assert_eq!(fib(0, 1, 0), 0);
        assert_eq!(fib(0, 1, 1), 1);
        assert_eq!(fib(0, 1, 2), 1);
        assert_eq!(fib(0, 1, 3), 2);
        assert_eq!(fib(0, 1, 4), 3);
        assert_eq!(fib(0, 1, 5), 5);
        assert_eq!(fib(0, 1, 6), 8);
        assert_eq!(fib(0, 1, 7), 13);
        assert_eq!(fib(0, 1, 8), 21);
        assert_eq!(fib(0, 1, 9), 34);
        assert_eq!(fib(0, 1, 10), 55);
    }
    #[test]
    fn test_je_prestopno() {
        assert_eq!(je_prestopno(2013), false);
        assert_eq!(je_prestopno(2024), true);
        assert_eq!(je_prestopno(2000), true);
        assert_eq!(je_prestopno(1900), false);
    }
}
