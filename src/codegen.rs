use std::iter::Extend;

use ast;
use instruction::Instruction;

struct Function {
    name: String,
    arity: usize,
    code: Vec<Instruction>,
}

struct Type {
    name: String,
    methods: Vec<Function>,
}

fn generate_from_expr<E: Extend<Function>>(
    expr: &ast::Expr,
    out: &mut E,
    use_result: bool,
) -> Vec<Instruction> {
    use ast::Expr::*;
    let (mut insns, pushes_value) = match *expr {
        Num(n) => (vec![Instruction::CreateInt { value: n as i64 }], true),
        Ident(ast::Ident(ref name)) => (vec![Instruction::LookupName { name: name.clone() }], true),
        BinOp(ref a, op, ref b) => (generate_binop(&a, op, &b, out, use_result), use_result),
        Block(ref block) => (generate_from_block(block, out, use_result), use_result),
        Assign(ref ident, ref expr) => (generate_assign(&ident, &expr, out), false),
        AttrLookup(ref expr, ref ident) => (generate_assign(&ident, &expr, out), false),
        _ => (vec![], false),
    };

    if pushes_value && !use_result {
        insns.push(Instruction::Pop);
    }

    insns
}

fn generate_binop<E: Extend<Function>>(
    a: &ast::Expr,
    op: ast::BinOp,
    b: &ast::Expr,
    out: &mut E,
    use_result: bool,
) -> Vec<Instruction> {
    let mut v = vec![];
    v.extend(generate_from_expr(a, out, true));
    v.extend(generate_from_expr(b, out, true));
    v.push(match op {
        ast::BinOp::Add => Instruction::CallMethod {
            name: "add".to_owned(),
            num_args: 1,
            use_result,
        },
        _ => unreachable!(),
    });
    v
}

fn generate_assign<E: Extend<Function>>(
    ident: &ast::Ident,
    expr: &ast::Expr,
    out: &mut E,
) -> Vec<Instruction> {
    use instruction::Instruction::*;
    let mut v = generate_from_expr(expr, out, true);
    v.push(Assign {
        name: ident.0.clone(),
    });
    v
}

fn generate_attrlookup<E: Extend<Function>>(
    expr: &ast::Expr,
    ident: &ast::Ident,
    out: &mut E,
    use_result: bool,
) -> Vec<Instruction> {
    use instruction::Instruction::*;
    let mut v = generate_from_expr(expr, out, true);
    v.push(GetMember {
        name: ident.0.clone(),
    });
    v
}

fn generate_from_block<E: Extend<Function>>(
    block: &ast::Block,
    out: &mut E,
    use_result: bool,
) -> Vec<Instruction> {
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
