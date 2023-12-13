use std::io::{self, Write};
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Clone)]
#[derive(PartialEq)]
#[derive(Debug)]
enum Token{
    Number(isize),
    LParen,
    RParen,
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
enum Expr {
    Number(isize),
    BinaryOp(Box<Expr>, Token, Box<Expr>),
}

struct Parser{
    tokens: Peekable<IntoIter<Token>>,
    error: Option<String>,
}

impl Parser{
    fn new(input: &String) -> Self{
        let mut error = None;
        let result = match Parser::tokenize(&input) {
            Ok(value) => value,
            Err(err) => {
                error = Some(err);
                Vec::new()
            }
        };
        Parser{
            tokens: result.into_iter().peekable(),
            error: error,
        }
    }
    fn tokenize(input: &String) -> Result<Vec<Token>, String>{
        let mut result: Vec<Token> = Vec::new();
        let mut itr = input.chars().peekable();
        
        while let Some(char) = itr.next() {
            match char {
                '(' => result.push(Token::LParen),
                ')' => result.push(Token::RParen),
                '+' => result.push(Token::Add),
                '-' => result.push(Token::Sub),
                '*' => result.push(Token::Mul),
                '/' => result.push(Token::Div),
                '0'..='9' => {
                    let mut number = char.to_string();
                    while let Some(&next_char) = itr.peek() {
                        match next_char{
                            '0'..='9' => {
                                itr.next();
                                number.push(next_char)
                            }
                            _ => break
                        }
                    }
                    result.push(Token::Number(number.parse::<isize>().unwrap()))
                }
                ' ' | '\t' => (),
                _ => return Err(
                    String::from("Unknown token(s): Failed to tokenize.")
                )
            }
        }
        Ok(result)
    }
    fn peek(&mut self) -> Option<Token> {
        self.tokens.peek().cloned()
    }
    fn consume(&mut self) -> Option<Token> {
        self.tokens.next()
    }
    fn expect(&mut self, expected: Token) -> Option<Token> {
        let result = expected.clone();
        if self.peek() == Some(expected){
            self.consume();
            Some(result)
        }else {
            self.error = Some(format!("Missing token: {:?}", result));
            None
        }
    }
    fn parse_expr(&mut self) -> Expr {
        let mut left = self.parse_term();

        while let Some(token) = self.peek() {
            match token {
                Token::Add | Token::Sub => {
                    self.consume();
                    let right = self.parse_term();
                    left = Expr::BinaryOp(Box::new(left), token, Box::new(right));
                }
                _ => break,
            }
        }
    
        left
    }
    fn parse_term(&mut self) -> Expr {
        let mut left = self.parse_factor();

        while let Some(token) = self.peek() {
            match token {
                Token::Mul | Token::Div => {
                    self.consume();
                    let right = self.parse_factor();
                    left = Expr::BinaryOp(Box::new(left), token, Box::new(right));
                }
                _ => break,
            }
        }
    
        left
    }
    fn parse_factor(&mut self) -> Expr {
        match self.peek().unwrap() {
            Token::Number(n) => {
                self.consume();
                Expr::Number(n)
            }
            Token::LParen => {
                self.consume();
                let expr = self.parse_expr();
                self.expect(Token::RParen);
                expr
            }
            _ => unreachable!()
        }
    }
    fn solve(expr: Expr) -> isize {
        match expr {
            Expr::Number(n) => n,
            Expr::BinaryOp(left, token, right) => {
                let left = Parser::solve(*left);
                let right = Parser::solve(*right);

                match token {
                    Token::Add => left + right,
                    Token::Sub => left - right,
                    Token::Mul => left * right,
                    Token::Div => left / right,
                    _ => unreachable!()
                }
            }
        }
    }
    fn eval(&mut self) -> Result<isize, String> {
        match &self.error {
            Some(err) => Err(err.to_string()),
            None =>  Ok(Parser::solve(self.parse_expr()))
        }
    }
}

fn run(){
    print!(">> ");
    let _ = io::stdout().flush();

    let mut expr = String::new();
    let _ = io::stdin().read_line(&mut expr);

    println!("{}\n", Parser::new(&expr.trim().to_string()).eval().unwrap());

    run();
}


fn main() {
    let expressions = [
    "(2 * 2) + (5)",
    "3 + 4 - 2",
    "10 / 2 * 3",
    "(7 - 3) * 2",
    "6 / (2 + 1)",
    "(4 * 2) + (6 / 3)",
    "5 * (3 + 2) - 8",
    "(9 / 3) + (2 * 2)",
    "(5 + 2) * (8 - 3)",
    "12 / 4 + (6 - 2)",
    "4 - 2 + 7 * 3",
    "8 / 2 - 1 + 5",
    "(3 * 2) / (1 + 1)",
    "(5 + 3) * (2 - 1)",
    "9 * 2 / (6 - 3)",
    "10 / (2 * 5) + 1",
    "7 * (4 - 2) + 6 / 3",
    "(6 + 2) * (9 - 3) / 2",
    "(4 * 3) + 2 - 5 / 1",
    "8 / 2 * (4 + 1)"
    ];
    let solutions = [9,5,15,8,2,10,17,7,35,7,23,8,3,8,6,2,16,24,9,20];

    let mut index = 0;

    for expr in expressions{
        println!("Assert {} == {}", &expr, solutions[index]);
        assert_eq!(Parser::new(&expr.to_string()).eval().unwrap(), solutions[index]);
        index += 1;
    }
    println!("Wowz, it works");

    run();

}
