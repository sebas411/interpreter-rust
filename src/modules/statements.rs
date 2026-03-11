use std::fmt::{self, Write};

use crate::modules::{errors::LoxError, expressions::Expr, token::Token, visitor::StmtVisitor};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(Box<Expr>),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Print(Box<Expr>),
    Return {
        keyword: Token,
        value: Option<Box<Expr>>,
    },
    Var{
        name: Token,
        initializer: Option<Box<Expr>>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Box<Expr>,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Box<Expr>,
        body: Box<Stmt>,
    },
}

impl Stmt {
    pub fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<()> {
        match self {
            Stmt::Expression(_) => {
                visitor.visit_expression_stmt(self)
            },
            Stmt::Print(_) => {
                visitor.visit_print(self)
            },
            Stmt::Var { .. } => {
                visitor.visit_var_stmt(self)
            },
            Stmt::Block { .. } => {
                visitor.visit_block(self)
            },
            Stmt::If { .. } => {
                visitor.visit_if_statement(self)
            },
            Stmt::While { .. } => {
                visitor.visit_while_statement(self)
            },
            Stmt::Function { .. } => {
                visitor.visit_function_statement(self)
            },
            Stmt::Return { .. } => {
                visitor.visit_return_statement(self)
            },
        }
    }
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Block { statements } => {
                f.write_str("{\n")?;
                for statement in statements {
                    f.write_fmt(format_args!("{}\n", statement))?;
                }
                f.write_char('}')?;
                Ok(())
            },
            Self::Expression(expr) => f.write_fmt(format_args!("{}", expr)),
            Self::While { condition, body } => {
                f.write_fmt(format_args!("while ({}) {{\n{}\n}}", condition, body))
            }
            Self::Var { name, initializer } => {
                f.write_fmt(format_args!("var {}", name.lexeme))?;
                if let Some(initializer) = initializer {
                    f.write_fmt(format_args!(" = {}", initializer))?;
                }
                f.write_char(';')
            }
            Self::Print(expr) => {
                f.write_fmt(format_args!("print {}", expr))
            }
            _ => f.write_str("[some statement]"),
        }
    }
}
