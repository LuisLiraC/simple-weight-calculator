use std::io;

struct Astro {
    name: &'static str,
    gravity: f32,
}

static OPTIONS: [Astro; 3] = [
    Astro { name: "Mars", gravity: 3.71 },
    Astro { name: "Moon", gravity: 1.622 },
    Astro { name: "Jupyter", gravity: 24.79 },
];

fn main() {
    let weight: f32 = get_weight();
    let selected_astro = select_astro();
    let calculated_weight = calculate_weight(weight, selected_astro.gravity);
    print_result(weight, selected_astro.name, calculated_weight);
}

fn get_weight() -> f32 {
    loop {
        println!("Hello, enter your weight (kg):");
        let parsed_weight = read_input().trim().parse::<f32>();

        match parsed_weight {
            Ok(value) => break value,
            Err(_) => println!("Error: Please enter a valid number\n")
        }
    }
}

fn select_astro() -> &'static Astro {
    loop {
        let menu_message = "Select where you want to know how much you weight:";
        let options_string: String = OPTIONS.iter()
            .enumerate()
            .map(|(key, astro)| format!("{}: {}\n", key + 1, astro.name))
            .collect();
        let menu: String = format!("{}\n{}", menu_message, options_string);

        println!("{}", menu);

        let parsed_option = read_input().trim().parse::<usize>();

        match parsed_option {
            Ok(v) => {
                if v > 0 && v <= OPTIONS.len() {
                    break &OPTIONS[v - 1];
                }
                println!("Error: Please enter a valid option\n")
            }
            Err(_) => println!("Error: Please enter a valid number\n")
        }
    }
}

fn print_result(weight: f32, astro_name: &str, result: f32) {
    println!("Your weight in Earth is {} kg and your weight in {} is {} kg", weight, astro_name, result)
}

fn calculate_weight(weight: f32, gravity: f32) -> f32 {
    (weight / 9.81) * gravity
}

fn read_input() -> String {
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).unwrap();
    user_input
}