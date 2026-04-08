// Using a hash map and vectors, create a text interface to allow a user to add employee names
// to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.”
// Then, let the user retrieve a list of all people in a department or all people in the company by
// department, sorted alphabetically.

use std::collections::HashMap;
use std::io;

enum Action {
    Update,
    Look,
    Exit,
}

fn main() {
    let mut employes: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let user_aciton: Action = get_action();

        match user_aciton {
            Action::Update => {
                let department = get_department(&employes);
                let person: String = get_person(&department);
                let depart_employes = employes.entry(department).or_insert(Vec::new());
                depart_employes.push(person);
            }
            Action::Look => {
                // println!("{:#?}", employes);
                data_output(&employes);
            }
            Action::Exit => break,
        }
    }
}

fn data_output(employes: &HashMap<String, Vec<String>>) {
    println!("Вывести сотрудников одного отдела: 1,");
    println!("Вывести сотрудников всего предприятия: 2,");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Wrong input!");

    match input.trim() {
        "1" => department_output(&employes),
        "2" => company_output(&employes),
        _ => {
            println!("Неверное задание. Вывод всего предприятия.");
            company_output(&employes);
        }
    }
}

fn company_output(employes: &HashMap<String, Vec<String>>) {
    let mut company_persons = Vec::new();
    for department in employes.values() {
        company_persons.extend(department);
    }
    company_persons.sort();
    println!("\n---COMPANY---");
    for person in company_persons {
        println!("{person}");
    }
    println!("---END---\n");
}

fn department_output(employes: &HashMap<String, Vec<String>>) {
    println!("Выберите отдел для вывода сотрудников.");
    for depart in employes.keys() {
        println!("{}", depart);
    }
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Wrong input!");
    let dep = input.trim();
    let persons = employes.get(dep);
    match persons {
        Some(data) => {
            println!("\n---{}---", dep);
            let mut data_out = data.clone();
            data_out.sort();
            for person in data_out {
                println!("{person}")
            }
            println!("---END---\n");
        }
        None => println!("Неверно указан отдел!"),
    }
}

fn get_person(department: &str) -> String {
    println!("Добавьте фамилию сотрудника в отдел {}.", department);
    let mut input_empl = String::new();
    io::stdin()
        .read_line(&mut input_empl)
        .expect("Wrong input.");

    input_empl.trim().to_string()
}

fn get_department(employes: &HashMap<String, Vec<String>>) -> String {
    println!("Выберите отделение или создайте новое.");
    for key in employes.keys() {
        println!("{key}");
    }
    let mut department = String::new();
    io::stdin()
        .read_line(&mut department)
        .expect("Uncorrekt input!");
    department.trim().to_string()
}

fn get_action() -> Action {
    loop {
        let mut input = String::new();

        println!("Выберите действие:");
        println!("1: Добавление работника в список,");
        println!("2: Просмотр списка работников.");
        println!("q: Завершение работы программы.");
        io::stdin().read_line(&mut input).expect("Wrong input!");

        match input.trim() {
            "1" => return Action::Update,
            "2" => return Action::Look,
            "q" => return Action::Exit,
            _ => {
                println!("-----------------------------");
                println!("Выбрано недопустимое действие!\n");
                continue;
            }
        };
    }
}
