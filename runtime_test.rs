// Basic runtime test for CURSED language features

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum CursedValue {
    Integer(i64),
    String(String),
    Boolean(bool),
    Nil,
}

#[derive(Debug)]
pub struct ExecutionContext {
    variables: HashMap<String, CursedValue>,
    functions: HashMap<String, FunctionDef>,
}

#[derive(Debug, Clone)]
pub struct FunctionDef {
    parameters: Vec<String>,
    body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    Let { name: String, value: Expression },
    Expression(Expression),
    If { condition: Expression, then_branch: Vec<Statement>, else_branch: Option<Vec<Statement>> },
    Return(Option<Expression>),
    Function { name: String, parameters: Vec<String>, body: Vec<Statement> },
}

#[derive(Debug, Clone)]
pub enum Expression {
    Integer(i64),
    String(String), 
    Boolean(bool),
    Identifier(String),
    Binary { left: Box<Expression>, operator: String, right: Box<Expression> },
    Call { function: String, arguments: Vec<Expression> },
}

impl ExecutionContext {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: CursedValue) {
        self.variables.insert(name, value);
    }

    pub fn get_variable(&self, name: &str) -> Option<CursedValue> {
        self.variables.get(name).cloned()
    }

    pub fn set_function(&mut self, name: String, func: FunctionDef) {
        self.functions.insert(name, func);
    }

    pub fn get_function(&self, name: &str) -> Option<FunctionDef> {
        self.functions.get(name).cloned()
    }
}

pub struct RuntimeExecutor;

impl RuntimeExecutor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute_statement(&mut self, statement: &Statement, context: &mut ExecutionContext) -> Result<CursedValue, String> {
        match statement {
            Statement::Let { name, value } => {
                let val = self.evaluate_expression(value, context)?;
                context.set_variable(name.clone(), val.clone());
                Ok(val)
            }
            Statement::Expression(expr) => {
                self.evaluate_expression(expr, context)
            }
            Statement::If { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate_expression(condition, context)?;
                if self.is_truthy(&cond_val) {
                    let mut last_val = CursedValue::Nil;
                    for stmt in then_branch {
                        last_val = self.execute_statement(stmt, context)?;
                    }
                    Ok(last_val)
                } else if let Some(else_stmts) = else_branch {
                    let mut last_val = CursedValue::Nil;
                    for stmt in else_stmts {
                        last_val = self.execute_statement(stmt, context)?;
                    }
                    Ok(last_val)
                } else {
                    Ok(CursedValue::Nil)
                }
            }
            Statement::Return(expr) => {
                if let Some(e) = expr {
                    self.evaluate_expression(e, context)
                } else {
                    Ok(CursedValue::Nil)
                }
            }
            Statement::Function { name, parameters, body } => {
                let func_def = FunctionDef {
                    parameters: parameters.clone(),
                    body: body.clone(),
                };
                context.set_function(name.clone(), func_def);
                Ok(CursedValue::Nil)
            }
        }
    }

    pub fn evaluate_expression(&mut self, expression: &Expression, context: &mut ExecutionContext) -> Result<CursedValue, String> {
        match expression {
            Expression::Integer(i) => Ok(CursedValue::Integer(*i)),
            Expression::String(s) => Ok(CursedValue::String(s.clone())),
            Expression::Boolean(b) => Ok(CursedValue::Boolean(*b)),
            Expression::Identifier(name) => {
                context.get_variable(name).ok_or_else(|| format!("Undefined variable: {}", name))
            }
            Expression::Binary { left, operator, right } => {
                let left_val = self.evaluate_expression(left, context)?;
                let right_val = self.evaluate_expression(right, context)?;
                self.apply_binary_operator(&left_val, operator, &right_val)
            }
            Expression::Call { function, arguments } => {
                match function.as_str() {
                    "print" => {
                        for arg in arguments {
                            let val = self.evaluate_expression(arg, context)?;
                            print!("{}", self.value_to_string(&val));
                        }
                        println!();
                        Ok(CursedValue::Nil)
                    }
                    _ => {
                        if let Some(func_def) = context.get_function(function) {
                            let mut func_context = ExecutionContext::new();
                            
                            if arguments.len() != func_def.parameters.len() {
                                return Err(format!("Function {} expects {} arguments, got {}", 
                                    function, func_def.parameters.len(), arguments.len()));
                            }
                            
                            for (param, arg) in func_def.parameters.iter().zip(arguments) {
                                let arg_val = self.evaluate_expression(arg, context)?;
                                func_context.set_variable(param.clone(), arg_val);
                            }
                            
                            let mut result = CursedValue::Nil;
                            for stmt in &func_def.body {
                                result = self.execute_statement(stmt, &mut func_context)?;
                                // Handle return statements
                                if let Statement::Return(_) = stmt {
                                    break;
                                }
                            }
                            
                            Ok(result)
                        } else {
                            Err(format!("Undefined function: {}", function))
                        }
                    }
                }
            }
        }
    }

    fn apply_binary_operator(&self, left: &CursedValue, operator: &str, right: &CursedValue) -> Result<CursedValue, String> {
        match (left, right) {
            (CursedValue::Integer(l), CursedValue::Integer(r)) => {
                match operator {
                    "+" => Ok(CursedValue::Integer(l + r)),
                    "-" => Ok(CursedValue::Integer(l - r)),
                    "*" => Ok(CursedValue::Integer(l * r)),
                    "/" => {
                        if *r == 0 {
                            Err("Division by zero".to_string())
                        } else {
                            Ok(CursedValue::Integer(l / r))
                        }
                    }
                    ">" => Ok(CursedValue::Boolean(l > r)),
                    "<" => Ok(CursedValue::Boolean(l < r)),
                    ">=" => Ok(CursedValue::Boolean(l >= r)),
                    "<=" => Ok(CursedValue::Boolean(l <= r)),
                    "==" => Ok(CursedValue::Boolean(l == r)),
                    "!=" => Ok(CursedValue::Boolean(l != r)),
                    _ => Err(format!("Unknown operator: {}", operator)),
                }
            }
            (CursedValue::String(l), CursedValue::String(r)) => {
                match operator {
                    "+" => Ok(CursedValue::String(format!("{}{}", l, r))),
                    "==" => Ok(CursedValue::Boolean(l == r)),
                    "!=" => Ok(CursedValue::Boolean(l != r)),
                    _ => Err(format!("Unsupported string operator: {}", operator)),
                }
            }
            _ => Err(format!("Type mismatch in binary operation")),
        }
    }

    fn is_truthy(&self, value: &CursedValue) -> bool {
        match value {
            CursedValue::Boolean(b) => *b,
            CursedValue::Integer(i) => *i != 0,
            CursedValue::String(s) => !s.is_empty(),
            CursedValue::Nil => false,
        }
    }

    fn value_to_string(&self, value: &CursedValue) -> String {
        match value {
            CursedValue::Integer(i) => i.to_string(),
            CursedValue::String(s) => s.clone(),
            CursedValue::Boolean(b) => b.to_string(),
            CursedValue::Nil => "nil".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_assignment() {
        let mut executor = RuntimeExecutor::new();
        let mut context = ExecutionContext::new();

        // sus x = 10
        let stmt = Statement::Let {
            name: "x".to_string(),
            value: Expression::Integer(10),
        };
        
        let result = executor.execute_statement(&stmt, &mut context).unwrap();
        assert_eq!(result, CursedValue::Integer(10));
        assert_eq!(context.get_variable("x"), Some(CursedValue::Integer(10)));
    }

    #[test]
    fn test_arithmetic() {
        let mut executor = RuntimeExecutor::new();
        let mut context = ExecutionContext::new();

        // Set variables
        context.set_variable("x".to_string(), CursedValue::Integer(10));
        context.set_variable("y".to_string(), CursedValue::Integer(20));

        // sus z = x + y
        let stmt = Statement::Let {
            name: "z".to_string(),
            value: Expression::Binary {
                left: Box::new(Expression::Identifier("x".to_string())),
                operator: "+".to_string(),
                right: Box::new(Expression::Identifier("y".to_string())),
            },
        };

        let result = executor.execute_statement(&stmt, &mut context).unwrap();
        assert_eq!(result, CursedValue::Integer(30));
        assert_eq!(context.get_variable("z"), Some(CursedValue::Integer(30)));
    }

    #[test]
    fn test_if_statement() {
        let mut executor = RuntimeExecutor::new();
        let mut context = ExecutionContext::new();

        context.set_variable("x".to_string(), CursedValue::Integer(15));

        // lowkey x > 10 { sus result = 1; } highkey { sus result = 0; }
        let stmt = Statement::If {
            condition: Expression::Binary {
                left: Box::new(Expression::Identifier("x".to_string())),
                operator: ">".to_string(),
                right: Box::new(Expression::Integer(10)),
            },
            then_branch: vec![Statement::Let {
                name: "result".to_string(),
                value: Expression::Integer(1),
            }],
            else_branch: Some(vec![Statement::Let {
                name: "result".to_string(),
                value: Expression::Integer(0),
            }]),
        };

        executor.execute_statement(&stmt, &mut context).unwrap();
        assert_eq!(context.get_variable("result"), Some(CursedValue::Integer(1)));
    }

    #[test] 
    fn test_function_call() {
        let mut executor = RuntimeExecutor::new();
        let mut context = ExecutionContext::new();

        // slay add(a, b) { yolo a + b; }
        let func_stmt = Statement::Function {
            name: "add".to_string(),
            parameters: vec!["a".to_string(), "b".to_string()],
            body: vec![Statement::Return(Some(Expression::Binary {
                left: Box::new(Expression::Identifier("a".to_string())),
                operator: "+".to_string(),
                right: Box::new(Expression::Identifier("b".to_string())),
            }))],
        };

        executor.execute_statement(&func_stmt, &mut context).unwrap();

        // Call function: add(5, 3)
        let call_expr = Expression::Call {
            function: "add".to_string(),
            arguments: vec![Expression::Integer(5), Expression::Integer(3)],
        };

        let result = executor.evaluate_expression(&call_expr, &mut context).unwrap();
        assert_eq!(result, CursedValue::Integer(8));
    }
}

fn main() {
    println!("Running CURSED runtime tests...");
    
    let mut executor = RuntimeExecutor::new();
    let mut context = ExecutionContext::new();

    // Test 1: Variable assignment
    println!("\n=== Test 1: Variable Assignment ===");
    let stmt1 = Statement::Let {
        name: "x".to_string(),
        value: Expression::Integer(42),
    };
    executor.execute_statement(&stmt1, &mut context).unwrap();
    println!("x = {}", executor.value_to_string(&context.get_variable("x").unwrap()));

    // Test 2: Arithmetic
    println!("\n=== Test 2: Arithmetic ===");
    let stmt2 = Statement::Let {
        name: "y".to_string(),
        value: Expression::Binary {
            left: Box::new(Expression::Identifier("x".to_string())),
            operator: "+".to_string(),
            right: Box::new(Expression::Integer(8)),
        },
    };
    executor.execute_statement(&stmt2, &mut context).unwrap();
    println!("y = x + 8 = {}", executor.value_to_string(&context.get_variable("y").unwrap()));

    // Test 3: Function definition and call
    println!("\n=== Test 3: Function Call ===");
    let func_stmt = Statement::Function {
        name: "multiply".to_string(),
        parameters: vec!["a".to_string(), "b".to_string()],
        body: vec![Statement::Return(Some(Expression::Binary {
            left: Box::new(Expression::Identifier("a".to_string())),
            operator: "*".to_string(),
            right: Box::new(Expression::Identifier("b".to_string())),
        }))],
    };
    executor.execute_statement(&func_stmt, &mut context).unwrap();

    let call_expr = Expression::Call {
        function: "multiply".to_string(),
        arguments: vec![Expression::Integer(6), Expression::Integer(7)],
    };
    let result = executor.evaluate_expression(&call_expr, &mut context).unwrap();
    println!("multiply(6, 7) = {}", executor.value_to_string(&result));

    // Test 4: If statement
    println!("\n=== Test 4: If Statement ===");
    let if_stmt = Statement::If {
        condition: Expression::Binary {
            left: Box::new(Expression::Identifier("x".to_string())),
            operator: ">".to_string(),
            right: Box::new(Expression::Integer(40)),
        },
        then_branch: vec![Statement::Expression(Expression::Call {
            function: "print".to_string(),
            arguments: vec![Expression::String("x is greater than 40".to_string())],
        })],
        else_branch: Some(vec![Statement::Expression(Expression::Call {
            function: "print".to_string(),
            arguments: vec![Expression::String("x is not greater than 40".to_string())],
        })]),
    };
    executor.execute_statement(&if_stmt, &mut context).unwrap();

    println!("\n=== All tests completed successfully! ===");
}
