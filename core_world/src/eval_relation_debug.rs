/// Evaluation relation implementation with debug logging
use crate::core_expr::CoreExpr;
use std::collections::HashMap;

/// Environment type for variable lookup
pub type Env = HashMap<usize, CoreExpr>;

/// Closure type representing a lambda abstraction with its environment
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Closure {
    pub env: Env,
    pub body: CoreExpr,
}

/// Evaluation result type
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalResult {
    Value(CoreExpr),
    Closure(Closure),
}

/// The Eval relation: env ⊢ expr ⇒ value
pub fn eval_debug(env: &Env, expr: CoreExpr) -> EvalResult {
    eval_with_limit_debug(env, expr, 1000, 0)
}

/// Evaluate with recursion limit and debug logging
fn eval_with_limit_debug(env: &Env, expr: CoreExpr, limit: usize, depth: usize) -> EvalResult {
    let indent = "  ".repeat(depth);
    println!("{}{}eval: {} in env {:?}", indent, depth, expr, env);

    if limit == 0 {
        println!("{}{}Hit limit, returning: {}", indent, depth, expr);
        return EvalResult::Value(expr);
    }

    match expr {
        CoreExpr::Var(index) => {
            println!("{}{}Var lookup: index {}", indent, depth, index);
            if let Some(value) = env.get(&index) {
                println!("{}{}Found in env: {} → {}", indent, depth, index, value);
                eval_with_limit_debug(env, value.clone(), limit - 1, depth + 1)
            } else {
                println!("{}{}Free variable: {}", indent, depth, index);
                EvalResult::Value(CoreExpr::Var(index))
            }
        }
        CoreExpr::Lam(body) => {
            println!(
                "{}{}Lambda: creating closure with body {}",
                indent, depth, *body
            );
            EvalResult::Closure(Closure {
                env: env.clone(),
                body: *body,
            })
        }
        CoreExpr::App(func, arg) => {
            println!("{}{}App: func={}, arg={}", indent, depth, *func, *arg);
            let func_expr = *func;
            let arg_expr = *arg;
            let func_result = eval_with_limit_debug(env, func_expr.clone(), limit - 1, depth + 1);
            let arg_result = eval_with_limit_debug(env, arg_expr.clone(), limit - 1, depth + 1);

            println!(
                "{}{}App results: func={:?}, arg={:?}",
                indent, depth, func_result, arg_result
            );

            match (func_result, arg_result) {
                (EvalResult::Closure(closure), EvalResult::Value(arg_value)) => {
                    println!(
                        "{}{}Applying closure to value: {:?}",
                        indent, depth, arg_value
                    );
                    apply_closure_debug(closure, arg_value, limit - 1, depth + 1)
                }
                (EvalResult::Closure(closure), EvalResult::Closure(arg_closure)) => {
                    println!("{}{}Applying closure to closure", indent, depth);
                    apply_closure_debug(
                        closure,
                        CoreExpr::Lam(Box::new(arg_closure.body)),
                        limit - 1,
                        depth + 1,
                    )
                }
                _ => {
                    println!("{}{}Cannot apply, returning app", indent, depth);
                    EvalResult::Value(CoreExpr::App(Box::new(func_expr), Box::new(arg_expr)))
                }
            }
        }
    }
}

/// Apply a closure to an argument value with debug logging
fn apply_closure_debug(
    closure: Closure,
    arg_value: CoreExpr,
    limit: usize,
    depth: usize,
) -> EvalResult {
    let indent = "  ".repeat(depth);
    println!(
        "{}{}apply_closure: body={}, arg={}",
        indent, depth, closure.body, arg_value
    );
    println!("{}{}closure env: {:?}", indent, depth, closure.env);

    // Shift all existing bindings in the environment by 1
    let shifted_env: Env = closure
        .env
        .iter()
        .map(|(k, v)| (*k + 1, v.clone()))
        .collect();

    println!("{}{}shifted env: {:?}", indent, depth, shifted_env);

    // Add the argument at index 0
    let mut final_env = shifted_env;
    final_env.insert(0, arg_value);

    println!("{}{}final env: {:?}", indent, depth, final_env);
    println!(
        "{}{}evaluating body: {} in final env",
        indent, depth, closure.body
    );

    eval_with_limit_debug(&final_env, closure.body, limit - 1, depth + 1)
}
