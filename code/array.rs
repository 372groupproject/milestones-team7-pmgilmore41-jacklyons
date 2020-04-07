fn main(){
    let arr1 = [1,2,3,4,5];
    let arr2: [f32; 5] = [1.0,2.0,3.5,4.5,5.8];
    let arr3 = ['A'; 5];

    for e in arr1.iter() {
        println!("{}", e);
    }

    for e in arr2.iter() {
        println!("{}", e);
    }

    for e in arr3.iter() {
        println!("{}", e);
    }
}