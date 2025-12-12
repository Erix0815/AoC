use std::fs;

fn transpose<T: Copy>(mat: Vec<Vec<T>>) -> Vec<Vec<T>> {
    (0..mat[0].len()).map(|i|
        mat.iter().map(|inner|
            inner[i]
        ).collect::<Vec<T>>()
    ).collect()
}

fn main() {
    const FILE: &str = "./input/day6.txt";
    let content = fs::read_to_string(FILE)
        .expect(format!("failed to read file: '{FILE}'").as_str());
    let mut lines = content.lines()
        .rev() // operand comes first
        .map(|str|str.split_whitespace());
    let operators = lines.next().expect("found no operators");
    let operands_matrix = transpose(lines.map(|ops|
        ops.map(|op| op.parse::<i64>()
            .expect(format!("illegeal operand: '{op}'").as_str())
        ).collect::<Vec<_>>()
    ).collect::<Vec<_>>());

    let problems = operators.zip(operands_matrix);
    let solutions = problems.map(|(operator, operands)|
        match operator {
            "+" => Ok(operands.into_iter().sum::<i64>()),
            "*" => Ok(operands.into_iter().product()),
             _  => Err(format!("illegal operator: '{operator}'")),
        }.unwrap()
    );
    let checksum: i64 = solutions.sum();
    println!("the checksum is {checksum}")
}
