fn main() {
    println!("Hello, world!");

    let numero = 10;

    println!("illo {} ke dise", numero);

    mutables();

    tuplas();
    /*
    Los datos de cadena que se almacenan dentro de otra
    estructura de datos, como una estructura o un vector,
    se deben convertir de una referencia literal de 
    cadena (&str) a un tipo String. Para realizar la 
    conversión, se usa el método String::from(&str) 
    estándar. Tambien se puede usar .toString()
    
    // Classic struct with named fields
    struct Student { name: String, level: u8, remote: bool }
    
    let user_2 = Student { name: String::from("Dyson Tan"), level: 5, remote: false };
    */
}

fn mutables(){
    let shadow_num = 5;
    // para que una variable sea mutable añadir mut entre let y el nombre de la var
    let shadow_num = shadow_num + 5;

    let shadow_num = shadow_num * 2;

    println!("The number is {}", shadow_num);
}

fn tuplas(){
    // Declare a tuple of three elements
    let tuple_e = ('E', 5i32, true);

    // Use tuple indexing and show the values of the elements in the tuple
    println!("Is '{}' the {}th letter of the alphabet? {}", tuple_e.0, tuple_e.1, tuple_e.2);
}