mod solver;

fn main() {
    /*
    * Solving following soduku with our SAT Solver

       - - - | - 9 4 | - 3 -
       - - - | 5 1 - | - - 7
       - 8 9 | - - - | - 4 -
       ----------------------
       - - - | - - - | 2 - 8
       - 6 - | 2 - 1 | - 5 -
       1 - 2 | - - - | - - -
       ---------------------
       - 7 - | - - - | 5 2 -
       9 - - | - 6 5 | - - -
       - 4 - | 9 7 - | - - -

    */

    // Modeling the sodoku into a SAT problem using the method explained here: https://cse.unl.edu/~choueiry/S17-235H/files/SATslides02.pdf
    // Model:
    // - p(i,j,n) means ith row and jth column has a value n, for eg: p(2,1,8) in our specific soduku puzzle

    let mut cnf: Vec<Vec<String>> = Vec::new();

    // each cell can be a number between 1 to 9
    for i in 0..9 {
        for j in 0..9 {
            let mut clause: Vec<String> = Vec::new();
            for n in 1..=9 {
                let p = format!("p({i},{j},{n})");
                clause.push(p);
            }
            cnf.push(clause);
        }
    }

    // every number exists in a column
    for i in 0..9 {
        for n in 1..=9 {
            let mut clause: Vec<String> = Vec::new();
            for j in 0..9 {
                let p = format!("p({i},{j},{n})");
                clause.push(p);
            }
            cnf.push(clause);
        }
    }

    // every number exists in a row
    for j in 0..9 {
        for n in 1..=9 {
            let mut clause: Vec<String> = Vec::new();
            for i in 0..9 {
                let p = format!("p({i},{j},{n})");
                clause.push(p);
            }
            cnf.push(clause);
        }
    }

    // the cell can only take one number
    for i in 0..9 {
        for j in 0..9 {
            for x in 1..=8 {
                for y in x + 1..=9 {
                    // not (x and y) = (not x) or (not y) (Demorgan's law)
                    let not_x = format!("!p({i},{j},{x})");
                    let not_y = format!("!p({i},{j},{y})");
                    cnf.push(vec![not_x, not_y]);
                }
            }
        }
    }

    // every 3X3 box contains every number
    for r in 0..3 {
        for c in 0..3 {
            for n in 1..=9 {
                let mut clause: Vec<String> = Vec::new();
                for i in 1..3 {
                    for j in 1..3 {
                        let p = format!("p({},{},{})", 3 * r + i, 3 * c + j, n);
                        clause.push(p);
                    }
                }
                cnf.push(clause);
            }
        }
    }

    // initial setup
    cnf.push(vec![format!("p(0,4,9)")]);
    cnf.push(vec![format!("p(0,5,4)")]);
    cnf.push(vec![format!("p(0,7,3)")]);
    cnf.push(vec![format!("p(1,3,5)")]);
    cnf.push(vec![format!("p(1,4,1)")]);
    cnf.push(vec![format!("p(1,8,7)")]);
    cnf.push(vec![format!("p(2,1,8)")]);
    cnf.push(vec![format!("p(2,2,9)")]);
    cnf.push(vec![format!("p(2,7,4)")]);
    cnf.push(vec![format!("p(3,6,2)")]);
    cnf.push(vec![format!("p(3,8,8)")]);
    cnf.push(vec![format!("p(4,1,6)")]);
    cnf.push(vec![format!("p(4,3,2)")]);
    cnf.push(vec![format!("p(4,5,1)")]);
    cnf.push(vec![format!("p(4,7,5)")]);
    cnf.push(vec![format!("p(5,0,1)")]);
    cnf.push(vec![format!("p(5,2,2)")]);
    cnf.push(vec![format!("p(6,1,7)")]);
    cnf.push(vec![format!("p(6,6,5)")]);
    cnf.push(vec![format!("p(6,7,2)")]);
    cnf.push(vec![format!("p(7,0,9)")]);
    cnf.push(vec![format!("p(7,4,6)")]);
    cnf.push(vec![format!("p(7,5,5)")]);
    cnf.push(vec![format!("p(8,1,4)")]);
    cnf.push(vec![format!("p(8,3,9)")]);
    cnf.push(vec![format!("p(8,4,7)")]);

    let mut sodoku_cnf: Vec<Vec<&str>> = Vec::new();
    for clause in &cnf {
        let v = Vec::from_iter(clause.iter().map(String::as_str));
        sodoku_cnf.push(v);
    }
    let mut satSolver = solver::SATSolver::parse_cnf(sodoku_cnf);
    dbg!(&satSolver);
    dbg!(&satSolver.solve());
}
