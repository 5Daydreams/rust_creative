fn main() 
{
    // const NAME_CONVENTION: type = constValue;
    const FEW_DIGITS_OF_PI: f32 = 3.15;
    println!("Pi starts with: {FEW_DIGITS_OF_PI}");

    // let varName = varValue;
    // OR
    // let mut (to indicate mutability) varName = varValue;
    let x:i128 = 5000;
    let x:i128 = x + 1;
    println!("The value of x is: {x}");

    // shadowing -> you can CHANGE the variable type by using `let varName` again within the same scope or a different one!
    // I can only imagine how much more readable and flexible this structure will become later.
    {
        let x: i128 = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let guess: u32 = "42".parse().expect("Not a number!");

    println!("guess is: {guess}");

    print!("Array time: ");

    const ARRAY_SIZE: usize = 2;

    // Remember the syntatic cohesion: `let varName:varType = varValue;`
    // the RHS expression denotes `[repeat this value; for this many indices]`
    let b: [char; ARRAY_SIZE] = ['f'; ARRAY_SIZE];

    for item in b 
    {
        print!("{} ",item);
    }

}