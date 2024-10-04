
use crate::compiler::Instruction;

pub fn eval(instructions: &[Instruction], chars: &Vec<char>, mut p_counter: usize, mut index: usize) -> bool {
    loop {
        let instruction: &Instruction = instructions.get(p_counter).unwrap();

        match instruction {
            Instruction::Char(c) => {
                let character = chars.get(index).unwrap();
                if c == character {
                    p_counter += 1;
                    index += 1;
                } else {
                    return false
                }
            }
            Instruction::Match => return true,
            Instruction::Jump(counter) => p_counter = *counter,
            Instruction::Split(counter1, counter2 ) => {
                if eval(instructions, chars, *counter1, index) || eval(instructions, chars, *counter2, index) {
                    return true
                } else {
                    return false
                }
            }
        }
    }
}

#[test]
fn test_eval() {
    // "ab(c|d)" が入力された Instraction
    let insts: Vec<Instruction> = vec![
        Instruction::Char('a'),
        Instruction::Char('b'),
        Instruction::Split(3, 5),
        Instruction::Char('c'),
        Instruction::Jump(6),
        Instruction::Char('d'),
        Instruction::Match
    ];

    // "abc" とマッチするケース = true
    let chars1:Vec<char> = vec!['a', 'b', 'c'];
    assert_eq!(
        eval(&insts, &chars1, 0, 0),
        true
    );

    // "abd"とマッチするケース = true
    let chars2:Vec<char> = vec!['a', 'b', 'd'];
    assert_eq!(
        eval(&insts, &chars2, 0, 0),
        true
    );

    // "abx" とマッチするケース
    let chars3:Vec<char> = vec!['a', 'b', 'X'];
    assert_eq!(
        eval(&insts, &chars3, 0, 0),
        false
    );
}