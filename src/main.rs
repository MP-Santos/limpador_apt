use std::fs::OpenOptions;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    // 1. Solicita a lista de programas ao usuário
    print!("Digite o(s) nome(s) do(s) programa(s) separados por espaço: ");
    io::stdout().flush().unwrap();

    let mut entrada_programas = String::new();
    io::stdin()
        .read_line(&mut entrada_programas)
        .expect("Falha ao ler a entrada");

    let programas: Vec<&str> = entrada_programas
        .trim()
        .split_whitespace()
        .collect();

    if programas.is_empty() {
        println!("❌ Erro: Nenhum programa foi informado.");
        return;
    }

    // 2. Menu de opções para atualização do sistema
    println!("\n🔄 Opções de atualização do sistema (apt update && upgrade):");
    println!("   1. Atualizar ANTES da limpeza");
    println!("   2. Atualizar DEPOIS da limpeza");
    println!("   3. Não atualizar (Apenas limpar)");
    print!("Escolha uma opção [1-3]: ");
    io::stdout().flush().unwrap();

    let mut opcao_atualizar = String::new();
    io::stdin()
        .read_line(&mut opcao_atualizar)
        .expect("Falha ao ler a opção");

    let opcao = opcao_atualizar.trim();

    // 3. Tela de confirmação de segurança [s/n]
    println!("\n⚠️  ATENÇÃO: Você solicitou a remoção completa de:");
    for prog in &programas {
        println!("   - {}", prog);
    }

    print!("\nTem certeza que deseja continuar? [s/N]: ");
    io::stdout().flush().unwrap();

    let mut confirmacao = String::new();
    io::stdin()
        .read_line(&mut confirmacao)
        .expect("Falha ao ler a confirmação");

    let confirmacao = confirmacao.trim().to_lowercase();

    if confirmacao != "s" && confirmacao != "sim" {
        println!("❌ Operação cancelada pelo usuário.");
        return;
    }

    // --- Início da Execução ---

    // Executa ANTES se a opção 1 foi escolhida
    if opcao == "1" {
        atualizar_sistema();
    }

    println!("\n🧹 Iniciando o processo de remoção...\n");

    // 4. Executa: sudo apt remove --purge (programa)* -y
    for programa in &programas {
        println!("-> Removendo o pacote e configurações de: {}...", programa);
        let formato_programa = format!("{}*", programa);
        ejecutar_comando("sudo", &["apt", "remove", "--purge", "-y", &formato_programa]);
    }

    // 5. Executa: sudo apt autoremove --purge -y
    println!("\n-> Removendo dependências desnecessárias globais...");
    ejecutar_comando("sudo", &["apt", "autoremove", "--purge", "-y"]);

    // 6. Executa: sudo apt autoclean
    println!("\n-> Limpando o cache do APT...");
    ejecutar_comando("sudo", &["apt", "autoclean"]);

    // Executa DEPOIS se a opção 2 foi escolhida
    if opcao == "2" {
        atualizar_sistema();
    }

    // 7. Busca profunda: sudo find / -iname "*nome_programa*" 2>/dev/null
    println!("\n🔍 Buscando por arquivos restantes em todo o sistema (/) (isso pode demorar)...");

    for programa in &programas {
        println!("\n📂 Arquivos restantes para [{}]:", programa);
        let formato_busca = format!("*{}*", programa);

        let output = Command::new("sudo")
            .args(&["find", "/", "-iname", &formato_busca])
            .output()
            .expect("Falha ao executar o comando find");

        let resultado_busca = String::from_utf8_lossy(&output.stdout);

        if resultado_busca.trim().is_empty() {
            println!("✨ Nenhum arquivo restante foi encontrado na raiz!");
        } else {
            println!("{}", resultado_busca);
        }
    }

    // 8. Salva o histórico no arquivo .txt usando comandos nativos do sistema para a data
    salvar_historico(&programas);

    println!("\n✅ Processo concluído com sucesso e registrado no histórico!");
}

// Função auxiliar para rodar os comandos e exibir a saída no terminal em tempo real
fn ejecutar_comando(comando: &str, argumentos: &[&str]) {
    let status = Command::new(comando)
        .args(argumentos)
        .status()
        .expect("Falha ao executar o comando");

    if !status.success() {
        println!("⚠️ Ocorreu um aviso ou erro ao executar: {} {:?}", comando, argumentos);
    }
}

// Função dedicada para atualizar a lista de pacotes e o sistema
fn atualizar_sistema() {
    println!("\n🚀 Atualizando o sistema (apt update && apt upgrade)...");

    println!("-> Executando apt update...");
    ejecutar_comando("sudo", &["apt", "update"]);

    println!("-> Executando apt upgrade...");
    ejecutar_comando("sudo", &["apt", "upgrade", "-y"]);
}

// Função para gravar no arquivo .txt sem precisar de bibliotecas externas (crates)
fn salvar_historico(programas: &[&str]) {
    // Coleta a data e hora do próprio sistema usando o comando 'date' do Linux
    let output_data = Command::new("date")
        .arg("+%d/%m/%Y %H:%M:%S")
        .output()
        .expect("Falha ao obter a data atual");

    let data_formatada = String::from_utf8_lossy(&output_data.stdout);
    let data_limpa = data_formatada.trim();

    // Formata a linha que será salva no arquivo
    let programas_texto = programas.join(", ");
    let linha_log = format!("[{}] Programas removidos: {}\n", data_limpa, programas_texto);

    // Abre o arquivo para anexar (append) ou cria se ele não existir
    let mut arquivo = OpenOptions::new()
        .create(true)
        .append(true)
        .open("historico_limpeza.txt")
        .expect("Não foi possível abrir ou criar o arquivo de histórico");

    // Escreve a informação no arquivo
    arquivo.write_all(linha_log.as_bytes()).expect("Falha ao gravar no arquivo de histórico");
}