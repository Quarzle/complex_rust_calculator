use complex_calc::Calculator;
use std::io::{self, Write};


fn main() {
    let mut calc = Calculator::new(); //this stores vars and previous ans

    loop {
        print!("Enter equation: ");
        io::stdout().flush().expect("Failed to flush stdout");

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        let text = user_input.trim();
        if text.is_empty() {
            return;
        }

        match calc.evaluate(text) {
            Ok(result) => {
                println!("{}", result);
            }
            Err(err) => println!("Error: {}", err),
        }
    }

	// calc.set_variable(name, value) and calc.get_variable(name) should also work
	// note that all variables in this project are not case sensitive
}