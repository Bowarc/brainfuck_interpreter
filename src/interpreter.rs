// This function tries to read a file from env::arg
// outputs a vec of instructions if it finds one
// In a lack of a better name i'll keep this one for now
pub fn init() -> Option<Vec<crate::instruction::Instruction>> {
    use {
        crate::instruction::Instruction,
        std::{env, fs, io::Read as _, path, str::FromStr as _},
    };
    let argv = env::args().collect::<Vec<_>>();
    let argc = argv.len();

    if argc == 1 {
        return None;
    }

    let Ok(input_path) = path::PathBuf::from_str(argv.get(1).unwrap()) else {
        return None;
    };

    if !input_path.exists() || !input_path.is_file() {
        return None;
    }

    let input_path = input_path.canonicalize().unwrap();
    println!(
        "Reading: {}\n",
        input_path
            .to_str()
            .unwrap()
            .to_owned()
            .replace("\\\\?\\", "")
            .replace("\\", "/")
    );

    let mut input = String::new();
    let mut file = fs::OpenOptions::new().read(true).open(input_path).unwrap(); // we know it's a file
    file.read_to_string(&mut input).unwrap();

    Some(Instruction::parse(&input))
}

pub fn run(instruction_buffer: Vec<crate::instruction::Instruction>) {
    let mut instruction_pointer = 0;

    let mut data_buffer: [u8; 30000] = [0; 30_000];
    let mut data_pointer = 0;

    let mut loop_stack = Vec::new();

    while instruction_pointer < instruction_buffer.len() {
        let instruction = instruction_buffer.get(instruction_pointer).unwrap();
        // println!("Running {instruction_pointer}: {instruction:?}");
        run_one(
            instruction,
            &instruction_buffer,
            &mut instruction_pointer,
            &mut loop_stack,
            &mut data_buffer,
            &mut data_pointer,
        );
        instruction_pointer += 1;
    }
}

fn run_one(
    instruction: &crate::instruction::Instruction,
    instruction_buffer: &[crate::instruction::Instruction],
    instruction_pointer: &mut usize,
    loop_stack: &mut Vec<usize>,
    data_buffer: &mut [u8],
    data_pointer: &mut usize,
) {
    use crate::instruction::Instruction;

    match instruction {
        Instruction::MoveRight => {
            if *data_pointer == data_buffer.len() - 1 {
                eprintln!("Tried to go past buffer len\nInstruction {instruction:?} at {instruction_pointer}\nBuffer cell value: {} at {data_pointer}", data_buffer.get(*data_pointer).unwrap());
                panic!();
            }
            *data_pointer += 1;
        }
        Instruction::MoveLeft => {
            if *data_pointer == 0 {
                eprintln!("Tried to go past buffer len\nInstruction {instruction:?} at {instruction_pointer}\nBuffer cell value: {}", data_buffer.get(*data_pointer).unwrap());
                panic!();
            }
            *data_pointer -= 1;
        }
        Instruction::Add => {
            let cell = data_buffer.get_mut(*data_pointer).unwrap();

            *cell = cell.wrapping_add(1);
        }
        Instruction::Sub => {
            let cell = data_buffer.get_mut(*data_pointer).unwrap();

            *cell = cell.wrapping_sub(1);
        }
        Instruction::Output => {
            use std::io::{stdout, Write as _};
            let value = data_buffer.get(*data_pointer).unwrap();

            let mut lock = stdout().lock();

            let converted = char::from_u32(u32::from(*value)).unwrap();

            write!(lock, "{converted}").unwrap();

            drop(lock);
        }
        Instruction::Input => {
            use std::io::{stdin, Read as _};

            println!("[Debug] Reading key");

            let mut buffer = [0; 1];

            stdin().read_exact(&mut buffer).unwrap();

            println!("[Debug] Read key: {buffer:?}");

            let cell = data_buffer.get_mut(*data_pointer).unwrap();
            *cell = buffer[0];
            println!("Buffer value: {}", data_buffer.get(*data_pointer).unwrap());
        }
        Instruction::LoopStart => {
            if *data_buffer.get(*data_pointer).unwrap() == 0 {
                // println!("Start quick jump\nIp: {instruction_pointer}");

                let mut start_founds = 0;

                loop {
                    *instruction_pointer += 1;
                    let Some(inst) = instruction_buffer.get(*instruction_pointer) else {
                        eprintln!("Could not skip to LoopEnd due to: End of instruction list");
                        return;
                    };

                    if *inst == Instruction::LoopStart {
                        // println!("Recursion issue");
                        start_founds += 1;
                    }
                    if *inst == Instruction::LoopEnd {
                        if start_founds == 0 {
                            break;
                        }
                        start_founds -= 1;
                    }
                }
                // println!("Stop quick jump\nIp: {instruction_pointer}");
            } else {
                loop_stack.push(*instruction_pointer);
            }
        }
        Instruction::LoopEnd => {
            if loop_stack.is_empty() {
                eprintln!("Found a loop end before any loop start");
                panic!("");
            }

            let last_loop_start = loop_stack.last().unwrap(); // We just checked before if it was empty

            if *data_buffer.get(*data_pointer).unwrap() == 0 {
                loop_stack.remove(loop_stack.len() - 1); // Exit the loop and remove the loop entry from the stack
            } else {
                *instruction_pointer = *last_loop_start; // This will point to the last [ and the instruction pointer will be incremented at the end of the current loop
                                                         // println!("JUMP");
            }
        }
    };
}
