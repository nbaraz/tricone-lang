extern crate tricone_lang;

fn main() {
    let p = tricone_lang::grammar::ExprParser::new();
    let res = p.parse("{1 + 2; 3 + 3}");
    println!("{:#?}", res);
    match res {
        Err(e) => println!("{}", e),
        _ => {}
    }
}