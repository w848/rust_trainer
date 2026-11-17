mod temperature {
    fn c_to_f(c: f64) -> f64 {
        todo!()
    }

    fn f_to_c(f: f64) -> f64 {
        todo!()
    }

    mod describe {
        fn feeling(c: f64) -> &'static str {
            todo!()
        }

        fn feeling_f(f: f64) -> &'static str {
            todo!()
        }
    }
}

use temperature::describe::feeling;

fn main() {
    println!("20 °C = {} °F", temperature::c_to_f(20.0));
    println!("30 °C feels {}", feeling(30.0));
}
