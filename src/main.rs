// future notes:
// Make an equation solver
// also double check if the amount of rounding is suitable

use std::fmt;
use std::io::{self, Write};
use std::env;
use std::collections::HashMap;

fn main() {

    let mut ans = Complex::real(0.0);
    let mut variables: HashMap<String, Complex> = HashMap::new();

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

        match evaluate(text, ans, &mut variables) {
            Ok(result) => {
                println!("{}", result);
                ans = result;
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}

// Complex numbers
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    re: f64,
    im: f64,
}

impl Complex {
	// Create a new complex number with the inputs
    fn new(re: f64, im: f64) -> Self {
        Self { re, im }
    }

	// create a new real number
    fn real(value: f64) -> Self {
        Self::new(value, 0.0)
    }

	// Create a new imaginary only number
    fn imaginary(value: f64) -> Self {
        Self::new(0.0, value)
    }

	// Create a new complex number equal to i
    fn i() -> Self {
        Self::new(0.0, 1.0)
    }

    fn add(self, other: Self) -> Self {
        Self::new(self.re + other.re, self.im + other.im)
    }

    fn sub(self, other: Self) -> Self {
        Self::new(self.re - other.re, self.im - other.im)
    }

    fn mul(self, other: Self) -> Self {
        Self::new(
            self.re * other.re - self.im * other.im,
            self.re * other.im + self.im * other.re,
        )
    }
	
	fn div(self, other: Self) -> Result<Self, String> {
		let a = self.re;
		let b = self.im;
		let c = other.re;
		let d = other.im;
	
		if c == 0.0 && d == 0.0 {
			return Err("Division by zero".to_string());
		}
	
		// Scale by the larger component to avoid squaring large numbers.
		if c.abs() >= d.abs() {
			let r = d / c;
			let denominator = c + d * r;
	
			Ok(Self::new(
				(a + b * r) / denominator,
				(b - a * r) / denominator,
			))
		} else {
			let r = c / d;
			let denominator = d + c * r;
	
			Ok(Self::new(
				(a * r + b) / denominator,
				(b * r - a) / denominator,
			))
		}
	}
	
    fn neg(self) -> Self {
        Self::new(-self.re, -self.im)
    }

    fn abs(self) -> f64 {
        self.re.hypot(self.im)
    }

    fn ln(self) -> Self {
        let arg = if self.im == 0.0 && self.re < 0.0 {
            std::f64::consts::PI
        } else {
            self.im.atan2(self.re)
        };

        Self::new(self.abs().ln(), arg)
    }

    fn exp(self) -> Self {
        let scale = self.re.exp();

        Self::new(scale * self.im.cos(), scale * self.im.sin())
    }

    fn pow(self, other: Self) -> Result<Self, String> {
        if self.re == 0.0 && self.im == 0.0 {
            if other.im == 0.0 && other.re > 0.0 {
                return Ok(Self::real(0.0));
            }

            if other.re == 0.0 && other.im == 0.0 {
                return Ok(Self::real(1.0));
            }

            return Err("Invalid power of zero".to_string());
        }

        Ok(other.mul(self.ln()).exp())
    }

	fn sqrt(self) -> Self {
		if self.re == 0.0 && self.im == 0.0 {
			return Self::real(0.0);
		}
	
		let r = self.re.hypot(self.im);
	
		if self.re >= 0.0 {
			let re = ((r + self.re) / 2.0).sqrt();
			let im = self.im / (2.0 * re);
			Self::new(re, im)
		} else {
			let im_abs = ((r - self.re) / 2.0).sqrt();
			let re = self.im.abs() / (2.0 * im_abs);
			Self::new(re, self.im.signum() * im_abs)
		}
	}

    fn sin(self) -> Self {
        Self::new(
            self.re.sin() * self.im.cosh(),
            self.re.cos() * self.im.sinh(),
        )
    }

    fn cos(self) -> Self {
        Self::new(
            self.re.cos() * self.im.cosh(),
            -self.re.sin() * self.im.sinh(),
        )
    }

    fn tan(self) -> Result<Self, String> {
        self.sin().div(self.cos())
    }

    // fn sinh(self) -> Self {
    //     Self::new(
    //         self.re.sinh() * self.im.cos(),
    //         self.re.cosh() * self.im.sin(),
    //     )
    // }

    // fn cosh(self) -> Self {
    //     Self::new(
    //         self.re.cosh() * self.im.cos(),
    //         self.re.sinh() * self.im.sin(),
    //     )
    // }

    // fn tanh(self) -> Result<Self, String> {
    //     self.sinh().div(self.cosh())
    // }

    fn asin(self) -> Self {
        let one = Self::real(1.0);
        let i = Self::i();

        let z_squared = self.mul(self);
        let inside_sqrt = one.sub(z_squared).sqrt();
        let inside_ln = i.mul(self).add(inside_sqrt);
        let ln_result = inside_ln.ln();

        // Multiply by -i
        Self::new(ln_result.im, -ln_result.re)
    }

    fn acos(self) -> Self {
        Self::real(std::f64::consts::FRAC_PI_2).sub(self.asin())
    }

    fn atan(self) -> Self {
        let one = Self::real(1.0);
        let i = Self::i();

        let iz = i.mul(self);
        let numerator = one.add(iz).ln().sub(one.sub(iz).ln());

        // Divide by 2i
        Self::new(numerator.im / 2.0, -numerator.re / 2.0)
    }
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let re = if self.re.abs() < 1e-12 { 0.0 } else { self.re };
        let im = if self.im.abs() < 1e-12 { 0.0 } else { self.im };

        if im == 0.0 {
            return write!(f, "{}", format!("{:.14}", re).trim_end_matches('0').trim_end_matches('.'));
        }

        if re == 0.0 {
            if im == 1.0 {
                return write!(f, "i");
            }
            if im == -1.0 {
                return write!(f, "-i");
            }
            return write!(f, "{}i", format!("{:.14}", im).trim_end_matches('0').trim_end_matches('.'));
        }

        if im < 0.0 {
            if im == -1.0 {
                write!(f, "{} - i", format!("{:.14}", re).trim_end_matches('0').trim_end_matches('.'))
            } else {
                write!(f, "{} - {}i", format!("{:.14}", re).trim_end_matches('0').trim_end_matches('.'), format!("{:.16}", -im).trim_end_matches('0').trim_end_matches('.'))
            }
        } else if im == 1.0 {
            write!(f, "{} + i", format!("{:.14}", re).trim_end_matches('0').trim_end_matches('.'))
        } else {
            write!(f, "{} + {}i", format!("{:.14}", re).trim_end_matches('0').trim_end_matches('.'), format!("{:.16}", im).trim_end_matches('0').trim_end_matches('.'))
        }
    }
}

// AST
#[derive(Debug)]
pub enum Expr {
    Number(Complex),
	Variable(String),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Unary {
        value: Box<Expr>,
        op: UnaryOp,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Neg,
    Sqrt,
    Sin,
    Cos,
    Tan,
    Asin,
    Acos,
    Atan,
    Abs,
    Real,
    Imaginary,
    Cis,
	Ln
}

// Lexer
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Text(String),
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    UpArrow,
    LParen,
    RParen,
    Invalid(char),
    EOF,
}

struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    fn next_token(&mut self) -> Token {
        while let Some(&c) = self.chars.peek() {
            if c.is_whitespace() {
                self.chars.next();
                continue;
            }

            if c.is_ascii_digit() || c == '.' {
                return self.read_number();
            }

            // Identifiers may start with a letter or underscore.
            if c.is_alphabetic() || c == '_' {
                return self.read_text();
            }

            self.chars.next();

            return match c {
                '+' => Token::Plus,
                '-' => Token::Minus,
                '*' => Token::Star,
                '^' => Token::UpArrow,
                '/' => Token::Slash,
                '(' => Token::LParen,
                ')' => Token::RParen,
                _ => Token::Invalid(c),
            };
        }

        Token::EOF
    }

    fn read_number(&mut self) -> Token {
        let mut num_str = String::new();
        let mut decimal_seen = false;

        while let Some(&c) = self.chars.peek() {
            if c.is_ascii_digit() {
                num_str.push(c);
                self.chars.next();
            } else if c == '.' && !decimal_seen {
                decimal_seen = true;
                num_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        match num_str.parse::<f64>() {
            Ok(value) => Token::Number(value),
            Err(_) => Token::Invalid('.'),
        }
    }

    fn read_text(&mut self) -> Token {
        let mut text_str = String::new();

        while let Some(&c) = self.chars.peek() {
            if c.is_alphanumeric() || c == '_' {
                text_str.push(c);
                self.chars.next();
            } else {
                break;
            }
        }

        Token::Text(text_str)
    }
}

// Parser
struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    ans: Complex,
}

impl<'a> Parser<'a> {
    fn new(mut lexer: Lexer<'a>, ans: Complex) -> Self {
        let current_token = lexer.next_token();

        Self {
            lexer,
            current_token,
            ans,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    // Get the binding power (precedence) of binary operators.
    fn get_precedence(token: &Token) -> u32 {
        match token {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash => 2,
            Token::UpArrow => 4,
            _ => 0,
        }
    }

    fn parse_expression(&mut self, precedence: u32) -> Result<Expr, String> {
        // Prefix parsing
        let mut left = match self.current_token.clone() {
            Token::Invalid(c) => {
                return Err(format!("Unrecognised character: '{}'", c));
            }

            Token::Number(val) => {
                self.advance();
                Expr::Number(Complex::real(val))
            }

            Token::Text(val) => {
                self.advance();

                let name = val.to_lowercase();

                if name == "pi" {
                    Expr::Number(Complex::real(std::f64::consts::PI))
                } else if name == "phi" {
                    Expr::Number(Complex::real(std::f64::consts::GOLDEN_RATIO))
                } else if name == "e" {
                    Expr::Number(Complex::real(std::f64::consts::E))
                } else if name == "i" {
                    Expr::Number(Complex::i())
                } else if name == "ans" {
                    Expr::Number(self.ans)
                } else {
                    let op = match name.as_str() {
                        "sqrt" => Some(UnaryOp::Sqrt),
                        "sin" => Some(UnaryOp::Sin),
                        "cos" => Some(UnaryOp::Cos),
                        "tan" => Some(UnaryOp::Tan),
                        "asin" => Some(UnaryOp::Asin),
                        "acos" => Some(UnaryOp::Acos),
                        "atan" => Some(UnaryOp::Atan),
                        "abs" => Some(UnaryOp::Abs),
                        "re" => Some(UnaryOp::Real),
                        "im" => Some(UnaryOp::Imaginary),
                        "cis" => Some(UnaryOp::Cis),
						"ln" => Some(UnaryOp::Ln),
                        _ => None,
                    };

                    if let Some(op) = op {
                        let arg = if self.current_token == Token::LParen {
                            // Explicit parentheses allow a full expression.
                            self.advance();

                            let arg = self.parse_expression(0)?;

                            if self.current_token != Token::RParen {
                                return Err(
                                    "Expected matching closing parenthesis ')'".to_string()
                                );
                            }

                            self.advance();
                            arg
                        } else {
                            // Powers bind inside functions, but multiplication
                            // and addition do not.
                            self.parse_expression(2)?
                        };

                        Expr::Unary {
                            value: Box::new(arg),
                            op,
                        }
                    } else {
                        // Store the variable name in the AST.
                        Expr::Variable(name)
                    }
                }
            }

            Token::LParen => {
                self.advance();

                let expr = self.parse_expression(0)?;

                if self.current_token != Token::RParen {
                    return Err(
                        "Expected matching closing parenthesis ')'".to_string()
                    );
                }

                self.advance();
                expr
            }

            Token::Minus => {
                self.advance();

                // Power binds more tightly than unary minus.
                let expr = self.parse_expression(3)?;

                Expr::Unary {
                    value: Box::new(expr),
                    op: UnaryOp::Neg,
                }
            }

            Token::Plus => {
                self.advance();
                self.parse_expression(3)?
            }

            _ => {
                return Err(format!(
                    "Unexpected token: {:?}",
                    self.current_token
                ));
            }
        };

        // Infix parsing
        loop {
            // A new expression without an operator means multiplication.
            let implicit_mul = matches!(
                &self.current_token,
                Token::Number(_) | Token::Text(_) | Token::LParen
            );

            let op_precedence = if implicit_mul {
                3
            } else {
                Self::get_precedence(&self.current_token)
            };

            if op_precedence <= precedence {
                break;
            }

            let op = if implicit_mul {
                BinaryOp::Mul
            } else {
                match self.current_token.clone() {
                    Token::Plus => BinaryOp::Add,
                    Token::Minus => BinaryOp::Sub,
                    Token::Star => BinaryOp::Mul,
                    Token::Slash => BinaryOp::Div,
                    Token::UpArrow => BinaryOp::Pow,
                    _ => break,
                }
            };

            // Don't consume the next expression for implicit multiplication.
            if !implicit_mul {
                self.advance();
            }

            // Powers are right-associative.
            let right_precedence = if op == BinaryOp::Pow {
                op_precedence - 1
            } else {
                op_precedence
            };

            let right = self.parse_expression(right_precedence)?;

            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(right),
            };
        }

        Ok(left)
    }
}

// Actual calculator
fn eval_ast(
    expr: &Expr,
    variables: &HashMap<String, Complex>,
) -> Result<Complex, String> {
    match expr {
        Expr::Number(val) => Ok(*val),

        Expr::Variable(name) => variables
            .get(name)
            .copied()
            .ok_or_else(|| format!("Unknown variable: {}", name)),

        Expr::Binary { left, op, right } => {
            let l = eval_ast(left, variables)?;
            let r = eval_ast(right, variables)?;

            match op {
                BinaryOp::Add => Ok(l.add(r)),
                BinaryOp::Sub => Ok(l.sub(r)),
                BinaryOp::Mul => Ok(l.mul(r)),
                BinaryOp::Div => l.div(r),
                BinaryOp::Pow => l.pow(r),
            }
        }

        Expr::Unary { value, op } => {
            let v = eval_ast(value, variables)?;

            match op {
                UnaryOp::Neg => Ok(v.neg()),
                UnaryOp::Sqrt => Ok(v.sqrt()),
                UnaryOp::Sin => Ok(v.sin()),
                UnaryOp::Cos => Ok(v.cos()),
                UnaryOp::Tan => v.tan(),
                UnaryOp::Asin => Ok(v.asin()),
                UnaryOp::Acos => Ok(v.acos()),
                UnaryOp::Atan => Ok(v.atan()),
                UnaryOp::Abs => Ok(Complex::real(v.abs())),
                UnaryOp::Real => Ok(Complex::real(v.re)),
                UnaryOp::Imaginary => Ok(Complex::imaginary(v.im)),
                UnaryOp::Cis => {
                    Ok(v.cos().add(v.sin().mul(Complex::i())))
                },
				UnaryOp::Ln => {
					Ok(v.ln())
				}
            }
        }
    }
}

// This could be updated later to not allow some variable names along with read_text
fn is_valid_variable_name(name: &str) -> bool {
    let mut chars = name.chars();

    match chars.next() {
        Some(c) if c.is_alphabetic() => {}
        _ => return false,
    }

    chars.all(|c| c.is_alphanumeric() || c == '_')
}

// Evaluates an expression or assigns a variable.
fn evaluate(
    text: &str,
    ans: Complex,
    variables: &mut HashMap<String, Complex>,
) -> Result<Complex, String> {
    if let Some((name, expression)) = text.split_once('=') {
        if expression.contains('=') {
            return Err("Only one '=' is allowed in an assignment".to_string());
        }

        let name = name.trim().to_lowercase();
        let expression = expression.trim();

        if !is_valid_variable_name(&name) {
            return Err("Invalid variable name".to_string());
        }

        if expression.is_empty() {
            return Err("Expected an expression after '='".to_string());
        }

        // Prevent overwriting built-in constants and functions.
		// This code only shows the user an error; as built ins are parsed first they can never be reassigned
        if matches!(
            name.as_str(),
            "pi" | "phi" | "e" | "i" | "ans"
                | "sqrt" | "sin" | "cos" | "tan"
                | "asin" | "acos" | "atan" | "abs"
                | "re" | "im" | "cis" | "ln"
        ) {
            return Err(format!("'{}' is a reserved name", name));
        }

        // Evaluate before storing, so failed assignments don't overwrite
        // an existing variable.
        let value = evaluate_expression(expression, ans, variables)?;
        variables.insert(name, value);

        return Ok(value);
    }

    evaluate_expression(text, ans, variables)
}

fn evaluate_expression(
    text: &str,
    ans: Complex,
    variables: &HashMap<String, Complex>,
) -> Result<Complex, String> {
    let lexer = Lexer::new(text);
    let mut parser = Parser::new(lexer, ans);

    let ast = parser.parse_expression(0)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "debug" {
        println!("AST:\n{:#?}", ast);
    }

    if parser.current_token != Token::EOF {
        return match parser.current_token {
            Token::Invalid(c) => {
                Err(format!("Unrecognised character: '{}'", c))
            }
            _ => Err(format!(
                "Unexpected token after expression: {:?}",
                parser.current_token
            )),
        };
    }

    eval_ast(&ast, variables)
}