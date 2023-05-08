use std::env;
use std::io::Read;
use std::fs::File;

/// Opcodes determined by the lexer
#[derive(Debug)]
#[derive(Clone)]
enum OpCode {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

#[derive(Debug)]
#[derive(Clone)]
enum Instruction {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>)
}

/// Lexer turns the source code into a sequence of opcodes
fn lex(source: String) -> Vec<OpCode> {
    let mut operations = Vec::new();

    for symbol in source.chars() {
        let op = match symbol {
            '>' => Some(OpCode::IncrementPointer),
            '<' => Some(OpCode::DecrementPointer),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            _ => None
        };

        // Non-opcode characters are simply comments
        match op {
            Some(op) => operations.push(op),
            None => ()
        }
    }

    operations
}

fn parse(opcodes: Vec<OpCode>) -> Vec<Instruction> {
    let mut program: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (i, op) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instr = match op {
                OpCode::IncrementPointer => Some(Instruction::IncrementPointer),
                OpCode::DecrementPointer => Some(Instruction::DecrementPointer),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read),

                OpCode::LoopBegin => {
                    loop_start = i;
                    loop_stack += 1;
                    None
                },

                OpCode::LoopEnd => panic!("Loop ending at #{} has no beginning", i),
            };

            match instr {
                Some(instr) => program.push(instr),
                None => ()
            }
        } else {
            match op {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                },
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        program.push(Instruction::Loop(parse(opcodes[loop_start+1..i].to_vec())));
                    }
                },
                _ => (),
            }
        }
    }

    if loop_stack != 0 {
        panic!("Loop that starts at #{} has no matching ending!", loop_start);
    }

    program
}

/// Executes a program that was previously parsed
fn run(instructions: &Vec<Instruction>, bf_memory: &mut Vec<u8>, data_pointer: &mut usize) {
    for instr in instructions {
        match instr {
            Instruction::IncrementPointer => *data_pointer += 1,
            Instruction::DecrementPointer => *data_pointer -= 1,
            Instruction::Increment => bf_memory[*data_pointer] += 1,
            Instruction::Decrement => bf_memory[*data_pointer] -= 1,
            Instruction::Write => print!("{}", bf_memory[*data_pointer] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                std::io::stdin().read_exact(&mut input).expect("failed to read stdin");
                bf_memory[*data_pointer] = input[0];
            },
            Instruction::Loop(nested_instructions) => {
                while bf_memory[*data_pointer] != 0 {
                    run(&nested_instructions, bf_memory, data_pointer)
                }
            }
        }
    }
}

fn main() {
    // Determine which file to execute
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: bfi_rs 'some.bf'");
        std::process::exit(1);
    }
    
    let filename = &args[1];

    // Read file
    let mut file = File::open(filename).expect("Executable file not found");
    let mut source = String::new();
    file.read_to_string(&mut source).expect("Failed to read executable file");

    let opcodes = lex(source);

    let program = parse(opcodes);

    let mut bf_memory: Vec<u8> = vec![0; 1024];
    let mut data_pointer = 0;

    run(&program, &mut bf_memory, &mut data_pointer);
}