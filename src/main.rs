use parser::error::Error;
use parser::ops::UnOp;
use syn::{Expr, Result, UnOp};

fn main() -> Result<()> {
    let code = "assert_eq!(u8::max_value(), 255)";
    let expr = syn::parse_str::<Expr>(code)?;
    println!("{:#?}", expr);
    let x = UnOp::Not(());
    println!("{:#?}", x);
    Ok(())
}
