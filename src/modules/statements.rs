use crate::modules::{errors::LoxError, expressions::Expr, token::Token, visitor::StmtVisitor};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(Expr),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Print(Expr),
    Var{
        name: Token,
        initializer: Option<Expr>,
    },
    Block {
        statements: Vec<Stmt>,
    },
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
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
            }
        }
    }
}
