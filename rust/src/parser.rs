use std::collections::HashMap;

enum AST {
    Equals(Box<AST>, Box<AST>),
    Constant(f64),
    Variable(String),
    Addition(Box<AST>, Box<AST>),
    Subtraction(Box<AST>, Box<AST>),
    Multiplication(Box<AST>, Box<AST>),
    Division(Box<AST>, Box<AST>),
}

enum Token {
    Number(f64),
    Variable(String),
    Operation(char),
}

pub struct Equation {
    equation: AST,
}

type Env = HashMap<String, f64>;

impl AST {
    fn eval(&self, env: &Env) -> f64 {
        match self {
            AST::Constant(n) => *n,

            AST::Variable(name) => *env
                .get(name)
                .unwrap_or_else(|| panic!("Undefined variable: {name}")),

            AST::Addition(a, b) => a.eval(env) + b.eval(env),
            AST::Subtraction(a, b) => a.eval(env) - b.eval(env),
            AST::Multiplication(a, b) => a.eval(env) * b.eval(env),
            AST::Division(a, b) => a.eval(env) / b.eval(env),

            AST::Equals(_, _) => {
                panic!("Cannot directly evaluate an equation. Use solve or compare instead.")
            }
        }
    }
}

impl Equation {
    pub fn eval_left_right(&self, env: &Env) -> (f64, f64) {
        match &self.equation {
            AST::Equals(left, right) => (left.eval(env), right.eval(env)),
            _ => panic!("Invalid equation structure"),
        }
    }

    pub fn is_true(&self, env: &Env) -> bool {
        let (l, r) = self.eval_left_right(env);
        (l - r).abs() < 1e-9
    }

    fn residual(&self, env: &Env) -> f64 {
        let (l, r) = self.eval_left_right(env);
        l - r
    }

    pub fn solve(&self, knowns: &Env, var: &str, min: f64, max: f64) -> Option<f64> {
        let mut lo = min;
        let mut hi = max;
        let mut env = knowns.clone();

        let f_lo = {
            env.insert(var.to_string(), lo);
            self.residual(&env)
        };
        let f_hi = {
            env.insert(var.to_string(), hi);
            self.residual(&env)
        };

        if f_lo.abs() < 1e-9 {
            return Some(lo);
        }
        if f_hi.abs() < 1e-9 {
            return Some(hi);
        }

        if f_lo * f_hi > 0.0 {
            return None;
        }

        for _ in 0..64 {
            let mid = (lo + hi) / 2.0;
            env.insert(var.to_string(), mid);
            let f_mid = self.residual(&env);

            if f_mid.abs() < 1e-9 {
                return Some(mid);
            }

            if f_lo * f_mid < 0.0 {
                hi = mid;
            } else {
                lo = mid;
            }
        }

        Some((lo + hi) / 2.0)
    }

    pub fn new(equation: String) -> Self {
        let parts: Vec<String> = equation.split('=').map(String::from).collect();
        assert_eq!(parts.len(), 2, "Equation must contain exactly one '='");
        let left = Equation::parse(&parts[0]);
        let right = Equation::parse(&parts[1]);
        Self {
            equation: AST::Equals(Box::new(left), Box::new(right)),
        }
    }

    fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();

        while let Some(&c) = chars.peek() {
            match c {
                // Skip whitespace
                ' ' | '\t' => {
                    chars.next();
                }

                // Numbers: consume digits and at most one '.'
                '0'..='9' | '.' => {
                    let mut num = String::new();
                    while let Some(&d) = chars.peek() {
                        if d.is_ascii_digit() || d == '.' {
                            num.push(d);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(num.parse().expect("Invalid number literal")));
                }

                // Variables / identifiers: one or more alphanumeric chars
                'a'..='z' | 'A'..='Z' => {
                    let mut name = String::new();
                    while let Some(&a) = chars.peek() {
                        if a.is_alphanumeric() || a == '_' {
                            name.push(a);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Variable(name));
                }

                // Operators
                '+' | '-' | '*' | '/' => {
                    tokens.push(Token::Operation(c));
                    chars.next();
                }

                other => panic!("Unexpected character: '{other}'"),
            }
        }

        tokens
    }

    fn parse(input: &str) -> AST {
        let tokens = Equation::tokenize(input);
        let mut pos = 0;
        let ast = Equation::parse_expr(&tokens, &mut pos);
        assert_eq!(pos, tokens.len(), "Unexpected tokens after expression");
        ast
    }

    // -----------------------------------------------------------------------
    // Recursive descent — three precedence levels
    //
    //   parse_expr   →  parse_term   (('+' | '-') parse_term)*
    //   parse_term   →  parse_factor (('*' | '/') parse_factor)*
    //   parse_factor →  Number | Variable
    // -----------------------------------------------------------------------

    /// Handles `+` and `-`  (lowest precedence)
    fn parse_expr(tokens: &[Token], pos: &mut usize) -> AST {
        let mut left = Equation::parse_term(tokens, pos);

        while let Some(Token::Operation(op @ ('+' | '-'))) = tokens.get(*pos) {
            let op = *op;
            *pos += 1;
            let right = Equation::parse_term(tokens, pos);
            left = match op {
                '+' => AST::Addition(Box::new(left), Box::new(right)),
                '-' => AST::Subtraction(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }

        left
    }

    /// Handles `*` and `/`  (higher precedence)
    fn parse_term(tokens: &[Token], pos: &mut usize) -> AST {
        let mut left = Equation::parse_factor(tokens, pos);

        while let Some(Token::Operation(op @ ('*' | '/'))) = tokens.get(*pos) {
            let op = *op;
            *pos += 1;
            let right = Equation::parse_factor(tokens, pos);
            left = match op {
                '*' => AST::Multiplication(Box::new(left), Box::new(right)),
                '/' => AST::Division(Box::new(left), Box::new(right)),
                _ => unreachable!(),
            };
        }

        left
    }

    /// Handles atomic values — numbers and variables  (highest precedence)
    fn parse_factor(tokens: &[Token], pos: &mut usize) -> AST {
        match tokens.get(*pos) {
            Some(Token::Number(n)) => {
                let node = AST::Constant(*n);
                *pos += 1;
                node
            }
            Some(Token::Variable(v)) => {
                let node = AST::Variable(v.clone());
                *pos += 1;
                node
            }
            Some(Token::Operation(op)) => {
                panic!("Expected a value, found operator '{op}'")
            }
            None => panic!("Unexpected end of expression"),
        }
    }
}
