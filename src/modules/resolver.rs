use std::{collections::HashMap, rc::Rc};
use crate::modules::{errors::{LoxError, ResolverError}, expressions::Expr, interpreter::Interpreter, statements::Stmt, token::Token, value::Value, visitor::{ExprVisitor, StmtVisitor}};

type Result<T> = std::result::Result<T, Box<dyn LoxError>>;

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Self {
        Self { interpreter, scopes: Vec::new() }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_multi(&mut self, statements: &Vec<Stmt>) -> Result<()> {
        for statement in statements {
            self.resolve(statement)?;
        }
        Ok(())
    }

    fn resolve(&mut self, stmt: &Stmt) -> Result<()> {
        stmt.accept(self)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> Result<()> {
        expr.accept(self)?;
        Ok(())
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        for i in (0..self.scopes.len()).rev() {
            if let Some(scope) = self.scopes.get(i) && scope.contains_key(&name.lexeme) {
                self.interpreter.resolve(expr, (self.scopes.len() - 1 - i) as u8);
                return;
            }
        }
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }
}

impl StmtVisitor for Resolver {
    fn visit_block(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Block { statements } = stmt {
            self.begin_scope();
            self.resolve_multi(statements)?;
            self.end_scope();
            Ok(())
        } else {
            Err(Box::new(ResolverError::new()))
        }
    }
    fn visit_expression_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        Ok(())
    }
    fn visit_function_statement(&mut self, stmt: &Stmt) -> Result<()> {
        Ok(())
    }
    fn visit_if_statement(&mut self, stmt: &Stmt) -> Result<()> {
        Ok(())
    }
    fn visit_print(&mut self, stmt: &Stmt) -> Result<()> {
        Ok(())
    }
    fn visit_return_statement(&mut self, stmt: &Stmt) -> Result<()> {
        Ok(())
    }
    fn visit_var_stmt(&mut self, stmt: &Stmt) -> Result<()> {
        if let Stmt::Var { name, initializer } = stmt {
            self.declare(name);
            if let Some(initializer) = initializer {
                self.resolve_expr(initializer)?;
            }
            self.define(name);
            Ok(())

        } else {
            Err(Box::new(ResolverError::new()))
        }
    }
    fn visit_while_statement(&mut self, stmt: &Stmt) -> Result<()> {
        Ok(())
    }
}

impl ExprVisitor for Resolver {
    fn visit_asign(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Assign { name, value: _ } = expr {
            self.resolve_expr(expr)?;
            self.resolve_local(expr, name);
            Ok(Value::Nil)
        } else {
            Err(Box::new(ResolverError::new()))
        }
    }
    fn visit_binary(&mut self, expr: &Expr) -> Result<Value> {
        Ok(Value::Nil)
    }
    fn visit_call(&mut self, expr: &Expr) -> Result<Value> {
        Ok(Value::Nil)
    }
    fn visit_grouping(&mut self, expr: &Expr) -> Result<Value> {
        Ok(Value::Nil)
    }
    fn visit_literal(&self, expr: &Expr) -> Result<Value> {
        Ok(Value::Nil)
    }
    fn visit_logical(&mut self, expr: &Expr) -> Result<Value> {
        Ok(Value::Nil)
    }
    fn visit_unary(&mut self, expr: &Expr) -> Result<Value> {
        Ok(Value::Nil)
    }
    fn visit_variable(&mut self, expr: &Expr) -> Result<Value> {
        if let Expr::Variable(name) = expr {
            if let Some(scope) = self.scopes.last() &&
               let Some(val) = scope.get(&name.lexeme) && !val {
                    return Err(Box::new(ResolverError::new_with_values(name.clone(), "Can't read local variable in its own initializer.")))
            }
            self.resolve_local(expr, name);
            Ok(Value::Nil)
        } else {
            Err(Box::new(ResolverError::new()))
        }
    }
}
