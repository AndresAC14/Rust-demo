fn main() {
    println!("Hello, world!");

    let numero = 10;

    println!("illo {} ke dise", numero);

    mutables();
}

fn mutables(){
    let shadow_num = 5;
    // para que una variable sea mutable añadir mut entre let y el nombre de la var
    let shadow_num = shadow_num + 5;

    let shadow_num = shadow_num * 2;

    println!("The number is {}", shadow_num);
}
