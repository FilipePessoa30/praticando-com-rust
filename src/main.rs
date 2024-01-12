fn soma(a: i32, b: i32) -> i32 {
    println!("{} + {} = {}", a, b, a + b);
    //Quando eu quero retornar algo em RUST, é necessário colocar a expressão sem ";"
    a + b
}

const PI: f32 = 3.14;
static mut GLOBAL: u8 = 1;
//static é usado para variáveis global
//variaveis globais podem ficar fora da função e vai ser chamada

fn sombra() {
    let a = 123;

    {
        let b = 456;
        println!("dentro, b ={}", b);

        let a = 789;
        println!("dentro, a = {}", a);
        //dentro vai ter um valor e fora outro
        //o nome disso é shadowing
    }

    println!("fora, a = {}", a);
}
fn escopo() {
    unsafe {
        println!("Variável global = {}", GLOBAL);
    }
    //unsafe aqui é usado para impedir o warning que é apresentando ao usar o mut na variavel global

    //Diferença de variável para constante, na variável há um endereço de memória
    //Já a constante não tem endereço de memória, ao ser executado, sua chamada se transforma no valor

    let _essa_string: &'static str = "meu nome";
    //em Rust essa string é um pedaço de um vetor de caracteres, logo não
    //consegue pegar uma parte dessa string

    println!("PI = {}", PI);
    //i8 é um inteiro de 8bits
    //u8 é um unsight de 8bits vai de 0 a 255
    //i32 é um inteiro de 32bits
    let variavel: i32 = 128;
    println!(
        "Variável = {}, tamanho = {} bytes",
        variavel,
        std::mem::size_of_val(&variavel)
    );

    //Você pode chamar a mesma variável duas vezes, caso não use uma das duas, aparece um warning
    //Se você quiser evitar o warning deve utilizar ela pelo menos uma vez

    let variavel: i32 = 120;
    println!(
        "Variável = {}, tamanho = {} bytes",
        variavel,
        std::mem::size_of_val(&variavel)
    );

    //f32 é um float de 32bits
    let decimal: f32 = 2.5;
    println!("decimal = {}", decimal);

    //ao inserir mut a variavel pode ser modificada, do contrário ela é imutável

    //let mut booleana: bool = false;
    let booleana: bool = false;
    println!(
        "Booleana = {}, Tamanho da Booleana = {}",
        booleana,
        std::mem::size_of_val(&booleana)
    );

    let letra: char = 'C';
    println!("Tamanho do char = {}", std::mem::size_of_val(&letra));
}

fn condicionais() {
    let idade: u8 = 17;
    let responsavel_autorizou = true;
    let eh_maior = idade >= 18;

    // if idade > 18 || responsavel_autorizou && idade > 16 {
    //     println!("Pode entrar na balada!")
    // }else {
    //     println!("Não pode entrar na balada!")
    // }
    if eh_maior {
        println!("Pode entrar na balada!");
    } else if responsavel_autorizou && idade >= 16 {
        println!("Pode entrar com assinatura do responsável");
    } else {
        println!("Não pode entrar na balada!");
    }

    let condicao;

    if idade >= 18 {
        condicao = "maior"
    } else {
        condicao = "menor"
    }

    //outra forma de escrever
    //let condicao = if eh_maior { “maior” } else { “menor” };

    println!("Você é {} de idade", condicao);

    //Patch matching, quando associamos uma coisa a outra

    let linguagem = "Kotlin";
    let proposito = match linguagem {
        "PHP" => "Web",
        "Kotlin" => "Android",
        "Python" => "Data Science",
        _ => "Desconhecido",
    };

    println!("O proposito de {} eh {}", linguagem, proposito);
}

fn repeticoes() {
    let multiplicador: u8 = 5;

    let mut contador: u8 = 0;

    while contador < 10 {
        contador += 1;

        if contador == 5 {
            continue;
        }

        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );
    }

    contador = 0;
    loop {
        contador += 1;
        println!(
            "{} x {} = {}",
            multiplicador,
            contador,
            multiplicador * contador
        );

        if contador == 10 {
            break;
        }
    }

    for i in 1..11 {
        println!("{} x {} = {}", multiplicador, i, multiplicador * i);
    }
}

fn main() {
    escopo();
    //println!("decimal = {}", decimal);
    //Essa função de print não funciona porque está sendo chamada fora da função main
    sombra();
    println!("Soma = {}", soma(2, 2));

    condicionais();

    repeticoes();

    ownership();

    pattern_matching();

    erros();
}

//Passou a variável por referência
fn ownership() {
    let mut uma_string = String::from("Vinicius");
    rouba(&mut uma_string);
    //String é um tipo de dado que é alocado na memória heap
    println!("{}", uma_string);
}
//Borrowing é quando você passa uma referencia mutavel com o intuito de mudar uma variavel
fn rouba(string: &mut String) {
    //Adicionado outra mensagem na passagem por referencia
    string.push_str(" Dias");
    println!("{}", string);
}

// //Foi criado uma outra variável para que ele consiga exibir duas vezes
// fn ownership() {
//     let uma_string = String::from("Vinicius");
//     let outra_string = rouba(uma_string);
//     //String é um tipo de dado que é alocado na memória heap
//     println!("{}", outra_string);
// }

// fn rouba(string: String) -> String {
//     println!("String = {}", string);

//     string
// }

fn pattern_matching() {
    for x in 1..=20 {
        println!(
            "{}:{}",
            x,
            match x {
                1 => "Pouco",
                2 | 3 => "Um pouquinho",
                4..=10 => "Um bocado",
                _ if x % 2 == 0 => "Uma boa quantidade",
                _ => "Muito",
            }
        )
    }
}

fn erros() {
    // //Exibe um erro
    // panic!("Erro proposital");
    match resultado() {
        Ok(s) => println!("String de sucesso = {}", s),
        Err(numero) => println!("Codigo de erro = {}", numero),
    }
}

fn resultado() -> Result<String, u8> {
    Ok(String::from("Tudo deu certo"))
}

