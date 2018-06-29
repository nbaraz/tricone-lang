use std::iter::Extend;

use instruction::Instruction;
use ast;

struct Function {
    name: String,
    arity: usize,
    code: Vec<Instruction>,
}

struct Type {
    name: String,
    methods: Vec<Function>,
}

fn generate_from_expr<E: Extend<Function>>(expr: &ast::Expr, out: &mut E, use_result: bool) -> Vec<Instruction>
{
    use ast::Expr::*;
    let (mut insns, pushes_value) =  match *expr {
        Num(n) => (vec![Instruction::CreateInt {value: n as i64}], true),
        Ident(ast::Ident(ref name)) => (vec![Instruction::LookupName {name: name.clone()}], true),
        _ => (vec![], false)
    };

    if pushes_value && !use_result {
        insns.push(Instruction::Pop);
    }

    insns
}

fn generate_from_block<E: Extend<Function>>(block: &ast::Block, out: &mut E, use_result: bool) -> Vec<Instruction>
{
    let len = block.0.len();
    let mut insns = vec![];
    for (idx, expr) in block.0.iter().enumerate() {
        let expr_use_result = (len - 1 == idx) && use_result;
        insns.extend(generate_from_expr(expr, out, expr_use_result));
    }

    insns

}

// TODO: error handling
fn generate_function<E: Extend<Function>>(func: ast::Func, out: &mut E) {
    generate_from_block(&func.body, out, true);
}

