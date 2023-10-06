#![allow(dead_code)]

fn is_operator(c: char) -> bool {
    match c {
        '+' => true,
        '-' => true,
        '*' => true,
        '/' => true,
        '^' => true,
        _ => false
    }
}

fn is_decimal(c: char) -> bool {
    match c {
        ',' => true,
        '.' => true,
        _ => false
    }
}

fn is_parenthesis(c: char) -> bool {
    match c {
        '(' => true,
        ')' => true,
        _ => false
    }
}

#[derive(Debug)]
#[derive(Clone)]
enum TokenType {
    Number,     // 1, 2, 3... exc.
    BinaryOp,   // + - * / exc.
    UnaryOp,   // ^ - exc.
    Function,   // sqrt, log, exc.
    Parenthesis,
    Null
}

#[derive(Debug)]
#[derive(Clone)]
struct Token {
    typ: TokenType,
    lex: String  // Data as a string
}

impl Token {
    fn copy(&self) -> Token {
        Token {
            typ: self.typ.clone(),
            lex: self.lex.clone(),
        }
    }
}

#[derive(Debug)]
enum ParseNode {
    Binary (char, Box<ParseNode>, Box<ParseNode>),
    Unary (char, Box<ParseNode>),
    Number(f64),
    Null
}

impl ParseNode {
    fn evaluate(&mut self) -> f64 {
        match self {
            ParseNode::Binary(operator, left_operand, right_operand) => {
                match operator {
                    '+' => return left_operand.evaluate() + right_operand.evaluate(),
                    '-' => return left_operand.evaluate() - right_operand.evaluate(),
                    '*' => return left_operand.evaluate() * right_operand.evaluate(),
                    '/' => return left_operand.evaluate() / right_operand.evaluate(),
                    '^' => return f64::powf(left_operand.evaluate(), right_operand.evaluate()),
                    _ => return 0.0
                }
            }
            ParseNode::Unary(operator, left_operand) => {
                match operator {
                    '-' => return -left_operand.evaluate(),
                    _ => return 0.0
                }
            }
            ParseNode::Number(num) => {
                *num
            }
            _ => return 0.0
        }
    }
}

#[derive(Debug)]
enum ParserError {
    SyntaxError(String),
    MathError(String)
}

#[derive(Debug)]
struct Parser {
    curr_token: usize,
    all_tokens: Vec<Token>
}

impl Parser {
    fn new() -> Self {
        Self {
            curr_token: 0,
            all_tokens: Vec::new(),
        }
    }

    fn create_ast(&mut self, tokens: &Vec<Token>) -> Result<Box<ParseNode>, ParserError> {
        self.all_tokens = tokens.clone();
        self.parse_expression(tokens)
    }

    fn parse_factor(&mut self, token: &Token) -> Result<Box<ParseNode>, ParserError> {
        // Implement your factor parsing logic here.
        // If the token isn't what's expected, return a custom error like this:
        // Err(ParserError::CustomError("Expected a different token type".to_string()))
    }

    fn parse_term(&mut self, tokens: &[Token]) -> Result<Box<ParseNode>, ParserError> {
        // Implement your term parsing logic here.
        // If the token isn't what's expected, return a custom error like this:
        // Err(ParserError::CustomError("Expected a different token type".to_string()))
    }

    fn parse_expression(&mut self, tokens: &[Token]) -> Result<Box<ParseNode>, ParserError> {
        // Implement your expression parsing logic here.
        // If the token isn't what's expected, return a custom error like this:
        // Err(ParserError::CustomError("Expected a different token type".to_string()))
    }

    fn next_token(&self) -> Token {
        if self.curr_token < self.all_tokens.len() - 1 {
            self.all_tokens[self.curr_token + 1].clone()
        } else {
            Token {
                typ: TokenType::Null,
                lex: String::from("0"),
            }
        }
    }
}

fn lexer(data: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut chars = data.chars().peekable();
    let mut last_tok = Token {typ: TokenType::Number, lex: "0".to_string()};

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
        } else if c.is_numeric() {
            let mut buffer: String = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_numeric() || is_decimal(c){
                    buffer.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            let mut new_tok: Token = Token {
                typ: TokenType::Number,
                lex: buffer
            };
            last_tok = new_tok.clone();
            tokens.push(new_tok);
        } else if c.is_alphabetic() {
            let mut buffer: String = String::new();
            while let Some(&c) = chars.peek() {
                if c.is_alphabetic() {
                    buffer.push(c);
                    chars.next();
                } else {
                    break;
                }
            }
            let mut new_tok: Token = Token {
                typ: TokenType::Function,
                lex: buffer
            };
            last_tok = new_tok.clone();
            tokens.push(new_tok);
        } else if is_operator(c) {
            if c == '-' 
            && (matches!(last_tok.typ, TokenType::BinaryOp)
            || (matches!(last_tok.typ, TokenType::Parenthesis)
            && last_tok.lex == "(")) {
                let mut new_tok: Token = Token {
                    typ: TokenType::UnaryOp,
                    lex: String::from(c)
                };
                last_tok = new_tok.clone();
                tokens.push(new_tok);
            } else {
                let mut new_tok: Token = Token {
                    typ: TokenType::BinaryOp,
                    lex: String::from(c)
                };
                last_tok = new_tok.clone();
                tokens.push(new_tok);
            }
            chars.next();
        } else if is_parenthesis(c) {
            let mut new_tok: Token = Token {
                typ: TokenType::Parenthesis,
                lex: String::from(c)
            };
            last_tok = new_tok.clone();
            tokens.push(new_tok);
            chars.next();
        } else {
            chars.next();
        }
    }
    tokens
}

fn program_loop() {
    let mut parser: Parser = Parser::new();

    println!("\n----SpeedJunk™----\n");
    println!("Begin Calculation Or Type [ :q ] To Quit\n");
    
    let mut data: String = String::new();
    loop {
        data.clear();
        std::io::stdin().read_line(&mut data).unwrap();
        if data.trim() == ":q" {
            break;
        }

        let lexed: Vec<Token> = lexer(data.as_str());
        let ast: Result<Box<ParseNode>, ParserError> = parser.create_ast(&lexed);

        match ast {
            Ok(mut result) => {
                println!("{}", result.evaluate());
                continue;
            }
            Err(error) => match error {
                ParserError::SyntaxError(message) => {
                    println!("Syntax error: {}", message);
                    continue;
                }
                ParserError::MathError(message) => {
                    println!("Math error: {}", message);
                    continue;
                }
            }
        }
    }
}

fn main() {
    program_loop();
}