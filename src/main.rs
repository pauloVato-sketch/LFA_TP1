/*
    Projeto: Máquina de Turing em Rust
    Disciplina: Linguagens Formais e Autômatos
    Autor: Paulo Lopes do Nascimento e Vinicius Hiago Gonçalves Ribeiro
    Profº: Andrei Rimsa Alvares
*/
// A atributo usado no escopo do arquivo para suprimir warning contra preferência por snake_case
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused_assignments)]
use std::env;
// Importando crate 'serde' para parsing do JSON
extern crate serde;
// Imports utilizados:
// Biblioteca básica para uso de arquivos locais
use std::fs;
// Trait (próximo a interface) para definir objeto como deserializável, relacionado ao parsing JSON.
use serde::Deserialize;
// Trait que define leitura de bytes de uma fonte qualquer.
use std::io::Read;
// Trait que permite parsing de argumentos de linha de comando, inserida por facilidade,
// poderia usar o básico std::end::args
//use clap::Parser;
use std::path::Path;


// Atributo que atribui o parse de linha de comando à esta struct
#[derive(Default/*, Parser*/)]
/* Struct que possui estrutura com os parâmetros a serem inseridos pela linha de comando,
 neste caso o nome do arquivo e a palavra a ser processada*/
struct Cli {
    nomeArquivo : String,
    palavra : String,
}
// Atributo que faz com que o compilador seja capaz de atribuir implementações básicas
// Neste caso, Debug permite que a struct seja impressa, Deserialize indica que pode ser obtida de um JSON,
// Default inclui um construtor vazio para uso e Clone permite realizar cópia de objeto deste tipo.
#[derive(Debug, Deserialize, Default, Clone)]
// Atributo da biblioteca serde que renomeia campos puxados do JSON e da struct para o case indicado.
#[serde(rename_all = "snake_case")]
/* Struct que define uma transição da máquina de Turing
 Contém estado de origem e destino, simbolo atual e simbolo a sobrescever e a direcao do cabeçote.*/
struct Transicao {
    estadoOrigem : String,
    simboloConsumido : String,
    estadoDestino : String,
    simboloDeTroca : String,
    direcaoCabecote : String
}
// Ver linhas 38- 40
#[derive(Debug, Deserialize)]
// Ver linha 42
#[serde(rename_all = "snake_case")]
/* Struct que define uma máquina de Turing de acordo com a base definida no enunciado em (1),
 Contém um vec com os estados, vec com o alfabeto de entrada e da fita (ver (2)),
 marcadores de início e vazio, vec de transições possíveis, estado inicial e vec de estados finais.
*/
struct MaquinaTuring {
    vecEstados : Vec<String>,
    alfabetoEntrada : Vec<String>,
    alfabetoFita :  Vec<String>,
    marcadorInicio : String,
    marcadorVazio : String,
    vecTransicoes : Vec<Transicao>,
    estadoInicial : String,
    vecEstadosFinais : Vec<String>
}
// Ver linhas 38- 40
#[derive(Debug, Deserialize)]
// Ver linha 42
#[serde(rename_all = "snake_case")]
/* Struct que define um autômato geral, neste caso uma máquina de turing apenas
 Sua criação se deve ao fato da maneira de como o JSON foi criado/funcionamento da biblioteca, 
 objeto 'mt' é considerado por estar dentro de um outro objeto*/
struct Automata {
    mt : MaquinaTuring
}
impl Automata{
    /*  Método de processamento da palavra de uma Máquina de Turing
        Argumentos : &self: Automata = instância do autômato,
                     palavra : String = palavra a ser processada
        Retorno: string Sim ou Não
    */
    fn processaPalavraMT(&self, palavra : String)-> String {
        // Variável de resultado, usando o menor tipo de inteiro disponível 8 bits, 0 = Não e 1 = Sim
        let mut res : i8 = 0;
        // Extrai objeto MaquinaDeTuring do objeto Automata.
        let machine = &self.mt;
        // Define estadoAtual como variável mutável e inicializa com o estado inicial proveniente da máquina.
        let mut estadoAtual : String = machine.estadoInicial.to_string();
        // Declara fita como sendo marcador de ínicio concatenado com a palavra a ser processada.
        let mut fita : String = machine.marcadorInicio.to_string()+&palavra.clone();
        // Define iter como índice do cabeçote a ser movido para a direita e esquerda, devido ao item de ínicio,
        // o processamento como no segundo índice, 1.
        let mut iter = 1;
        // Variável que encerra o loop infinito.
        let mut breakOut = 0;
        let mut noLoop = 0;
        if fita.len() == 1{
            if machine.vecEstadosFinais.contains(&estadoAtual) {
                res = 1;
            }else {
                res = 0;
            }
            noLoop = 1;
        }

        // Loop infinito
        if noLoop == 0 {
            // Primeiro símbolo (char) a ser avaliado da fita.
            let mut symbol = fita.chars().nth(iter).unwrap();
            loop {
                // Define transicao a ser tomada, inicialmente como objeto vazio.
                //println!("Simbolo: {}",symbol);
                let mut transicaoASerTomada = Transicao::default();
                println!(" Estado atual : {:?}", estadoAtual);
                for trans in machine.vecTransicoes.iter() {
                    // Verifica se o simbolo é o marcador de inicio, se for, verifica se o simbolo a s
                    if (trans.estadoOrigem == estadoAtual) && (trans.simboloConsumido == symbol.to_string()){
                        if trans.simboloConsumido.to_string() != machine.marcadorInicio{
                            transicaoASerTomada = trans.clone();
                            breakOut=0;
                            break;
                        } else if trans.simboloDeTroca != machine.marcadorInicio {
                            breakOut = 1; break;
                        } else if trans.simboloConsumido.to_string() == machine.marcadorInicio &&
                            trans.simboloDeTroca != machine.marcadorInicio {

                        }else{
                            if trans.direcaoCabecote == "<" {
                                // Inválida
                            }else {
                                transicaoASerTomada = trans.clone();
                                breakOut=0;
                                break;
                            }
                        }
                    }else {
                        breakOut = 1;
                    }
                }
                println!("{:?}",transicaoASerTomada);
                if breakOut == 0 {
                    estadoAtual = transicaoASerTomada.estadoDestino;
                    //println!("EstadoAtual: {}",estadoAtual);
                    fita.replace_range(iter..iter+1,&transicaoASerTomada.simboloDeTroca);
                    //println!("{}",fita);
                    if transicaoASerTomada.direcaoCabecote == ">"{
                        iter = iter + 1;
                    }else {
                        iter = iter - 1;
                    }
                    //println!("CabeçotePos : {}",iter);
                }else if breakOut == 1 && machine.vecEstadosFinais.contains(&estadoAtual){ 
                    res = 1;
                    break;
                }else if breakOut == 1 && !machine.vecEstadosFinais.contains(&estadoAtual){
                    res = 0;
                    break;
                }
                if iter >= fita.len(){
                    fita.push_str(&machine.marcadorVazio);
                }
                symbol = fita.chars().nth(iter).unwrap();
            }
        }
        return if res == 1 { "Sim".to_string() } else {"Não".to_string()};
    }
}
/*  Função que lê o arquivo e retorna o autômato 
    Argumentos : filename : String = Nome do arquivo JSON
    Retorno: objeto do tipo Automata
*/
fn leArquivo(filename : String) -> Automata {
    // Declara variável para concatenar o nome do arquivo com estrutura de diretório do projeto.
    let mut directory : String= "testFiles/".to_owned();
    directory.push_str(&filename);
    println!("{:?}",directory);
    println!("{}",Path::new(&directory).is_file());
    // Declara objeto do arquivo, tentando abrir o caminho especificado, exibindo mensagem
    // "Arquivo não encontrado" caso não consiga proceder com a operação.
    let mut file = fs::File::open(directory).expect("Arquivo não encontrado");
    // Declara buffer para arquivo
    let mut buff = String::new();
    // Lê arquivo JSON para o buffer
    file.read_to_string(&mut buff).unwrap();
    //println!("{}",buff);
    // Monta o autômato como objeto retornado da string no formato JSON.
    let completeAutomata: Automata = serde_json::from_str(&buff).unwrap();
    return completeAutomata;
}

// Atributo para remover warnings sobre imports não utilizados
#[warn(unused_imports)]
// Função main
fn main() {
    // Recebe objeto Cli com os argumentos da linha de comando.
    let mut args = Cli::default();
    args.nomeArquivo = env::args().nth(1).unwrap_or("".to_string());
    args.palavra = env::args().nth(2).unwrap_or("".to_string());
    if args.nomeArquivo.is_empty() {
        println!("Usar: ./mt [MT] [Word]")
    }else {
        // Recebe objeto Automata da função de leitura de arquivo
        let auto : Automata = leArquivo(args.nomeArquivo);
        println!("{}", auto.processaPalavraMT(args.palavra))
    }
    

}
