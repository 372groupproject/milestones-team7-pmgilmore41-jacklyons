fn main(){
    let truth: bool = true;

    let fallacy: bool = false;
    
    if truth {
        println!("truthful");
        if fallacy {
            println!("super truth");
        }
        else {
            println!("Not that truthful");
        }
    }

}