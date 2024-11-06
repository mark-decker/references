//mark decker 6/11/2024
//following rust-lang references chapter
//made a function to get the second word using slices
//
fn main() {
    //let mut s = String::from("hello");

    //let r0 = &s;  //can create immuatable reference
    //println!("r0 is {r0}");


    //change(&mut s); 

    //println!("s is: {s}");

    //let r1 = &mut s; //can create one mutable reference and one only
    //                 //previous r0 refernce cannot be used now
    //                 //
    ////change(&mut r1);  Cannot call with a borrowed mutable

    let mut s = String::from("hello world how are you");

    let word = first_word(&s); // word will get the value 

    println!("word is {word}");

    let second = second_word(&s);

    println!("second is {second}");

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}

//fn change(some_string: &mut String) {
//    some_string.push_str(", world");
//}

//return refence to the slice we want
fn first_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        println!("i is {i} {item}");
        if item == b' ' {
            return &s[0..i];
        }
    }

    return &s[..]
}

fn second_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return first_word(&s[i+1..]);
        }
    }

    return &s[..]
}



