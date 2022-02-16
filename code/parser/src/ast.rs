use crate::{
    error::{rule_error_as_parse_error, wrong_rule_as_parse_error, Result, QKaledioscopeError},
    util::ResultIter,
    parser::*, ast_builder::TryParse
};
use std::{str::FromStr, thread::LocalKey};
use pest::{iterators::Pair, Parser, Span};

// NB: Located should not be used for structs that are atomic --- that is, that
//     wrap a single value, such as Identifier. Those structs and enums which
//     have Identifiers as items should use Located to say where they got those
//     Identifiers, however.
#[derive(Debug)]
pub struct Located<T: std::fmt::Debug>(pub T, pub Option<(usize, usize)>);
impl<T> From<T> for Located<T> where T: std::fmt::Debug {
    fn from(value: T) -> Self {
        Located(value, None)
    }
}

#[derive(Debug)]
pub struct Program(pub Vec<Located<FileElement>>);

#[derive(Debug)]
pub enum FileElement {
    Declaration(Located<Prototype>),
    // TODO: Finish adding items to Definition.
    Definition {
        prototype: Located<Prototype>,
        body: Vec<Located<Statement>>,
    },
}

#[derive(Debug)]
pub struct Prototype {
    pub name: Located<Identifier>,
    pub arguments: Vec<Located<ArgumentDeclaration>>,
    pub return_type: Option<Located<Type>>,
}

#[derive(Debug)]
pub struct ArgumentDeclaration(pub Located<Identifier>, pub Located<Type>);

#[derive(Debug)]
pub enum Type {
    Number,
    Qubit,
    Bit,
}

#[derive(Debug)]
pub struct Identifier(pub String);

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(Located<Identifier>, Located<Type>, Located<Expression>),
    Assignment(Located<Identifier>, Located<Expression>),
    Call(Located<Identifier>, Vec<Located<Expression>>),
    If {
        condition: Located<Expression>,
        true_body: Vec<Located<Statement>>,
        false_body: Vec<Located<Statement>>
    },
    While {
        condition: Located<Expression>,
        body: Vec<Located<Statement>>,
    }
}



#[derive(Debug)]
pub enum Expression {
    Call(Located<Identifier>, Vec<Located<Expression>>),
    Identifier(Identifier),
    QubitLiteral(usize),
    NumberLiteral(f64),
    BitLiteral(bool),
}
