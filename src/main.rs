use std::env;

struct Tarefa {
    nome: String,
    feita: char
}

impl Tarefa {
    fn new() -> Tarefa {
        let tarefa = Tarefa {
            nome: String::new(),
            feita: ' '
        };
        return tarefa;
    }
}


struct ListaTarefas {
    lista: Vec<Tarefa>
}

impl ListaTarefas {
    fn new() -> ListaTarefas {
        ListaTarefas {
            lista: Vec::new(),
        }
    }

    fn listar(&self) {
        for tarefa in &self.lista {
            println!("[{}] {}", tarefa.feita, tarefa.nome);
        }
    }

    fn add(&mut self, nome: String) {
        let mut tarefa = Tarefa::new();
        tarefa.nome = nome;
        self.lista.push(tarefa);
    }

    fn done(&mut self, index: usize) {
        if self.lista[index].feita == ' ' {
            self.lista[index].feita = 'x';
        } else {
            self.lista[index].feita = ' ';
        }
    }

    fn rem(&mut self, index: usize) {
        self.lista.remove(index);
    }
}

enum Command {
    List,
    Add(String),
    Done(usize),
    Remove(usize),
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Você deve prover um argumento [list, add, done, rm]");
        return;
    }

    let command = match args[1].as_str() {
        "list" => Command::List,
        "add"  => Command::Add(args[2].clone()),
        "done" => Command::Done(args[2].parse()
            .expect("Erro na conversão para inteiro")),
        "rm"   => Command::Remove(args[2].parse()
            .expect("Erro na conversão para inteiro")),
        _      => panic!("Você deve digitar um argumento válido")
    };

    let mut tarefas = ListaTarefas::new();
    tarefas.add("Terminar esse programa".to_string());
    tarefas.add("Pensar".to_string());
    tarefas.lista[1].feita = 'x';
    tarefas.add("Alguma outra tarefa".to_string());

    match command {
        Command::List => tarefas.listar(),
        Command::Add(tarefa)  => {
            tarefas.add(tarefa);
            tarefas.listar();
        },
        Command::Done(index) => {
            tarefas.done(index);
            tarefas.listar();
        },
        Command::Remove(index) => {
            tarefas.rem(index);
            tarefas.listar();
        }
    }
}
