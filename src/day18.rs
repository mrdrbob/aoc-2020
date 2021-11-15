pub fn execute() {
    let file = std::fs::read_to_string(".\\data\\18.txt").unwrap();

    let sum:usize = file.lines().fold(0, |acc, line| {
        println!("{}", line);
        let tokenized = tokenize(line);
        let result = process_expression(ParserState { state: ExpressionState::Empty, position: 0 }, &tokenized);
        let value = match result.state {
            ExpressionState::Done(v) => v,
            _ => panic!("Invalid expression state - {:?}", result)
        };

        acc + value
    });

    println!("{}", sum);

    // let t = "9 + 2 * ((7 + 9 + 5 + 7) * 3 + (5 * 3 * 6 * 5 + 6) + (9 * 4 + 9 * 6 + 9)) * 8 + 5";
    // let tokenized = tokenize(t);
    // let value = process_expression(ParserState { state: ExpressionState::Empty, position: 0 }, &tokenized);
    // println!("{:?}", value);
}

fn process_expression(state:ParserState, tokens:&Vec<Token>) -> ParserState {
    let token = &tokens[state.position];
    // println!("{:?}", token);
    match token {
        Token::Close => match state.state {
            ExpressionState::ValueOnly(v) => ParserState { position: state.position + 1, state: ExpressionState::Done(v) },
            _ => panic!("Invalid operation")
        },
        Token::Open => {
            let sub_result = process_expression(ParserState { position: state.position + 1, state: ExpressionState::Empty }, tokens);
            match sub_result.state {
                ExpressionState::Done(v) => match state.state {
                    ExpressionState::Empty => process_expression(ParserState { position: sub_result.position, state: ExpressionState::ValueOnly(v) }, tokens),
                    ExpressionState::ValueAndOperation(ov, op) => match op {
                        Operation::Plus => process_expression(ParserState { position: sub_result.position, state: ExpressionState::ValueOnly(ov + v) }, tokens),
                        Operation::Times => process_expression(ParserState { position: sub_result.position, state: ExpressionState::ValueOnly(ov * v) }, tokens)
                    },
                    _ => panic!("Invalid operation {:?} {:?}", state, token)
                },
                _ => panic!("Invalid operation")
            }
        },
        Token::Whitespace => ParserState { position: state.position + 1, state: state.state },
        Token::Digit(v) => match state.state {
            ExpressionState::Empty => process_expression(ParserState { position: state.position + 1, state: ExpressionState::ValueOnly(*v) }, tokens),
            ExpressionState::ValueAndOperation(ov, op) => match op {
                Operation::Plus => process_expression(ParserState { position: state.position + 1, state: ExpressionState::ValueOnly(ov + v) }, tokens),
                Operation::Times => process_expression(ParserState { position: state.position + 1, state: ExpressionState::ValueOnly(ov * v) }, tokens)
            },
            _ => panic!("Invalid operation {:?} {:?}", state, token)
        },
        Token::Op(op) => match state.state {
            ExpressionState::ValueOnly(v) => process_expression(ParserState { position: state.position + 1, state: ExpressionState::ValueAndOperation(v, *op) }, tokens),
            _ => panic!("Invalid operation")
        }
        _ => panic!("Invalid operation")
    }
}

#[derive(Debug)]
struct ParserState {
    position: usize,
    state: ExpressionState
}

#[derive(Debug)]
enum ExpressionState {
    Empty,
    ValueOnly(usize),
    ValueAndOperation(usize, Operation),
    Done(usize)
}

fn tokenize(line:&str) -> Vec<Token> {
    let mut output:Vec<Token> = Vec::new();

    for c in line.chars() {
        let token = match c {
            '0' => Token::Digit(0),
            '1' => Token::Digit(1),
            '2' => Token::Digit(2),
            '3' => Token::Digit(3),
            '4' => Token::Digit(4),
            '5' => Token::Digit(5),
            '6' => Token::Digit(6),
            '7' => Token::Digit(7),
            '8' => Token::Digit(8),
            '9' => Token::Digit(9),
            '+' => Token::Op(Operation::Plus),
            '*' => Token::Op(Operation::Times),
            '(' => Token::Open,
            ')' => Token::Close,
            _ => Token::Whitespace
        };
        match token {
            Token::Whitespace => {},
            other => { output.push(other); }
        };
    }

    // Dumb hack to force close end of line
    // assumes all input is well-formed.
    // Saves me from checking for the end of a line.
    output.push(Token::Close);

    output
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Times,
    Plus
}

#[derive(Debug)]
enum Token {
    Digit(usize),
    Open,
    Close,
    Op(Operation),
    Whitespace
}
