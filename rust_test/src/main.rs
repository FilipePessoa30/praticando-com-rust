use std::{option, string};

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
    println!(
        "É fim de semana ? {}",
        eh_fim_de_semana(diaDaSemana::Quarta)
    );

    let dia: diaDaSemana = diaDaSemana::Sexta;

    cores();

    conteudoOpcional();

    vetores();

    conta_corrente();
}

#[allow(dead_code)]
enum diaDaSemana {
    Domingo,
    Segunda,
    Terca,
    Quarta,
    Quinta,
    Sexta,
    Sabado,
}

fn eh_fim_de_semana(dia_de_semana: diaDaSemana) -> bool {
    match dia_de_semana {
        diaDaSemana::Domingo | diaDaSemana::Sabado => true,
        _ => false,
    }
}

fn matriz() {
    let matriz: [[f32; 3]; 2] = [[0.0, 1.2, 0.1], [1.3, 0.3, 1.4]];

    for linha in matriz {
        for elemento in linha {
            println!("Elemento da matriz = {}", elemento);
        }
    }
}

#[allow(dead_code)]
enum Color {
    Red,
    Green,
    Blue,
    RgbColor(u8, u8, u8), //tuple
    CymkColor {
        cyan: u8,
        magenta: u8,
        yellow: u8,
        black: u8,
    },
}

fn cores() {
    let cor: Color = Color::CymkColor {
        cyan: 100,
        magenta: 50,
        yellow: 70,
        black: 255,
    };

    println!(
        "Cor = {}",
        match cor {
            Color::Red => "Vermelho",
            Color::Green => "Verde",
            Color::Blue => "Azul",
            Color::RgbColor(0, 0, 0)
            | Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: 255,
            } => "Preto",
            Color::RgbColor(_, _, _) => "RGB desconhecido",
            Color::CymkColor {
                cyan: _,
                magenta: _,
                yellow: _,
                black: _,
            } => "CMYK desconhecido",
        }
    );
}

//É um tipo opcional ou mônada
fn conteudoOpcional() {
    let conteudo_arquivo = ler_arquivo(String::from(""));
    match &conteudo_arquivo {
        Some(valor) => println!("{}", valor),
        None => println!("Não foi possível ler o arquivo"),
    };

    //debugg no rust
    println!("{:?}", &conteudo_arquivo);

    //tanto o if como while let podem ser usados, por baixo dos panos ele usa o Pattern Matching
    if let Some(valor) = conteudo_arquivo {
        println!("Agora, tenho certeza de que há o valor {}", valor);
    }
}

//Estou criando um template para que ele crie enum usando o T
// enum Option<T> {
//     Some(T),
//     None,
// }

fn ler_arquivo(caminho_arquivo: String) -> Option<String> {
    Some(String::from("Conteudo do arquivo"))
    // Option<u8>
}

fn vetores() {
    //Usando macros, usando essa macro quando vocÊ tiver um vetor de 3 e adicionar por push um valor o tamanho
    //acaba sendo duplicado, é melhor usar esse quando você tiver uma ideia do tamanho do vetor
    // let mut notas: Vec<f32> = vec![10.0, 8.8, 6.5];
    //outra forma de criar um vetor
    // let mut notas: Vec<f32> = Vec::new();
    //mais uma forma de criar um vetor
    let mut notas: Vec<f32> = Vec::with_capacity(4);

    notas.push(10.0);
    notas.push(8.8);
    notas.push(6.5);

    println!("Capacidade = {}", notas.capacity());

    println!("{:?}", notas);
    notas.push(6.8);
    println!("Capacidade = {}", notas.capacity());
    println!("{:?}", notas);

    println!("Nota 1 = {}", notas[0]);

    println!(
        "Nota 6 = {}",
        match notas.get(7) {
            //o * significa que foi passado por referência
            Some(n) => *n,
            None => 0.0,
        }
    );

    //exibe e remove o último valor
    //notas.pop();
    if let Some(nota) = notas.pop() {
        println!("Último valor = {}", nota);
        println!("{:?}", notas);
    }

    //usando o while para remover elementos do vetor
    // while let Some(nota) = notas.pop() {
    //     println!("Valor removido = {}", nota);
    // }
    // println!("{:?}", notas);

    //Se for usar a variável em outro lugar adicione & para passar como referência
    for nota in &notas {
        println!("Nota = {}", nota)
    }
    println!("{:?}", notas);
}

struct Conta {
    titular: Titular,
    saldo: f64,
}
//Eu tenho a estrutura, e dentro dessa estrutura eu posso ter implementações(impl)
impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor;
    }
}

struct Titular {
    nome: String,
    sobrenome: String,
}

fn conta_corrente() {
    let mut conta: Conta = Conta {
        titular: (Titular {
            nome: String::from("Vinicius"),
            sobrenome: String::from("Dias"),
        }),
        saldo: (100.0),
    };
    // let titular_conta: String = String::from("Vinicius Dias");
    // let saldo_conta: f64 = 100.0;

    conta.sacar(50.0);

    println!(
        "Dados da conta: Titular = {} {}, Saldo = {}",
        conta.titular.nome, conta.titular.sobrenome, conta.saldo
    );
}
