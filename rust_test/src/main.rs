fn main() {
    // let nota1: f32 = 10f32;
    // //é um cast onde eu digo que ele será salvo no mesmo formato de um f32
    // let nota2: f32 = 8f32;
    // let nota3: f32 = 9.5;
    // let nota4: f32 = 6.0;

    // //array
    // let notas: [f32; 4] = [10f32, 8f32, 9.5, 6.0];

    // println!(
    //     "Nota 1: {}, Nota 2:{}, Nota 3 = {}, Nota 4 = {}",
    //     notas[0], notas[1], notas[2], notas[3]
    // );

    // let notas: [f32; 4] = [10f32, 8f32, 9.5, 6.0];
    let notas: [f32; 4] = [6.5; 4];
    let inteiro: usize = 0;

    println!("{}", notas[inteiro]);
    for indice in 0..notas.len() {
        println!("Nota {} é = {}", indice + 1, notas[indice])
    }

    matriz();
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

    for linha in matriz {
        for elemento in linha {
            println!("Elemento da matriz = {}", elemento);
        }
    }
}
