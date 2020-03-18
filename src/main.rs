// These comments are throughout my rust code to provide meaning to statements and expressions
// Gives me a better understanding while I'm learning but I will try to keep well commented source

// const aren't just immutable but always immutable
// const can also be declared in global scope (main usage)
// can only be bound to a constant expression
// naming convention is all uppercase with underscores between words
const Y_CONST: i32 = 100_000; // must always annotate type on constants (: type)
                              // underscores can be inserted into numeric literals to increase readability

// Data Type Notes
/// rust is a statically typed language so it must know all types at compile time
/// type annotations (: type) are required in cases where there can be many storable types

// Scalar Type Notes
/// scalar types are single valued (integers, floating-point, booleans, and characters)
/// integers are numbers without fractional components (i/u 8/16/32/64/128/size)
/// signed integers have a sign in them (-)
/// _ is a visual separator in number literals
/// decimal - 100_000
/// hex - 0xff
/// octal - 0o77
/// binary - 0b1111_0000
/// Byte (u8 only) - b'A'
/// Rust panics if integer overflows occur during compile time
/// Rust wraps around on release builds (256 ib a u8 int becomes 0)
/// floating point numbers are numbers with decimal points (f 32/64)
/// f32 is single precision while f64 is double precision
/// bool type allows true or false values
/// char type is a single value and allows any unicode values ('😻')
/// basic numeric operations include +, -, *, /, %

// Compound Type Notes
/// Compound types group multiple values into one type (tuples and arrays)
/// Tuples types are a general way of grouping together a number of values
/// Tuple values can all be different types
/// Arrays are a collection of multiple values
/// Array values must all be the same type

fn main() {
    // variables are immutable by default
    let mut x = 19; // adding mut before variable name makes it mutable
    println!("Value of X is: {}", x);
    x = 6; // can't reassign without mut keyword variable

    println!("Value of changed X is: {}", x);

    let z = 10; // declares a immutable z and binds it to 10
    let z = z + 10; // old z is shadowed by let z
    let z = z * 2; // old z is operated on and stored in the shadow
                   // allows immutable variables to be transformed and becomes immutable after transformations are finished

    println!("Value of math Z is: {}", z);

    // good for type casting where we need to change the type of the variable but keep the name
    let spaces = "    ";
    let spaces = spaces.len(); //

    // tuple example (with optional type annotations)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // tuple destructuring to access all the values in the tuple
                         // you can directly access tuple values with "tuple.index"
                         // tup.0 is 500

    // array example (with inferred type (all values are the same))
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let a2 = [3; 5]; // same as [3,3,3,3,3]
    let index = 0; // makes index for first value in an array
    let first = a[index]; // accessing array value

    println!("Value of tuple X is: {}", x);
    println!("Value of 1st value in a is: {}", first);
    println!("Value of 2nd value in a2 is: {}", a2[1]);
    println!("Value of Y is: {}", Y_CONST);
    println!("Value of tuple Z is: {}", z);
    println!("Value of spaces is: {}", spaces);
    println!("Values in the tuple are {}, {}, and {}", x, y, z);

    println!("Return of another_function is {}", another_function(6)); // this is an expression
    control_flows();
    chapter_projects();
}

// fn names are declared with snake case (all lowercase separated by underscores)
// functions can be defined anywhere rust doesn't care
// function definitions are statements
fn another_function(x: i32) -> i32 {
    // block ({}) within the function is an expression
    // arguments require a type annotation
    // "->" allows us to declare the return type
    // statements are instructions that perform some action with no return
    // expressions evaluate to a resulting value
    let y = 7 + 2; // "let y = 7 + 2" is a statement and "7 + 2" is an expression because it returns 9

    println!("Value of function input is {}.", x); // these are also expressions
    println!("Value of function input is {}.", y);

    // return value is synonymous with the final expression in the block
    // use "return" keyword to return a value early in a function block
    y // last expression has no semicolon to prevent empty return
}

fn control_flows() {
    let number = 3;

    // if condition
    // condition must always be a bool type
    if number > 2 {
        // when condition is met
        println!("Condition was true");
    } else if number < 4 {
        // when condition 2 is met
        println!("Condition 2 was true");
    } else {
        // if no previous conditions are met
        println!("No conditions were met",)
    }

    // if expressions can be used in let statements
    let condition = true;
    let number = if condition { 5 } else { 6 }; // all arms must match the same return type
                                                // all arms must be the same type in order to verify that its type is valid everywhere number is used
    println!("Number is {}", number);

    let mut counter = 0;

    // result = expression return
    let result = loop {
        counter += 1;

        println!("Run three times!");

        if counter == 3 {
            break counter * 2; // loop expression returns 3 * 2 after break
        }
    };

    println!("Result of loop is {}", result);

    // while loop (countdown)
    let mut number = 3; // creates a mutable variable bound to 3
                        // while the number variable is not equal to 0 do "stuff in {}"
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    } // while loops are cleaner than previous if loop
    println!("...");
    println!("LIFTOFF!!!");

    // while loop (looping through a collection)
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        // for loop is cleaner than this
        // static - array has to have 5 values
        println!("the value is: {}", a[index]);
        index += 1;
    } // took 6 lines

    // for loop (looping through a collection)
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        // iter() method converts an array to an iterator (object that allows you to transverse through a container (compound type))
        // dynamic - array can have any amount of values
        println!("the value is: {}", element);
    } // took 4 lines

    // for loop (better countdown)
    for number in (1..4).rev() {
        // 1..4 is an iterator from 1 to for and at the end of each for loop cycle it goes to the next number
        // rev is a method that reverses the iterators direction starting it at 4 and going backwards
        println!("{}!", number)
    }
    println!("...");
    println!("LIFTOFF!!!");
    // much cleaner
}

fn chapter_projects() {
    // Test 1 - Fahrenheit to Celsius converter
    println!("22.2 Fahrenheit is {} Celsius", f_to_c('f', 22.2));
    println!("55.21 Celsius is {} Fahrenheit", f_to_c('c', 55.21));

    // Test 2 - Generate the nth Fibonacci number
    println!("29th Fibonacci number is {}", fib(29));

    // Test 3 - Print the lyrics to the Christmas carol “The Twelve Days of Christmas”
    sing();
}

fn f_to_c(c: char, v: f64) -> f64 {
    if c == 'f' {
        (v - 32.0) * 0.556
    } else if c == 'c' {
        (v * 1.8) + 32.0
    } else {
        println!("Not a valid operation (f2c).");
        -1.0
    }
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}

fn sing() { // wip
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for day in days.iter() {
        println!("On the {} day of Christmas", day);
        println!("My true love gave to me");
    }
}
