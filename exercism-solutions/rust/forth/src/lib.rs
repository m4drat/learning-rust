pub mod lexer;
pub mod stack;
pub mod symbols;

use lexer::Lexer;
use symbols::Handler;
use symbols::Scope;
use symbols::SymbolTable;

use crate::lexer::Token;
use crate::stack::Stack;

use std::rc::Rc;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

/// Forth interpreter.
#[derive(Default)]
pub struct Forth {
    /// Tokens to be processed by the interpreter.
    tokens: Vec<Token>,
    /// Stack for the interpreter.
    stack: Stack<Value>,
    /// Mapping of all the symbols in the program to their handlers with the respective scopes.
    symbol_table: SymbolTable,
    /// Current word being processed.
    current_word: Option<Token>,
    /// Tokens for the current word being processed.
    word_tokens: Vec<Token>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

/// Macro to define a binary operation handler.
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

/// Macro to register a builtin handler in the symbol table.
macro_rules! register_builtin {
    ($symbol_table:expr, $token0:expr, $handler0:expr) => {
        $symbol_table.register(
            $token0,
            Handler {
                builtin: Some(Rc::new($handler0)),
                custom: None,
            },
        )
    };
}

impl Forth {
    pub fn new() -> Forth {
        // Register all the builtin handlers.
        let mut symbol_table = SymbolTable::default();
        register_builtin!(symbol_table, Token::Add, Forth::add);
        register_builtin!(symbol_table, Token::Sub, Forth::sub);
        register_builtin!(symbol_table, Token::Mul, Forth::mul);
        register_builtin!(symbol_table, Token::Div, Forth::div);
        register_builtin!(symbol_table, Token::Dup, Forth::dup);
        register_builtin!(symbol_table, Token::Drop, Forth::drop);
        register_builtin!(symbol_table, Token::Swap, Forth::swap);
        register_builtin!(symbol_table, Token::Over, Forth::over);

        Self {
            tokens: vec![],
            stack: Stack::new(),
            symbol_table,
            current_word: None,
            word_tokens: vec![],
        }
    }

    /// Returns the top value from the data stack.
    ///
    /// # Returns
    ///
    /// The top value from the data stack. If the stack is empty - returns a `StackUnderflow` error.
    fn top_value(&mut self) -> std::result::Result<Value, Error> {
        let top = self.stack.top().ok_or(Error::StackUnderflow);
        top.copied()
    }

    /// Pops the top value from the data stack.
    ///
    /// # Returns
    ///
    /// The top value from the data stack. If the stack is empty - returns a `StackUnderflow` error.
    fn pop_value(&mut self) -> std::result::Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    /// Pushes a value to the data stack.
    ///
    /// # Arguments
    ///
    /// * `val` - Value to push to the data stack.
    fn push_value(&mut self, val: Value) {
        self.stack.push(val);
    }

    /// Returns the current data stack.
    ///
    /// # Returns
    ///
    /// The current data stack.
    pub fn stack(&self) -> &[Value] {
        &self.stack.vec
    }

    // Define builtin handlers for some of the binary operations.
    define_binary_op!(add, +);
    define_binary_op!(sub, -);
    define_binary_op!(mul, *);

    /// Handler for the division operation.
    ///
    /// # Returns
    ///
    /// Result of the division operation. If the stack is empty - returns a `StackUnderflow` error.
    /// If the top value on the stack is 0 - returns a `DivisionByZero` error.
    fn div(&mut self) -> Result {
        let a = self.pop_value()?;
        let b = self.pop_value()?;

        if a == 0 {
            return Err(Error::DivisionByZero);
        }

        let result = b / a;
        self.push_value(result);

        Ok(())
    }

    /// Handler for the dup operation. Duplicates the top value on the stack.
    fn dup(&mut self) -> Result {
        let top = self.top_value()?;
        self.push_value(top);
        Ok(())
    }

    /// Handler for the drop operation. Drops the top value from the stack.
    fn drop(&mut self) -> Result {
        self.stack.pop().ok_or(Error::StackUnderflow)?;
        Ok(())
    }

    /// Handler for the swap operation. Swaps the top two values on the stack.
    fn swap(&mut self) -> Result {
        let a = self.pop_value()?;
        let b = self.pop_value()?;

        self.push_value(a);
        self.push_value(b);

        Ok(())
    }

    /// Handler for the over operation. Duplicates the second value on the stack.
    fn over(&mut self) -> Result {
        let a = self.pop_value()?;
        let b = self.pop_value()?;

        self.push_value(b);
        self.push_value(a);
        self.push_value(b);

        Ok(())
    }

    /// Handler for the colon operation. Starts a new word definition.
    fn start_word_definition(&mut self) {
        self.current_word = Some(Token::default());
    }

    /// Handler for the semi operation. Ends a word definition.
    fn end_word_definition(&mut self) {
        if let Some(current_word) = &self.current_word {
            self.symbol_table.register(
                current_word.clone(),
                Handler {
                    builtin: None,
                    custom: Some(self.word_tokens.to_vec()),
                },
            );
        }

        self.current_word = None;
        self.word_tokens = vec![];
    }

    /// If we've started processing a word definition - returns whether we've already set the word name.
    fn is_word_name_set(&self) -> bool {
        if let Some(word_name) = &self.current_word {
            return *word_name != Token::Invalid;
        }

        false
    }

    /// Returns whether we're currently processing a word definition.
    fn is_word_definition(&self) -> bool {
        self.current_word.is_some()
    }

    /// Evaluates the given tokens with the given scope.
    /// This is a recursive function that can be called multiple times for nested word definitions.
    ///
    /// # Arguments
    ///
    /// * `tokens` - Tokens to evaluate.
    /// * `current_scope` - Current scope to evaluate the tokens with.
    ///
    /// # Returns
    ///
    /// Result of the evaluation.
    fn eval_with_scope(&mut self, tokens: Vec<Token>, mut current_scope: Scope) -> Result {
        for token in tokens.iter() {
            println!("Token: {:?}", token);

            // If we're processing a word definition - we need to handle it differently.
            if self.is_word_definition() {
                // If we haven't set the word name yet - we need to do it now. E.g., last token
                // was `:` and we're expecting a token (potentially word) to be the name of
                // the word we're defining.
                if !self.is_word_name_set() {
                    // If we didn't get a correct word name - we have an error.
                    match token {
                        Token::Num(_) | Token::Colon | Token::Semi => {
                            println!("Invalid word name: {:?}", token);
                            return Err(Error::InvalidWord);
                        }
                        _ => {}
                    };

                    // Set the word name and continue to the next token.
                    self.current_word = Some(token.clone());
                    continue;
                }

                // Exit word definition mode if we got a semicolon.
                if *token == Token::Semi {
                    // Register the word in the symbol table, and do some cleanup.
                    self.end_word_definition();
                    // After new word is added - update current scope to include it.
                    current_scope = self.symbol_table.snapshot_scope();
                } else {
                    // Otherwise - add the token to the list of tokens for the current word.
                    self.word_tokens.push(token.clone());
                }

                // We're done processing a token from inside the word definion - continue to the next one.
                continue;
            }

            // If we're not working on a word definition - try to handle the token as a value, or as
            // a start of a word definition.
            match token {
                Token::Num(num) => self.push_value(*num),
                Token::Colon => self.start_word_definition(),
                _ => {}
            };

            // Finally, if we're neither in the word definition mode, nor we're processing a value -
            // we need to handle the token as a word (or as an operation).
            if let Some(scoped_handler) = current_scope.get(token) {
                // If we got a custom handler for the token - evaluate it with the scope it was defined in.
                if let Some(custom) = &scoped_handler.handler.custom {
                    self.eval_with_scope(custom.to_vec(), scoped_handler.scope.clone())?;
                } else if let Some(builtin) = &scoped_handler.handler.builtin {
                    // If we got a builtin handler for the token - call it.
                    builtin(self)?;
                } else {
                    panic!("No handler for the token!");
                }
            } else if std::mem::discriminant(token)
                == std::mem::discriminant(&Token::Word("".to_string()))
            {
                // Finally, if we got a word that's not in the symbol table - we have an error.
                println!("Unknown word: {:?}", token);
                return Err(Error::UnknownWord);
            }
        }

        // If we're still in the word definition mode - we have an error.
        if self.is_word_definition() {
            return Err(Error::InvalidWord);
        }

        Ok(())
    }

    /// Evaluates the given input.
    /// This is a wrapper around `eval_with_scope` that sets the initial scope to the global scope.
    ///
    /// # Arguments
    ///
    /// * `input` - Input to evaluate.
    ///
    /// # Returns
    ///
    /// Result of the evaluation.
    pub fn eval(&mut self, input: &str) -> Result {
        let lexer = Lexer::new();
        self.tokens = lexer.parse(input);
        self.eval_with_scope(self.tokens.to_vec(), self.symbol_table.snapshot_scope())
    }
}
