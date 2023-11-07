
const PI:f32 = 3.14;
static variavel_global:u8  =1;
static mut GLOBAL:u8  =1;

fn main() {
    printvariables();
}

fn printvariables(){
    println!("PI {}", PI);

    let idade:u8 = 130;
    println!("Idade {}, tamanho {} bytes", idade, std::mem::size_of_val(&idade));

    let valor:f32 = 2.5;
    println!("Valor {}, tamanho {} bytes", valor, std::mem::size_of_val(&valor));

    let vf:bool = false;
    println!("Boleana {}, tamanho {} bytes", vf, std::mem::size_of_val(&vf));

    let mut idadeMutavel:u8 = 130;
    idadeMutavel = 100;
    println!("Idade mutavel {}, tamanho {} bytes", idadeMutavel, std::mem::size_of_val(&idadeMutavel));

    let vChar:char = 'C';
    println!("Caracte {}, tamanho {} bytes", vChar, std::mem::size_of_val(&vChar));

    println!("Variavel global {}, tamanho {} bytes", variavel_global, std::mem::size_of_val(&variavel_global));

    unsafe{
        GLOBAL = 2;
        println!("Variavel GLOBAL {}, tamanho {} bytes", GLOBAL, std::mem::size_of_val(&GLOBAL));
    }
}
