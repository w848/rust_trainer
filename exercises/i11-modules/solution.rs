mod temperature {
    pub fn c_to_f(c: f64) -> f64 {
        c * 9.0 / 5.0 + 32.0
    }

    pub fn f_to_c(f: f64) -> f64 {
        (f - 32.0) * 5.0 / 9.0
    }

    pub mod describe {
        pub fn feeling(c: f64) -> &'static str {
            if c < 10.0 {
                "cold"
            } else if c < 25.0 {
                "nice"
            } else {
                "hot"
            }
        }

        pub fn feeling_f(f: f64) -> &'static str {
            feeling(super::f_to_c(f))
        }
    }
}

use temperature::describe::feeling;

fn main() {
    println!("20 °C = {} °F", temperature::c_to_f(20.0));
    println!("30 °C feels {}", feeling(30.0));
}
