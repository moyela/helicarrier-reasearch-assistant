//finding the modular multiplicative inverse of a given number and modulus pair

//adding this std::io moudule lets the program use inbuilt input/output functions
//we need this so we can accept our number and modulus from the user as inputs
use std::io;


//in Rust the main function below automatically runs once the program is compiled
fn main() 
    {   
        //the function below is used to print text to the console in rust
        println!("Evaluating the modular multiplicative inverse of a given number and modulus pair");

        //variables in Rust are immutable by default
        //declaring mut changes input_number and input_variable below to mutable variables
        let mut input_number = String::new();
        let mut input_modulus = String::new();

        println!("type your number");
        
        //the match statement below accepts a user input and writes it to: input_number
        match io::stdin().read_line(&mut input_number)
        //match statements return a response depending on the outcome of the input
        {
            //if successful, match returns Ok below
            Ok(_) => println!("your number is: {}, now enter your modulus", input_number);

            //if not successful, match returns Err below
            Err(e) => println!("oh! something went wrong. please restart");
 
        }

        println!("type your modulus");

        //the match statement below accepts a user input and writes it to: input_modulus
        match io::stdin().read_line(&mut input_modulus) 
        {

            Ok(_) => println!("success! your number is: {} and your modulus is: {}", input_number, input_modulus);

            Err(e) => println!("oh! something went wrong. please restart");

        }

        //if successful, our inputs will be in character type, similar to string
        //the trim function below removes the whitespace around our input
        //ch.to.digit() converts our input from character data type to integer
        //the number inside ch.to.digit() indicates the base conversion, 10 means decimal [base 10]
        input_number = input_number.trim() = ch.to_digit(10).unwrap();
        input_modulus = input_modulus.trim() = ch.to_digit(10).unwrap();

        //once our two input values are confirmed, we run the modular inverse function described below
        modular_inverse(input_number, input_modulus);

    }


//the given number is accepted as 'a'
//the given modulus is accepted as 'm'
fn modular_inverse(a, m)
    {
        let a = input_number;
        let m = input_modulus;
        let initial_modulus = m;

        // in 'y: i64' below, y is assigned the data type i64
        // i64 refers to 64 bit signed integer data type used in Rust
        let mut y: i64 = 0;

        let mut x: i64 = 1;
        //variables in Rust are immutable by default
        //declaring mut changes y and x to mutable variables

        //the if statement below runs the code in bracket if its condition is true
        //the function below returns 0 because the modular multiplicative inverse of any number with modulus 1 is 0
        //1 is a perfect divisor for all integers
        if m == 1
        {

            return 0;

        }
        
        //the while statement below runs the code in bracket on a loop as long as its condition remains true
        //as long as a remains greater that 1, the while statement runs
        while a > 1
        {

            let q = a / m;

            //to convert the floating point result from 'q' above into an integer 'q1':
            let mut q1 = (q * 100.0).round() / 1.0;

            let mut t = m;

            //using the Euclidean algorithm, we evaluate the remainder and divisors of a and m
            //using the new values, we iterate until a drops below 1
            m = a % m;

            a = t;

            t = y;

            //updating y and x and iterating
            y = x - q1 * y;

            x = t;

        }

        //if x is negative, to make it positive again:
        if x < 0
        {

            x = x + initial_modulus;

            return x;
        }

        //if everything has worked exactly as it's supposed to up to this point, we print the modular multiplicative inverse as given below
        println!("{} is the modular multiplicative inverse of a number: {} with modulus: {}", x, input_number, input_modulus); 

        //ces't fin!!!

    }
