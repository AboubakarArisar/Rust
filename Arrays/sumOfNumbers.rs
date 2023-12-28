fn main(){
    let array = [1,2,4,5,6,7];
    let  mut sum = 0;
    for i in 0..array.len() {
        sum+=array[i];
    }

    println!("sum of values in the array is {}",sum);
}


