use crate::modules::{errors::LoxError, expressions::Expr, visitor::StmtVisitor};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

#[derive(Clone, Debug)]
pub enum Stmt {
    Expression(Expr),
    Print(Expr)
}

impl Stmt {
    pub fn accept(&self, visitor: &dyn StmtVisitor) -> Result<()> {
        match self {
            Stmt::Expression(_) => {
                Ok(())
            },
            Stmt::Print(_) => {
                visitor.visit_print(self)
            },
        }
    }
}
