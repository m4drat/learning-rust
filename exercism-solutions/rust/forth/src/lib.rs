pub mod lexer;
pub mod stack;

use lexer::Lexer;

use crate::lexer::Token;
use crate::stack::Stack;

use core::panic;
use std::collections::HashMap;
use std::rc::Rc;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;
pub type Builtin = Rc<dyn Fn(&mut Forth) -> Result>;

#[derive(Clone)]
struct Handler {
    builtin: Option<Builtin>,
    custom: Option<Vec<Token>>,
}

pub struct SymbolTable {
    table: HashMap<Token, Vec<Handler>>,
}

// Everytime we define a new word - we need to keep track of the current scope visibility
// for all the words in the symbol table. This is needed to support nested word definitions.
impl SymbolTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }
}

pub struct Forth {
    tokens: Vec<Token>,
    stack: Stack<Value>,
    symbol_table: HashMap<Token, Vec<Handler>>,
    current_word: Option<Token>,
    word_tokens: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

macro_rules! define_binary_op {
    ($function_name:ident, $symbol:tt) => {
        fn $function_name(&mut self) -> Result {
            let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
            let b = self.stack.pop().ok_or(Error::StackUnderflow)?;
            let result = b $symbol a;
            self.stack.push(result);

            Ok(())
        }
    };
}

macro_rules! create_builtins_table {
    (($token0:expr, $handler0:expr) $(, ($token:expr, $handler:expr) )*) => {
        HashMap::from([
            ($token0,
                vec![Handler {
                    builtin: Some(Rc::new($handler0)),
                    custom: None,
                }]
            ),
            $((
                $token,
                vec![Handler {
                    builtin: Some(Rc::new($handler)),
                    custom: None,
                }],
            )),*
        ])
    };
}

impl Forth {
    pub fn new() -> Forth {
        Self {
            tokens: vec![],
            stack: Stack::new(),
            symbol_table: create_builtins_table!(
                (Token::Add, Forth::add),
                (Token::Sub, Forth::sub),
                (Token::Mul, Forth::mul),
                (Token::Div, Forth::div),
                (Token::Dup, Forth::dup),
                (Token::Drop, Forth::drop),
                (Token::Swap, Forth::swap),
                (Token::Over, Forth::over)
            ),
            current_word: None,
            word_tokens: vec![],
        }
    }

    fn top_value(&mut self) -> std::result::Result<Value, Error> {
        let top = self.stack.top().ok_or(Error::StackUnderflow);
        top.copied()
    }

    fn pop_value(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn push_value(&mut self, val: Value) {
        self.stack.push(val);
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack.vec
    }

    // Opcodes handlers
    define_binary_op!(add, +);
    define_binary_op!(sub, -);
    define_binary_op!(mul, *);

    fn div(&mut self) -> Result {
        let a = self.stack.pop().ok_or(Error::StackUnderflow)?;
        let b = self.stack.pop().ok_or(Error::StackUnderflow)?;

        if a == 0 {
            return Err(Error::DivisionByZero);
        }

        let result = b / a;
        self.stack.push(result);

        Ok(())
    }

    fn dup(&mut self) -> Result {
        let top = self.top_value()?;
        self.push_value(top);
        Ok(())
    }

    fn drop(&mut self) -> Result {
        self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok(())
    }

    fn swap(&mut self) -> Result {
        let a = self.pop_value()?;
        let b = self.pop_value()?;

        self.push_value(a);
        self.push_value(b);

        Ok(())
    }

    fn over(&mut self) -> Result {
        let a = self.pop_value()?;
        let b = self.pop_value()?;

        self.push_value(b);
        self.push_value(a);
        self.push_value(b);

        Ok(())
    }

    fn start_word_definition(&mut self) {
        self.current_word = Some(Token::default());
    }

    fn end_word_definition(&mut self) {
        if let Some(current_word) = &self.current_word {
            // If we already have this word in the table - append some more code to it
            if let Some(symbol) = self.symbol_table.get_mut(current_word) {
                println!(
                    "Appending to word: {:?}, with code: {:?}",
                    current_word, self.word_tokens
                );

                symbol.push(Handler {
                    builtin: None,
                    custom: Some(self.word_tokens.to_vec()),
                });
            } else {
                println!(
                    "Adding new word: {:?}, with code: {:?}",
                    current_word, self.word_tokens
                );

                self.symbol_table.insert(
                    current_word.clone(),
                    vec![Handler {
                        builtin: None,
                        custom: Some(self.word_tokens.to_vec()),
                    }],
                );
            }
        }

        self.current_word = None;
        self.word_tokens = vec![];
    }

    fn is_word_name_set(&self) -> bool {
        if let Some(word_name) = &self.current_word {
            return *word_name != Token::Invalid;
        }

        false
    }

    fn is_word_definition(&self) -> bool {
        self.current_word.is_some()
    }

    fn eval_loop(&mut self, tokens: Vec<Token>, current_scope: Option<usize>) -> Result {
        for token in tokens.iter() {
            println!("Token: {:?}", token);

            if self.is_word_definition() {
                // Check whether we're processing a word definition. If so -
                if !self.is_word_name_set() {
                    match token {
                        Token::Num(_) | Token::Colon | Token::Semi => {
                            println!("Invalid word name: {:?}", token);
                            return Err(Error::InvalidWord);
                        }
                        _ => {}
                    };

                    self.current_word = Some(token.clone());
                    continue;
                }

                if *token == Token::Semi {
                    self.end_word_definition();
                } else {
                    self.word_tokens.push(token.clone());
                }

                continue;
            }

            match token {
                Token::Num(num) => self.push_value(*num),
                Token::Colon => self.start_word_definition(),
                _ => {}
            };

            // Try to handle builtins / custom words
            if let Some(handlers_chain) = self.symbol_table.get(token).cloned() {
                let new_scope;
                let handler = match current_scope {
                    Some(scope_idx) => {
                        let handler = handlers_chain.get(scope_idx).unwrap();
                        new_scope = Some(scope_idx.saturating_sub(1));
                        handler
                    }
                    None => {
                        new_scope = Some(handlers_chain.len().saturating_sub(2));
                        handlers_chain.last().unwrap()
                    }
                };

                if let Some(custom) = &handler.custom {
                    self.eval_loop(custom.to_vec(), new_scope)?;
                } else if let Some(builtin) = &handler.builtin {
                    builtin(self)?;
                } else {
                    panic!("No handler for the token!");
                }
            } else if std::mem::discriminant(token)
                == std::mem::discriminant(&Token::Word("".to_string()))
            {
                println!("Unknown word: {:?}", token);
                return Err(Error::UnknownWord);
            }
        }

        // If we're still in the word definition mode - we have an error
        if self.is_word_definition() {
            return Err(Error::InvalidWord);
        }

        Ok(())
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let lexer = Lexer::new();
        self.tokens = lexer.parse(input);
        self.eval_loop(self.tokens.to_vec(), None)
    }
}

impl Default for Forth {
    fn default() -> Self {
        Self::new()
    }
}
