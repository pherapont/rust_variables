// Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then, let the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.

use core::num;
use std::collections::HashMap;
use std::io;

// TODO добавить Action::Exit

enum Action {
    Update,
    Look,
    Exit,
}

fn main() {
    let mut employes: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let user_aciton: Action = update_or_look();

        match user_aciton {
            Action::Update => {
                let department = get_department(&employes);
                let person: String = get_person(&department);
                let depart_employes = employes.entry(department).or_insert(Vec::new());
                depart_employes.push(person);
            }
            Action::Look => {
                println!("{:#?}", employes);
            }
            Action::Exit => break,
        }
    }
}

fn get_person(department: &str) -> String {
    "Иванов".to_string()
}

fn get_department(employes: &HashMap<String, Vec<String>>) -> String {
    "Engineering".to_string()
}

// TODO создать цикл обработки ввода и спаршивать пока не будет правильный ввод (while)

fn update_or_look() -> Action {
    loop {
        let mut input = String::new();

        println!("Выберите действие:");
        println!("1: Добавление работника в список,");
        println!("2: Просмотр списка работников.");
        println!("q: Завершение работы программы.");
        io::stdin().read_line(&mut input).expect("Wrong input!");
        println!("{}", input);

        let input: u32 = input.trim().parse(){
            Ok(num) => num,

        }
    }


    match input.trim() {
        "1" => Action::Update,
        "2" => Action::Look,
        "q" => Action::Exit,
        _ => Action::Exit,
    }
}
