

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Func {
    pub name: Ident,
    pub params: Vec<Ident>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    Func(Func),
    Type {
        name: Ident,
        methods: Vec<Func>
    },
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
    Assign(Ident, Box<Expr>),
    AttrLookup(Box<Expr>, Ident),
    Call(Box<Expr>, Vec<Expr>),
}

impl Expr {
    pub fn new_op(a: Expr, op: BinOp, b: Expr) -> Expr {
        Expr::BinOp(Box::new(a), op, Box::new(b))
    }

    pub fn new_assign(ident: Ident, expr: Expr) -> Expr {
        Expr::Assign(ident, Box::new(expr))
    }

    pub fn new_attrlookup(expr: Expr, ident: Ident) -> Expr {
        Expr::AttrLookup(Box::new(expr), ident)
    }

    pub fn new_call(func: Expr, args: Vec<Expr>) -> Expr {
        Expr::Call(Box::new(func), args)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}
