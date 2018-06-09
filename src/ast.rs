
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Func {
        name: Ident,
        params: Vec<Ident>,
        body: Block,
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block(pub Vec<Expr>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expr {
    Num(i32),
    Ident(Ident),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Block(Block),
}

impl Expr {
    pub fn new_op(a: Expr, op: BinOp, b: Expr) -> Expr {
        Expr::BinOp(Box::new(a), op, Box::new(b))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
