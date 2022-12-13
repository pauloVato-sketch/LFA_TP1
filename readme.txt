# Máquina de Turing em Rust

Trabalho prático referente à disciplina do 5º período do curso de Engenharia de Computação, Linguagens Formais e Autômatos (LFA).
Alunos: Paulo Lopes do Nascimento e Vinicius Hiago Gonçalves Ribeiro
Professor: Andrei Rimsa Álvares

## Instalação
Este projeto requer [Rust].

A forma de instalar Rust difere de acordo com a plataforma:
Para instalar o compilador do rust em um sistema Unix/Mac, basta executar o seguinte comando (requer [curl]):

```
curl https://sh.rustup.rs -sSf | sh
```
On Windows, faça download e execute o arquivo rustup-init.exe [Rust] (para escolha do arquivo em relação a sua arquitetura, veja https://forge.rust-lang.org/infra/other-installation-methods.html).

## Execução
Para compilar o projeto, digite o seguinte comando na raiz do projeto (nível deste arquivo):
```
cargo build
```
Para executar o projeto, segue o seguinte:
- Coloque os arquivos de teste dentro da pasta 'testFiles'.
- Navegue até a raiz do projeto (nível deste arquivo), e execute o comando:
```
cargo run -- <nome_arquivo> <palavra_teste>
```
Exemplo:
```
cargo run -- mt.json "abba"
```
