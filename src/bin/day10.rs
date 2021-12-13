use advent_of_code_2021::Input;
use itertools::Itertools;
use std::error;
use thiserror::Error;

/// Input parse error
#[derive(Debug, Error, Clone, PartialEq, Eq)]
enum ParseError {
    #[error("Line corrupted, expected `{0}`, found `{1}`")]
    Corrupted(char, char),
    #[error("Line incomplete, chunks no closed: {0:?}")]
    Incomplete(Vec<char>),
    #[error("Syntax error")]
    Syntax,
}

fn parse(line: &str) -> Result<(), ParseError> {
    let mut chunks = Vec::new();

    for token in line.chars() {
        match token {
            '(' => chunks.push(')'),
            '[' => chunks.push(']'),
            '{' => chunks.push('}'),
            '<' => chunks.push('>'),
            ')' | ']' | '}' | '>' => {
                let expected = chunks.pop().ok_or(ParseError::Syntax)?;
                if token != expected {
                    return Err(ParseError::Corrupted(expected, token));
                }
            }
            _ => return Err(ParseError::Syntax),
        }
    }

    if !chunks.is_empty() {
        Err(ParseError::Incomplete(chunks))
    } else {
        Ok(())
    }
}

fn corrupt_score(line: &str) -> usize {
    match parse(line) {
        Err(ParseError::Corrupted(_, ')')) => 3,
        Err(ParseError::Corrupted(_, ']')) => 57,
        Err(ParseError::Corrupted(_, '}')) => 1197,
        Err(ParseError::Corrupted(_, '>')) => 25137,
        _ => 0,
    }
}

fn incomplete_score(line: &str) -> usize {
    match parse(line) {
        Err(ParseError::Incomplete(chunks)) => chunks.iter().rev().fold(0, |score, ch| {
            score * 5
                + match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
        }),
        _ => 0,
    }
}

fn total_corrupt_score<S: AsRef<str>>(lines: &[S]) -> usize {
    lines.iter().map(|line| corrupt_score(line.as_ref())).sum()
}

fn median_incomplete_score<S: AsRef<str>>(lines: &[S]) -> usize {
    let mut scores: Vec<_> = lines
        .iter()
        .map(|line| incomplete_score(line.as_ref()))
        .filter(|score| *score > 0)
        .collect();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let lines: Vec<_> = Input::day(10)?.lines().try_collect()?;

    println!(
        "Total corrupt syntax error score: {}",
        total_corrupt_score(&lines)
    );

    println!(
        "Median incomplete syntax error score: {}",
        median_incomplete_score(&lines)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: [&str; 10] = [
        "[({(<(())[]>[[{[]{<()<>>",
        "[(()[<>])]({[<{<<[]>>(",
        "{([(<{}[<>[]}>{[]{[(<()>",
        "(((({<>}<{<{<>}{[]{[]{}",
        "[[<[([]))<([[{}[[()]]]",
        "[{[{({}]{}}([{[{{{}}([]",
        "{<[[]]>}<{[{[{[]{()[[[]",
        "[<(<(<(<{}))><([]([]()",
        "<{([([[(<>()){}]>(<<{{",
        "<{([{{}}[<[[[<>{}]]]>[]]",
    ];

    #[test]
    fn part_1() {
        assert_eq!(parse(EXAMPLES[2]), Err(ParseError::Corrupted(']', '}')));
        assert_eq!(parse(EXAMPLES[4]), Err(ParseError::Corrupted(']', ')')));
        assert_eq!(parse(EXAMPLES[5]), Err(ParseError::Corrupted(')', ']')));
        assert_eq!(parse(EXAMPLES[7]), Err(ParseError::Corrupted('>', ')')));
        assert_eq!(parse(EXAMPLES[8]), Err(ParseError::Corrupted(']', '>')));

        assert_eq!(total_corrupt_score(&EXAMPLES), 26397);
    }

    #[test]
    fn part_2() {
        assert_eq!(
            parse(EXAMPLES[0]),
            Err(ParseError::Incomplete(vec![
                ']', ')', '}', ')', ']', ']', '}', '}'
            ]))
        );
        assert_eq!(
            parse(EXAMPLES[1]),
            Err(ParseError::Incomplete(vec![')', '}', ']', '>', '}', ')']))
        );
        assert_eq!(
            parse(EXAMPLES[3]),
            Err(ParseError::Incomplete(vec![
                ')', ')', ')', ')', '>', '}', '>', '}', '}'
            ]))
        );
        assert_eq!(
            parse(EXAMPLES[6]),
            Err(ParseError::Incomplete(vec![
                '>', '}', ']', '}', ']', '}', '}', ']', ']'
            ]))
        );
        assert_eq!(
            parse(EXAMPLES[9]),
            Err(ParseError::Incomplete(vec!['>', '}', ')', ']']))
        );

        assert_eq!(incomplete_score(EXAMPLES[0]), 288957);
        assert_eq!(incomplete_score(EXAMPLES[1]), 5566);
        assert_eq!(incomplete_score(EXAMPLES[3]), 1480781);
        assert_eq!(incomplete_score(EXAMPLES[6]), 995444);
        assert_eq!(incomplete_score(EXAMPLES[9]), 294);

        assert_eq!(median_incomplete_score(&EXAMPLES), 288957);
    }
}
