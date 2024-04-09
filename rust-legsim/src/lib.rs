use std::fs;

#[derive(Debug)]
struct RegisterBank(u8, u8, u8, u8, u8, u8, u8);

impl RegisterBank{
    pub fn new() -> RegisterBank{
        RegisterBank(0, 0, 0, 0, 0, 0, 0)
    }
}

#[derive(Debug)]
pub struct Machine {
    prom: [u8; 256],
    memory: [u8; 256],
    stack: [u8; 256],
    registers: RegisterBank
}

impl Machine {
    pub fn load(program_fn: &str) -> Machine{ 
        let program = fs::read_to_string(program_fn).expect("Couldn't read the file.");
        
        if program.lines().count() > 64 {
            panic!("Program had too many lines!")
        }
        
        let mut prom_end = 0;
        let mut prom: [u8; 256] = [0; 256];

        for line in program.lines(){
            let code = parse_line(line);
            let code = match code {
                Some(val) => val,
                None => continue
            };
            prom[prom_end] = code.0;
            prom[prom_end + 1] = code.1;
            prom[prom_end + 2] = code.2;
            prom[prom_end + 3] = code.3;
            prom_end += 4;
        }

        Machine { 
            prom, 
            memory: [0;256], 
            stack: [0;256], 
            registers: RegisterBank::new()
        }
    }
}

fn parse_line(line: &str) -> Option<(u8, u8, u8, u8)> {
    let tokens: Vec<&str> = line.trim().split_whitespace().collect();
    if tokens.contains(&"#"){
        return None
    }
    Some((tokens[0].parse().expect("Invalid token"), 
    tokens[1].parse().expect("Invalid token"), 
    tokens[2].parse().expect("Invalid token"), 
    tokens[3].parse().expect("Invalid token")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn missing_file(){
        let _ = Machine::load("fubar");
    }

    #[test]
    fn load_does_something(){
        let empty = Machine { 
            prom: [0;256], 
            memory: [0;256], 
            stack: [0;256], 
            registers: RegisterBank::new()
        };
        assert_ne!(Machine::load("../echo.out").prom, empty.prom);
    }

    #[test]
    fn load_works(){
        let real_prom: [u8; 12] = [64, 7, 0, 0, 64, 0, 0, 7, 32, 0, 0, 0];
        assert!(Machine::load("../echo.out").prom.starts_with(&real_prom));
    }

    #[test]
    fn parse_line_works(){
        let line = "64 32 0 128";
        assert_eq!(parse_line(line).expect("Failed to parse the ints."), (64, 32, 0, 128));
    }
}
