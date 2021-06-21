use std::io;
fn main() {
    println!("Hello Welcome to this Adjoint Solver");
    println!("Just type the elements of a matrix line by line and we will find it ");

    let mut array :[i32 ; 9];
    array = [0;9];
    for num in 0..9 {
        let mut buffer_number = String::new();
        io::stdin().read_line(&mut buffer_number).expect("Failed to read line");
        let guess: i32 = buffer_number.trim().parse().unwrap();
        array[num] = guess;
    }
    array_printer(&mut array);
    println!("determinant is -> {}",deter(&mut array));
    array_printer(&mut cofactor(&mut array));
    array_printer(&mut adjoint(&mut array));
}
fn array_printer(in_array : &mut [i32;9]){
    println!("[ {} {} {} ]",in_array[0],in_array[1],in_array[2]); 
    println!("[ {} {} {} ]",in_array[3],in_array[4],in_array[5]); 
    println!("[ {} {} {} ]",in_array[6],in_array[7],in_array[8]); 
}
fn cofactor(in_array : &mut [i32;9]) -> [i32;9]{
   let mut coftr_array :[i32;9]; // defining the array we are gonna use
   coftr_array = [0;9]; // setting all the initial values of the array to 0;

   coftr_array[0] = (in_array[4] * in_array[8]) -(in_array[5] * in_array[7]);
   coftr_array[1] = -1*((in_array[3] * in_array[8]) -(in_array[5] * in_array[6]));
   coftr_array[2] = (in_array[3] * in_array[7]) -(in_array[4] * in_array[6]);
   coftr_array[3] = -1*((in_array[1] * in_array[8]) -(in_array[2] * in_array[7]));
   coftr_array[4] = (in_array[0] * in_array[8]) -(in_array[2] * in_array[6]);
   coftr_array[5] = -1*((in_array[0] * in_array[7]) -(in_array[1] * in_array[6]));
   coftr_array[6] = (in_array[1] * in_array[5]) -(in_array[2] * in_array[4]);
   coftr_array[7] = -1*((in_array[0] * in_array[5]) -(in_array[2] * in_array[3]));
   coftr_array[8] = (in_array[0] * in_array[4]) -(in_array[1] * in_array[3]);
   return coftr_array;
}
fn adjoint(in_array : &mut [i32;9]) -> [i32;9]{
    let mut in_array = in_array;
   let mut in_array = cofactor(&mut in_array);

    let adj_array :[i32;9]; // defining the array we are gonna use
    adj_array = in_array;
    in_array[1] = adj_array[3];
    in_array[2] = adj_array[6];
    in_array[5] = adj_array[7];
    in_array[3] = adj_array[1];
    in_array[6] = adj_array[2];
    in_array[7] = adj_array[5];

    return in_array;
    }
fn deter(in_array : &mut [i32;9]) -> i32{
    let mut result :i32= 0; // setting all the initial values of the array to 0;
    result = (in_array[0]*(in_array[4]*in_array[8]-in_array[5]*in_array[7])
                -(in_array[1]*(in_array[3]*in_array[8]-in_array[5]*in_array[6]))
                +(in_array[2]*(in_array[3]*in_array[7]-in_array[4]*in_array[6])));
    return result;
}