pub mod lexing;

fn main() 
{
    let text = ": ;; += ==";
    let result = lexing::lex_text(text);
    println!("{:?}", result);
}
