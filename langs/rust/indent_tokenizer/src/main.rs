extern crate unindent;
use unindent::unindent;


mod ind_tok;


fn main() {

    let input = unindent("
        ....
        ....
            ....");

    println!("{}", input);

    // for s in ind_tok::split_lines(&input) {
    //     println!("{}", s)
    // }
}