use zk_stark::modules::field::{*};
use zk_stark::modules::univariate_poly::{*};
use zk_stark::modules::trace_lde::{*};


fn main() {

    // // solution 
    // let x: FieldElement = FieldElement::new(3141592);
    // println!("\nX: {}", x);

    // // fib square trace
    // let a = fib_square_trace(x.value);
    // println!("\nTrace[0]: {}", a.get(0).unwrap());
    // println!("Trace[1]: {}", a.get(1).unwrap());
    // println!("Trace[1022]: {}", a.get(a.len()-1).unwrap());

    // // UNCOMMENT TO COMPUTE LAGRANGE INTERPOLATION
    // /*
    // // domain of interpolation
    // let domain: Vec<FieldElement> = poly_domain();
    // println!("Domain length: {}", domain.len());
    // println!("Trace length: {}", a.len());
    // // lagrange interpolation (highest computational expense)
    // let f: Polynomial = Polynomial::lagrange(domain.clone(), a);
    // let eval_1: FieldElement = f.eval((domain.get(0).unwrap()).clone());
    // let eval_2: FieldElement = f.eval((domain.get(1).unwrap()).clone());
    // let eval_1023: FieldElement = f.eval(domain.get(1022).unwrap().clone());
    // println!("Eval at 0: {}", eval_1);
    // println!("Eval at 1: {}", eval_2);
    // println!("Eval at 1023: {}", eval_1023);
    // // save intermediate states for faster development
    // f.save("lagrange_poly.txt").unwrap();
    // */
    
    // // load pre evaluated polynomial for faster development
    // let f: Polynomial = Polynomial::load("lagrange_poly.txt").unwrap(); // NOTE: ensure in project root directory
    
    // // get extended coset domain
    // let domain: Vec<FieldElement> = eval_domain();

    // let coset_eval: Vec<FieldElement> = f.eval_domain(domain.clone());
    // println!("Coset eval length: {:?}", coset_eval);

    // println!("Coest eval:, {:?}", coset_eval);

}


