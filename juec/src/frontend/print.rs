use crate::frontend::ast::{Expr, Module, Stmt};

/// Public function to print a Module (top-level AST)
pub fn print_module(module: &Module) {
    println!("Module:");
    for st in &module.body {
        print_stmt(st, 1);
    }
}

pub fn print_stmt(stmt: &Stmt, indent: usize) {
    let padding = "  ".repeat(indent);
    match stmt {
        Stmt::Expr(expr) => {
            println!("{}ExprStmt:", padding);
            print_expr(expr, indent + 1);
        }
        Stmt::Assign { targets, value } => {
            println!("{}Assign:", padding);
            for target in targets {
                print_expr(target, indent + 1);
            }
            print_expr(value, indent + 1);
        }
        Stmt::AugAssign { target, op, value } => {
            println!("{}AugAssign: {}", padding, op);
            print_expr(target, indent + 1);
            print_expr(value, indent + 1);
        }
        Stmt::Return(expr_opt) => {
            println!("{}Return:", padding);
            if let Some(expr) = expr_opt {
                print_expr(expr, indent + 1);
            }
        }
        Stmt::Raise(expr_opt) => {
            println!("{}Raise:", padding);
            if let Some(expr) = expr_opt {
                print_expr(expr, indent + 1);
            }
        }
        Stmt::If { test, body, orelse } => {
            println!("{}If:", padding);
            print_expr(test, indent + 1);
            println!("{}Then:", padding);
            for s in body {
                print_stmt(s, indent + 2);
            }
            if !orelse.is_empty() {
                println!("{}Else:", padding);
                for s in orelse {
                    print_stmt(s, indent + 2);
                }
            }
        }
        Stmt::For {
            target,
            iter,
            body,
            orelse,
        } => {
            println!("{}For:", padding);
            print_expr(target, indent + 1);
            print_expr(iter, indent + 1);
            println!("{}Body:", padding);
            for s in body {
                print_stmt(s, indent + 2);
            }
            if !orelse.is_empty() {
                println!("{}Else:", padding);
                for s in orelse {
                    print_stmt(s, indent + 2);
                }
            }
        }
        Stmt::While { test, body, orelse } => {
            println!("{}While:", padding);
            print_expr(test, indent + 1);
            println!("{}Body:", padding);
            for s in body {
                print_stmt(s, indent + 2);
            }
            if !orelse.is_empty() {
                println!("{}Else:", padding);
                for s in orelse {
                    print_stmt(s, indent + 2);
                }
            }
        }
        Stmt::FuncDef {
            name,
            params,
            body,
            decorators,
        } => {
            // Print decorators first
            for deco in decorators {
                println!("{}@{}", "  ".repeat(indent), deco);
            }

            // Then print the function signature
            println!("{}FuncDef: {}({:?})", "  ".repeat(indent), name, params);

            // Print the body
            for s in body {
                print_stmt(s, indent + 1);
            }
        }

        Stmt::ClassDef {
            name,
            body,
            decorators,
        } => {
            // Print decorators first
            for deco in decorators {
                println!("{}@{}", "  ".repeat(indent), deco);
            }

            // Then print the class name
            println!("{}ClassDef: {}", "  ".repeat(indent), name);

            // Print the body
            for s in body {
                print_stmt(s, indent + 1);
            }
        }

        Stmt::Pass => println!("{}Pass", padding),
        Stmt::Break => println!("{}Break", padding),
        Stmt::Continue => println!("{}Continue", padding),
        Stmt::With { items, body } => {
            println!("{}With:", padding);
            for (expr, alias) in items {
                if let Some(alias) = alias {
                    print!("{}  Item: ", padding);
                    print_expr(expr, 0);
                    println!(
                        " as {}",
                        match alias {
                            Expr::Name(n) => n,
                            _ => "_",
                        }
                    );
                } else {
                    print!("{}  Item: ", padding);
                    print_expr(expr, 0);
                    println!();
                }
            }
            for s in body {
                print_stmt(s, indent + 1);
            }
        }
        Stmt::Try {
            body,
            handlers,
            orelse,
            finalbody,
        } => {
            println!("{}Try:", padding);
            for s in body {
                print_stmt(s, indent + 1);
            }
            for (exc, handler_body) in handlers {
                match exc {
                    Some(expr) => {
                        print!("{}Except: ", padding);
                        print_expr(expr, 0);
                        println!();
                    }
                    None => println!("{}Except:", padding),
                }
                for s in handler_body {
                    print_stmt(s, indent + 1);
                }
            }
            if !orelse.is_empty() {
                println!("{}Else:", padding);
                for s in orelse {
                    print_stmt(s, indent + 1);
                }
            }
            if !finalbody.is_empty() {
                println!("{}Finally:", padding);
                for s in finalbody {
                    print_stmt(s, indent + 1);
                }
            }
        }
        _ => {
            println!("{}<Unrecognized Stmt>", padding);
        }
    }
}

pub fn print_expr(expr: &Expr, indent: usize) {
    let padding = "  ".repeat(indent);
    match expr {
        Expr::Name(name) => println!("{}Name: {}", padding, name),
        Expr::Number(n) => println!("{}Number: {}", padding, n),
        Expr::String(s) => println!("{}String: {}", padding, s),
        Expr::Bool(b) => println!("{}Bool: {}", padding, b),
        Expr::None => println!("{}None", padding),
        Expr::BinOp { left, op, right } => {
            println!("{}BinOp: {}", padding, op);
            print_expr(left, indent + 1);
            print_expr(right, indent + 1);
        }
        Expr::UnaryOp { op, expr } => {
            println!("{}UnaryOp: {}", padding, op);
            print_expr(expr, indent + 1);
        }
        Expr::Call { func, args } => {
            println!("{}Call:", padding);
            print_expr(func, indent + 1);
            for arg in args {
                print_expr(arg, indent + 1);
            }
        }
        Expr::Lambda { params, body } => {
            println!("{}Lambda: params = {:?}", padding, params);
            print_expr(body, indent + 1);
        }
    }
}
