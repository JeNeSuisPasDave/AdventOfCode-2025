use std::collections::BTreeMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

/// Given input file containing the problem set,
/// solve the problems and accumulate the answers.ingredient database,
///
#[derive(Parser)]
struct Cli {
    /// The path to the file containing battery bank specs
    path: PathBuf,
}

#[derive(Debug)]
enum CephMathOperation {
    Add,
    Multiply,
    Unknown,
}

#[derive(Debug)]
enum InputColumnKind {
    Empty,
    Number,
    NumberAndOperation,
}

#[derive(Debug)]
struct InputColumn {
    chars: Vec<char>,
    op_char: char,
    kind: InputColumnKind,
}

impl InputColumn {
    fn new() -> Self {
        let chars: Vec<char> = Vec::new();
        InputColumn {
            chars: chars,
            op_char: ' ',
            kind: InputColumnKind::Empty,
        }
    }

    fn add_token(&mut self, token: &char) {
        let c: char = *token;
        if c.is_digit(10) {
            self.chars.push(c);
            self.kind = InputColumnKind::Number
        } else if c == '+' || c == '*' {
            self.op_char = c;
            if std::mem::discriminant(&self.kind)
                == std::mem::discriminant(&InputColumnKind::Number)
            {
                self.kind = InputColumnKind::NumberAndOperation;
            } else {
                panic!("Operation without preceding number in column");
            }
        } else if c == ' ' || c == '\t' {
        } else {
            panic!("Unrecognized character '{}'", c);
        }
    }

    fn get_value(&self) -> Option<i64> {
        match self.kind {
            InputColumnKind::Empty => None,
            InputColumnKind::Number
            | InputColumnKind::NumberAndOperation => {
                let s = self.chars.iter().cloned().collect::<String>();
                let s = s.trim();
                let v: i64 = s.parse::<i64>().unwrap();
                Some(v)
            }
        }
    }

    fn get_operation(&self) -> Option<CephMathOperation> {
        match self.kind {
            InputColumnKind::Empty | InputColumnKind::Number => None,
            InputColumnKind::NumberAndOperation => match self.op_char {
                '+' => Some(CephMathOperation::Add),
                '*' => Some(CephMathOperation::Multiply),
                _ => {
                    panic!("Column has invalid operation");
                }
            },
        }
    }
}

#[derive(Debug)]
struct InputColumns {
    columns: BTreeMap<u64, InputColumn>,
}

impl InputColumns {
    fn new() -> Self {
        let columns: BTreeMap<u64, InputColumn> = BTreeMap::new();
        InputColumns { columns: columns }
    }

    fn add_columns(&mut self, line: &str) {
        let mut idx: u64 = 0;
        for c in line.chars() {
            if !self.columns.contains_key(&idx) {
                let column: InputColumn = InputColumn::new();
                self.columns.insert(idx, column);
            }
            self.columns.get_mut(&idx).unwrap().add_token(&c);
            idx += 1;
        }
    }
}

#[derive(Debug)]
struct CephMathProblem {
    operation: CephMathOperation,
    terms: Vec<i64>,
    solution: i64,
}

impl CephMathProblem {
    fn new() -> Self {
        let terms: Vec<i64> = Vec::new();
        Self {
            operation: CephMathOperation::Unknown,
            terms: terms,
            solution: 0,
        }
    }

    fn add_term(&mut self, term: i64) {
        self.terms.push(term);
    }

    fn set_operation(&mut self, operation: CephMathOperation) {
        self.operation = operation;
    }

    fn solve(&mut self) -> i64 {
        let mut first = true;
        let mut result: i64 = 0;
        for term in self.terms.iter() {
            if first {
                result = *term;
                first = false;
            } else {
                match self.operation {
                    CephMathOperation::Add => {
                        result += *term;
                    }
                    CephMathOperation::Multiply => {
                        result *= *term;
                    }
                    CephMathOperation::Unknown => {
                        panic!("UNKNOWN OPERATION");
                    }
                }
            }
        }
        self.solution = result;
        self.solution
    }
}

#[derive(Debug)]
struct CephMathProblemSet {
    problems: BTreeMap<u64, CephMathProblem>,
}

impl CephMathProblemSet {
    // constructor
    //
    fn new() -> Self {
        let problems: BTreeMap<u64, CephMathProblem> = BTreeMap::new();
        CephMathProblemSet { problems: problems }
    }

    fn add_columns(&mut self, ics: &InputColumns) {
        let mut idx: u64 = 0;
        let mut current_problem: CephMathProblem =
            CephMathProblem::new();
        for kv in ics.columns.iter().rev() {
            let (_, ic): (&u64, &InputColumn) = kv;
            match ic.kind {
                InputColumnKind::Empty => {
                    if current_problem.terms.len() != 0 {
                        self.problems.insert(idx, current_problem);
                        idx += 1;
                        current_problem = CephMathProblem::new();
                    }
                }
                InputColumnKind::Number => {
                    let v: i64 = ic.get_value().unwrap();
                    current_problem.add_term(v);
                }
                InputColumnKind::NumberAndOperation => {
                    let v: i64 = ic.get_value().unwrap();
                    let op: CephMathOperation =
                        ic.get_operation().unwrap();
                    current_problem.add_term(v);
                    current_problem.set_operation(op);
                }
            }
        }
        if current_problem.terms.len() != 0 {
            self.problems.insert(idx, current_problem);
        }
    }

    #[allow(dead_code)]
    fn add_terms(&mut self, terms: &Vec<&str>) {
        // create the problems if they don't exist yet
        //
        if self.problems.len() == 0 {
            let mut idx: u64 = 0;
            for _term in terms.iter() {
                self.problems.insert(idx, CephMathProblem::new());
                idx += 1;
            }
        }

        // add the terms to each problem
        //
        if terms.len() != self.problems.len() {
            panic!(
                "Number of terms does not match number of existing problems."
            );
        }
        let mut idx: u64 = 0;
        for term in terms {
            let problem = self.problems.get_mut(&idx).unwrap();
            let val = term.parse::<i64>().unwrap();
            problem.add_term(val);
            idx += 1;
        }
    }

    #[allow(dead_code)]
    fn add_operations(&mut self, operations: &Vec<&str>) {
        if operations.len() != self.problems.len() {
            panic!(
                "Number of operations does not match number of existing problems."
            );
        }
        let mut idx: u64 = 0;
        for operation in operations {
            let problem = self.problems.get_mut(&idx).unwrap();
            if operation.eq(&"*") {
                problem.set_operation(CephMathOperation::Multiply);
            } else if operation.eq(&"+") {
                problem.set_operation(CephMathOperation::Add);
            } else {
                panic!("INVALID OPERATION");
            }
            idx += 1;
        }
    }

    fn get_solutions(&self) -> Vec<i64> {
        let mut solutions: Vec<i64> = Vec::new();
        let keys: Vec<u64> = self.problems.keys().cloned().collect();
        for key in keys {
            let problem = self.problems.get(&key).unwrap();
            solutions.push(problem.solution);
        }
        solutions
    }

    fn solve_all(&mut self) {
        let keys: Vec<u64> = self.problems.keys().cloned().collect();
        for key in keys {
            let problem = self.problems.get_mut(&key).unwrap();
            problem.solve();
        }
    }
}

// Binary crate entry point
//
fn main() -> Result<()> {
    let args = Cli::parse();
    let path = &args.path;

    let f = File::open(path).with_context(|| {
        format!("Could not open `{}`", path.display())
    })?;
    let rdr = BufReader::new(f);
    let lines = rdr.lines();

    let mut grand_total: i64 = 0;
    let mut cmps = CephMathProblemSet::new();
    let mut ics = InputColumns::new();
    for line in lines {
        let line = line.unwrap();
        // let line = line.trim();
        if 0 == line.len() {
            continue;
        }
        ics.add_columns(&line);
    }
    cmps.add_columns(&ics);
    cmps.solve_all();
    let solutions = cmps.get_solutions();
    for solution in solutions {
        grand_total += solution;
    }

    // Display the grand total of problem answers
    //
    println!("The grand total of problem answers is {}", grand_total);
    Ok(())
}

// test with example input
//
#[test]
fn given_example() {
    let expected: i64 = 3263827; // part 1 was  4277556;
    let raw_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
        .to_string();
    let input = raw_input.as_str();
    let mut cmps = CephMathProblemSet::new();
    let mut ics = InputColumns::new();
    let lines = input.split('\n');
    for line in lines {
        // let line = line.trim();
        if 0 == line.len() {
            continue;
        }
        ics.add_columns(line);
    }
    cmps.add_columns(&ics);
    cmps.solve_all();
    let solutions = cmps.get_solutions();
    let mut actual: i64 = 0;
    for solution in solutions {
        actual += solution;
    }
    assert_eq!(expected, actual);
}
