use std::{collections::HashMap, rc::Rc};

use crate::{lexer::Token, Forth, Result};

/// Type alias for a builtin handler.
pub type Builtin = Rc<dyn Fn(&mut Forth) -> Result>;

/// A snapshot of the symbol table at a given point in time.
#[derive(Default, Clone)]
pub struct Scope {
    /// Map of all the words in the symbol table and their current visibility.
    handlers: HashMap<Token, Rc<ScopedHandler>>,
}

impl Scope {
    /// Returns the current visibility of the word in the symbol table.
    ///
    /// # Arguments
    ///
    /// * `token` - Token to check the visibility for.
    ///
    /// # Returns
    ///
    /// The current visibility of the word in the symbol table.
    pub fn get(&self, token: &Token) -> Option<Rc<ScopedHandler>> {
        self.handlers.get(token).cloned()
    }
}

/// Handler for a symbol in the symbol table.
pub struct Handler {
    /// Builtin handler for the symbol, if any.
    pub builtin: Option<Builtin>,
    /// Custom handler for the symbol, if any.
    pub custom: Option<Vec<Token>>,
}

/// Handler for a symbol in the symbol table. Symbol in this context is a Token. It can be either
/// a builtin or a custom word, but not special tokens like `:`, `;`, etc.
pub struct ScopedHandler {
    /// Either a builtin or a custom handler for the symbol.
    pub handler: Handler,
    /// Scope in which the symbol was defined.
    pub scope: Scope,
}

/// Symbol table for the Forth interpreter.
#[derive(Default)]
pub struct SymbolTable {
    /// Map of all the Symbols in the program to their handlers with the respective scopes.
    table: HashMap<Token, Vec<Rc<ScopedHandler>>>,
}

/// Symbol table for the Forth interpreter.
/// Everytime we define a new word - we need to keep track of the current scope visibility
/// for all the words in the symbol table. This is needed to support nested word definitions.
impl SymbolTable {
    /// Creates a snapshot of the current symbol table.
    ///
    /// # Returns
    ///
    /// A snapshot of the current symbol table.
    pub fn snapshot_scope(&self) -> Scope {
        Scope {
            handlers: self
                .table
                .iter()
                .map(|(k, v)| (k.clone(), v.last().unwrap().clone()))
                .collect(),
        }
    }

    /// Registers a new symbol in the symbol table, while also capturing the current scope.
    ///
    /// # Arguments
    ///
    /// * `token` - Token to register in the symbol table.
    /// * `handler` - Handler for the token.
    pub fn register(&mut self, token: Token, handler: Handler) {
        let scope = self.snapshot_scope();
        self.table
            .entry(token)
            .or_insert_with(Vec::new)
            .push(Rc::new(ScopedHandler { handler, scope }));
    }
}
