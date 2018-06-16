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

fn generate_from_expr<E: Extend<Function>>(expr: &ast::Expr, out: E) -> Vec<Instruction>
{
    use ast::Expr::*;
    match *expr {
        Num(n) => vec![Instruction::CreateInt {value: n as i64}],
        Ident(ast::Ident(ref name)) => vec![Instruction::LookupName {name: name.clone()}],

    }
}

fn generate_from_block<E: Extend<Function>>(block: &ast::Block, out: E) -> Vec<Instruction>
{
    for expr in block.0 {
        use ast::Expr::*;
        match expr {
            expr @ (BinOp(..) | Block(..) | Assign(..) | AttrLookup)
        }
    }

}

// TODO: error handling
fn generate_function<E: Extend<Function>>(func: ast::Func, out: E) {
    generate_from_block(func.body, out);
}

