// can be used as a library too
/***************************************************************************************************************************************************/

// section helper functions
/***************************************************************************************************************************************************/
fn factorial_worker(mut held_number: u128, n: u128) -> u128 {
    if n > 0 {
        held_number = n * factorial_worker(held_number, n - 1);
    }
    return held_number;
}
fn factorial(held_number: u128,) -> u128 {
    return factorial_worker(1, held_number);
}
// section usable functions
/***************************************************************************************************************************************************/
fn blr_get_term(polynomial_power: u128, polynomial_term: u128) -> String{
    if polynomial_power < polynomial_term {
        panic!("power has to be bigger or equal than the term")
    }

    let multiplier = factorial(polynomial_power) / (factorial(polynomial_power - polynomial_term) * factorial(polynomial_term));
     
    
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
fn main() {
    println!("Hello, world!");
    blr_get_term(3, 2);
    blr_get_term(6, 0);
    blr_get_term(6, 1);
    blr_get_term(6, 2);
    blr_get_term(6, 3);
    blr_get_term(6, 4);
    blr_get_term(6, 5);
    blr_get_term(6, 6);
    println!("");
    let test = blr_get_power(5);
    println!("{test}");
}
