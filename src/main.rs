use std::fs::read;

use factorial::Factorial;

// can be used as a library too
/***************************************************************************************************************************************************/
extern crate text_io;
extern crate factorial;
// section helper functions
/***************************************************************************************************************************************************/

// section usable functions
/***************************************************************************************************************************************************/
fn blr_get_term(polynomial_power: u128, polynomial_term: u128) -> String{
    if polynomial_power < polynomial_term {
        panic!("power has to be bigger or equal than the term")
    }

    let multiplier = polynomial_power.factorial() / ((polynomial_power - polynomial_term).factorial() * polynomial_term.factorial());
    
    let mut a = "".to_owned();
    if (polynomial_power - polynomial_term) > 1 {
        a = "a^".to_owned();
        a.push_str((polynomial_power - polynomial_term).to_string().trim());
    }
    else if (polynomial_power - polynomial_term) == 1 {
        a = "a".to_owned();
    }

    let mut b = "".to_owned();
    if (polynomial_term) > 1 {
        b = "b^".to_owned();
        b.push_str((polynomial_term).to_string().trim());
    }
    else if (polynomial_term) == 1 {
        b = "b".to_owned();
    }

    let mut out: String = "".to_owned();
    if multiplier <= 1 {}
    else {
        out.push_str(multiplier.to_string().trim());
        out.push_str(" ");
    }
    out.push_str(a.to_string().trim());
    out.push_str(" ");
    out.push_str(b.to_string().trim());

    return out;
}
fn blr_get_power(polynomial_power: u128) -> String{
    let mut out = "(a+b)^".to_owned();
    out.push_str(polynomial_power.to_string().trim());
    out.push_str("=".trim());
    for i in 0..polynomial_power {
        out.push_str(blr_get_term(polynomial_power, i).trim());
        if i == (polynomial_power - 1) {
            out.push_str(" ".trim());
        }
        else{
            out.push_str("+".trim());
        }
    }
    return out;
}
// section main
/***************************************************************************************************************************************************/
#[cfg(feature = "tests")]
fn test_row() {
    println!("Hello, world!");
    println!("");
    let row = blr_get_power(5);
    println!("{row}");
}
#[cfg(feature = "tests")]
fn test_terms(){
    println!("Hewwo, Wowld!");
    println!("");
    let terms: [String;8] = [blr_get_term(3, 2),blr_get_term(6, 0),blr_get_term(6, 1),blr_get_term(6, 2),blr_get_term(6, 3),blr_get_term(6, 4),blr_get_term(6, 5),blr_get_term(6, 6)];
    for i in 0..8{
        println!("{}", terms[i]);
    }
    //println!("{:#?}", terms);
}
fn main(){
    #[cfg(feature = "tests")]{
        test_row();
        test_terms();
    }
    let full_version:u16=1;
    let dev_version:u16=7;
    println!("version!({})",full_version.factorial()+dev_version.factorial());    

    loop{
        println!("Enter if you want a term (T) or a full power(P)");
        let term_or_line: String = text_io::read!();
        match term_or_line.as_str() {
            "T" | "t" => {
                println!("Choose the triangle power (maximum 34)");
                let power:u128 = text_io::read!();
                println!("Choose the term");
                let term:u128 = text_io::read!();
                println!("The term is:");
                let answer:String = blr_get_term(power, term);
                println!("{}", answer);
                println!("You can copy it to a LaTeX interpreter (MS Word works too)");
                println!("press enter to exit");
                let buff:u8 = text_io::read!();
                break;
            },
            "P" | "p" => {
                println!("Choose the triangle power (maximum 34)");
                let power:u128 = text_io::read!();
                println!("The term is:");
                let answer:String = blr_get_power(power);
                println!("{}", answer);
                println!("You can copy it to a LaTeX interpreter (MS Word works too)");
                println!("press enter to exit");
                let buff:u8 = text_io::read!();
                break;
            },
            _ => {
                println!("Not a valid input option.");
                continue;
            }
        }
    }
}