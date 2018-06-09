extern crate lalrpop_util;

pub mod ast;
pub mod grammar;

#[cfg(test)]
mod tests {

    macro_rules! test_parser {
        ($parser:expr, $($to_parse:expr => $expected:expr,)*) => {{
            let parser = $parser;
            $(
                assert_eq!($expected, parser.parse($to_parse).unwrap());
            )*
        }};
    }

    use ast::{self, Expr, Item};
    use grammar;

    fn ident(id: &str) -> Expr {
        Expr::Ident(ast::Ident(id.to_owned()))
    }

    #[test]
    fn test_terms() {
        use ast::Expr::Num;

        test_parser![
            grammar::TermParser::new(),
            "20" => Num(20),
            "(20)" => Num(20),
            "(887698766)" => Num(887698766),
            "(    20 )" => Num(20),
            "a" => ident("a"),
            "(A  )" => ident("A"),
        ];
    }

    #[test]
    fn test_binops() {
        use ast::BinOp::*;
        use ast::Expr::Num;
        let op = Expr::new_op;

        test_parser![
            grammar::ExprParser::new(),
            "1 + 2" => op(Num(1), Add, Num(2)),
            "1 * 2 + b" => op(op(Num(1), Mul, Num(2)), Add, ident("b")),
            "1 * (2 + b)" => op(Num(1), Mul, op(Num(2), Add, ident("b"))),
        ]
    }

    macro_rules! inner_block {
        ($($e:expr),*) => {
            ast::Block(vec![$($e,)*])
        };
    }

    macro_rules! block {
        ($($e:expr),*) => {
            Expr::Block(inner_block![$($e),*])
        };
    }

    #[test]
    fn test_block() {
        use ast::BinOp::*;
        use ast::Expr::Num;
        let op = Expr::new_op;

        test_parser![
            grammar::ExprParser::new(),
            "{1 + 2}" => block![op(Num(1), Add, Num(2))],
            "{1 + 2; 3 + 3}" => block![op(Num(1), Add, Num(2)), op(Num(3), Add, Num(3))],
            "{1 + 2; 3 + 3; }" => block![op(Num(1), Add, Num(2)), op(Num(3), Add, Num(3))],
        ]
    }

    macro_rules! function {
        ($name:expr, ($($p:expr),*) $body:expr) => {
            Item::Func {
                name: ast::Ident($name.to_owned()),
                params: vec![$(ast::Ident($p.to_owned())),*],
                body: $body,
            }
        };
    }

    #[test]
    fn test_func() {
        use ast::BinOp::*;
        use ast::Expr::Num;
        let op = Expr::new_op;

        test_parser![
            grammar::FunctionParser::new(),
            "func test() {}" => function!("test", () inner_block![]),
            "func test(a) {}" => function!("test", ("a") inner_block![]),
            "func test(a) { 1 }" => function!("test", ("a") inner_block![Num(1)]),
            "func test(a, b) { a + b }" =>
                function!("test", ("a", "b") inner_block![op(ident("a"), Add, ident("b"))]),
        ]
    }


}
