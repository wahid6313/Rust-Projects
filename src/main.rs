use std::string;

fn main() { 

    // number-------------------------------
    let mut x: i32 = 5;  
    
    for i in 0..1000 {
        x = x + 100;
    }
    println!("x: {}", x);

    //boolean-------------------------------
    let is_male = true;
    let is_above_18 = true;

    if is_male {
        println!("you are a male")
    }
    else {
        println!("you are not a male")
    }

    if is_male && is_above_18 {
        println!("you are a legal male")
    }

    //String--------------------------------
    let greeting = String::from("hello world");
    println!("{}", greeting);

    let char1 = greeting.chars().nth(1);  //char is an option it does not a correct process.
    println!("char1: {}", char1.unwrap());
    
    //if else statement----------------------
    let is_even = true;

    if(is_even) {
        println!("the number is even")
    }else if !is_even {
        println!("the number is odd")
    }

    //sum of two numbers---------------------
    let a = 10;
    let b = 20;

    let sum = do_sum(a, b);
    println!("sum of {} and {} is :  {}", a, b , sum);


    //iterate over a string------------------
    let sentence = String::from("my name is wahid ali");
    let first_word = get_first(sentence);

    println!("first word is: {first_word}");
    
    // print_lebel(5, 'h');

    // let wahid = String::from("my name is wahid ");
    // for _ in 0..10 {
    //     // wahid = wahid + " " + "raza";
    //     println!("{}", wahid);
    // }
    
}

//function os sum of two number---------------
fn do_sum(a: i32, b: i32) -> i32 {
  return a + b ;
}



//function of iterate over a string----------
fn get_first(sentence: String) -> String {
    let mut  ans = String::from("");
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());

        if char == ' ' {
            break;
        }
    }

    return ans;
}

// fn print_lebel(value: i32, unit_lebel: char) {
//     println!("the value of that is: {value}{unit_lebel}")
// }


