use tokens::Tokens;

mod tokens;

fn construct_number(file: &Vec<u8>, index: &mut usize) -> Tokens {
    let mut int_string = String::new();

    let mut is_float = false;

    while *index < file.len() {
        let char = file[*index] as char;

        if !char.is_numeric() && char != '.' {
            break;
        }

        if char == '.' {
            is_float = true;
        }

        int_string += &char.to_string();
        *index += 1;
    }

    *index -= 1;

    if !is_float {
        Tokens::Integer(int_string.parse::<i32>().unwrap())
    } else {
        Tokens::Float(int_string.parse::<f32>().unwrap())
    }
}

fn main() {
    let file = std::fs::read("test/first.txt").unwrap();

    let mut tokens: Vec<_> = vec![];

    let mut index = 0;

    while index < file.len() {
        let char = file[index] as char;

        let token = match char {
            ' ' | '\t' | '\n' => {
                index += 1;
                continue;
            }
            '+' => Tokens::Plus,
            '-' => Tokens::Minus,
            '*' => Tokens::Multiply,
            '/' => Tokens::Divide,
            '=' => Tokens::Equals,
            _ => {
                if char.is_numeric() {
                    construct_number(&file, &mut index)
                } else {
                    Tokens::Unknown
                }
            }
        };

        println!("Token: {:?}", token);

        tokens.push(token);

        index += 1;
    }

    println!("{:?}", tokens);
}
