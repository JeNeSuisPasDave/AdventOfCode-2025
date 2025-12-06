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

enum CephMathOperation {
    Add,
    Multiply,
    Unknown,
}

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

struct CephMathProblemSet {
    count: u64,
    problems: BTreeMap<u64, CephMathProblem>,
}

impl CephMathProblemSet {
    // constructor
    //
    fn new() -> Self {
        let problems: BTreeMap<u64, CephMathProblem> = BTreeMap::new();
        CephMathProblemSet {
            count: 0,
            problems: problems,
        }
    }

    fn add_terms(&mut self, terms: &Vec<&str>) {
        // create the problems if they don't exist yet
        //
        if self.problems.len() == 0 {
            for _term in terms.iter() {
                self.problems
                    .insert(self.count, CephMathProblem::new());
                self.count += 1;
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
    for line in lines {
        let line = line.unwrap();
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() <= 1 {
            panic!(
                "INVALID INPUT. Problem set line contains only one token"
            );
        }
        let first_term = parts.get(0).unwrap();
        if first_term.parse::<i64>().is_ok() {
            cmps.add_terms(&parts);
        } else {
            cmps.add_operations(&parts);
        }
    }
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
    let expected: i64 = 4277556;
    let raw_input = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  "
        .to_string();
    let input = raw_input.as_str();
    let mut cmps = CephMathProblemSet::new();
    let lc = input.split('\n').count();
    let lines = input.split('\n');
    for line in lines {
        let line = line.trim();
        if 0 == line.len() {
            continue;
        }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() <= 1 {
            panic!(
                "INVALID INPUT. Problem set line contains only one token"
            );
        }
        let first_term = parts.get(0).unwrap();
        if first_term.parse::<i64>().is_ok() {
            cmps.add_terms(&parts);
        } else {
            cmps.add_operations(&parts);
        }
    }
    cmps.solve_all();
    let solutions = cmps.get_solutions();
    let mut actual: i64 = 0;
    for solution in solutions {
        actual += solution;
    }
    assert_eq!(expected, actual);
}
