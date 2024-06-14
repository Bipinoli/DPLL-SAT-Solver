use std::{collections::HashMap, usize};

#[derive(Debug, Clone, PartialEq)]
enum Literal {
    Must,
    MustNot,
    Absent,
}

#[derive(Debug)]
pub struct Clause {
    disjunction: Vec<Literal>,
}
impl Clause {
    pub fn is_empty(&self, is_literal_alive: &Vec<bool>) -> bool {
        for (i, alive) in is_literal_alive.iter().enumerate() {
            if alive.clone() == true {
                if self.disjunction[i] != Literal::Absent {
                    return false;
                }
            }
        }
        return true;
    }
    pub fn find_unit(&self, is_literal_alive: &Vec<bool>) -> Option<(Literal, usize)> {
        let mut count = 0;
        let mut unit: (Literal, usize) = (Literal::Must, 0);
        for (i, alive) in is_literal_alive.iter().enumerate() {
            if alive.clone() == true {
                match self.disjunction[i] {
                    Literal::MustNot | Literal::Must => {
                        count += 1;
                        unit = (self.disjunction[i].clone(), i);
                    }
                    _ => (),
                }
            }
        }
        if count == 1 {
            return Some(unit);
        }
        None
    }
    pub fn is_satisfied(&self, values: &Vec<(Literal, usize)>) -> bool {
        for (lit, item) in values {
            if self.disjunction[item.clone()] == lit.clone() {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct SATSolver {
    pub clauses: Vec<Clause>,
    pub total_literals: usize,
    literal_to_id: HashMap<String, usize>,
    id_to_literal: HashMap<usize, String>,
    values: Vec<(Literal, usize)>,
    literal_mask: Vec<bool>,
}

impl SATSolver {
    /// parses the CNF as an input
    ///
    /// for example:
    ///    CNF:
    ///         (apple OR !cat) AND
    ///         (!apple OR sugar_cane OR cat) AND
    ///         (!sugar_cane OR apple OR !cat)
    ///
    ///    should be input as vector of disjunctions as:
    ///
    ///     input:
    ///     >> [
    ///         ["apple", "!cat"],
    ///         ["!apple", "sugar_cane", "cat"],
    ///         ["!sugar_cane", "apple", !cat],
    ///        ]
    pub fn parse_cnf(cnf: Vec<Vec<&str>>) -> Self {
        let mut clauses: Vec<Clause> = Vec::new();
        let mut total_literals: usize = 0;
        let mut literal_to_id: HashMap<String, usize> = HashMap::new();
        let mut id_to_literal: HashMap<usize, String> = HashMap::new();

        for clause in &cnf {
            for literal in clause {
                let (_, lit) = SATSolver::read_literal(&literal.to_string());
                if !literal_to_id.contains_key(&lit) {
                    literal_to_id.insert(lit.clone(), total_literals);
                    id_to_literal.insert(total_literals, lit.clone());
                    total_literals += 1;
                }
            }
        }
        for clause in &cnf {
            let mut cls = vec![Literal::Absent; total_literals];
            for literal in clause {
                let (must, lit) = SATSolver::read_literal(&literal.to_string());
                let id: usize = literal_to_id.get(&lit).unwrap().clone();
                cls[id] = must;
            }
            clauses.push(Clause { disjunction: cls });
        }
        Self {
            clauses,
            total_literals,
            literal_to_id,
            id_to_literal,
            literal_mask: vec![true; total_literals],
            values: Vec::new(),
        }
    }

    fn read_literal(literal: &String) -> (Literal, String) {
        if literal.starts_with('!') {
            (Literal::MustNot, literal[1..].to_string())
        } else {
            (Literal::Must, literal.clone())
        }
    }

    fn find_unit_clause(&mut self) -> Option<(Literal, usize)> {
        for clause in &self.clauses {
            if clause.is_satisfied(&self.values) {
                continue;
            }
            if let Some(unit) = clause.find_unit(&self.literal_mask) {
                return Some(unit);
            }
        }
        None
    }

    fn find_pure_literal(&self) -> Option<(Literal, usize)> {
        for i in 0..self.total_literals {
            if self.literal_mask[i] == false {
                continue;
            }
            let mut must = false;
            let mut must_not = false;
            for clause in &self.clauses {
                match clause.disjunction[i] {
                    Literal::Must => must = true,
                    Literal::MustNot => must_not = true,
                    _ => (),
                }
            }
            if must && !must_not {
                return Some((Literal::Must, i));
            }
            if must_not && !must {
                return Some((Literal::MustNot, i));
            }
        }
        None
    }

    fn is_unsatisfiable(&self) -> bool {
        for cls in &self.clauses {
            if !cls.is_satisfied(&self.values) && cls.is_empty(&self.literal_mask) {
                return true;
            }
        }
        return false;
    }

    fn is_satisfied(&self) -> bool {
        for cls in &self.clauses {
            if !cls.is_satisfied(&self.values) {
                return false;
            }
        }
        return true;
    }

    pub fn solve(&mut self) -> (bool, Option<Vec<String>>) {
        let satisfiable = self.dpll();
        if !satisfiable {
            return (false, None);
        }
        let mut solution: Vec<String> = Vec::new();
        for (lit, lit_id) in &self.values {
            let lit_name = self.id_to_literal.get(lit_id).unwrap();
            match lit {
                Literal::Must => solution.push(lit_name.clone()),
                Literal::MustNot => solution.push(format!("!{lit_name}")),
                Literal::Absent => (),
            }
        }
        return (true, Some(solution));
    }

    fn dpll(&mut self) -> bool {
        if self.is_satisfied() {
            return true;
        }
        if self.is_unsatisfiable() {
            return false;
        }

        // unit propagation
        loop {
            match self.find_unit_clause() {
                Some(unit_clause) => {
                    self.values.push(unit_clause.clone());
                    let (_, item) = unit_clause;
                    self.literal_mask[item] = false;
                    if self.is_satisfied() {
                        return true;
                    }
                    if self.is_unsatisfiable() {
                        return false;
                    }
                }
                _ => break,
            }
        }

        // pure-literal elimination
        loop {
            match self.find_pure_literal() {
                Some(pure_literal) => {
                    self.values.push(pure_literal.clone());
                    let (_, item) = pure_literal;
                    self.literal_mask[item] = false;
                    if self.is_satisfied() {
                        return true;
                    }
                    if self.is_unsatisfiable() {
                        return false;
                    }
                }
                _ => break,
            }
        }

        // choose + backtrack process
        let mut literal = 0;
        while self.literal_mask[literal] == false {
            literal += 1;
        }
        self.literal_mask[literal] = false;
        dbg!(format!("choosing Must in {literal}"));
        self.values.push((Literal::Must, literal));
        if self.dpll() == true {
            return true;
        }
        while self.values.last().unwrap().1 != literal {
            self.values.pop();
        }
        self.values.pop();
        dbg!(format!("choosing MustNot in {literal}"));
        self.values.push((Literal::MustNot, literal));
        return self.dpll();
    }
}

#[cfg(test)]
mod test {
    use crate::solver::Literal;

    use super::SATSolver;

    #[test]
    fn parse_cnf() {
        let cnf = vec![
            vec!["apple", "!cat"],
            vec!["!apple", "sugar_cane", "cat"],
            vec!["!sugar_cane", "apple", "!cat"],
        ];
        let satSolver = SATSolver::parse_cnf(cnf);
        dbg!(&satSolver);
        assert_eq!(satSolver.total_literals, 3);
        assert_eq!(satSolver.clauses.len(), 3);
        assert_eq!(satSolver.clauses[0].disjunction[0], Literal::Must);
        assert_eq!(satSolver.clauses[0].disjunction[1], Literal::MustNot);
        assert_eq!(satSolver.clauses[0].disjunction[2], Literal::Absent);
    }

    #[test]
    fn unit_propagation() {
        let cnf = vec![
            vec!["apple"],
            vec!["!apple", "sugar_cane"],
            vec!["!sugar_cane", "apple", "!cat"],
        ];
        let mut satSolver = SATSolver::parse_cnf(cnf);
        let (satisfiable, solution) = satSolver.solve();
        assert_eq!(satisfiable, true);
        assert_eq!(
            solution,
            Some(vec!["apple".to_string(), "sugar_cane".to_string()])
        );
    }

    #[test]
    fn pure_literal_elimination() {
        let cnf = vec![
            vec!["apple", "!cat", "kangaroo"],
            vec!["sugar_cane", "!cat"],
            vec!["!sugar_cane", "apple", "eagle"],
        ];
        let mut satSolver = SATSolver::parse_cnf(cnf);
        let (satisfiable, solution) = satSolver.solve();
        assert_eq!(satisfiable, true);
        assert_eq!(
            solution,
            Some(vec!["apple".to_string(), "!cat".to_string()])
        );
    }

    #[test]
    fn simple() {
        let cnf = vec![
            vec!["apple", "!cat"],
            vec!["!apple", "sugar_cane", "cat"],
            vec!["!sugar_cane", "apple", "!cat"],
            vec!["cat", "!sugar_cane"],
        ];
        let mut satSolver = SATSolver::parse_cnf(cnf);
        let (satisfiable, solution) = satSolver.solve();
        assert_eq!(satisfiable, true);
        assert_eq!(solution, Some(vec!["apple".to_string(), "cat".to_string()]));
    }

    #[test]
    fn simple_unsatisfiable() {
        let cnf = vec![vec!["a"], vec!["!a"]];
        let mut satSolver = SATSolver::parse_cnf(cnf);
        let (satisfiable, _) = satSolver.solve();
        assert_eq!(satisfiable, false);
    }

    #[test]
    fn complex() {
        let cnf = vec![
            vec!["apple", "!cat"],
            vec!["!apple", "sugar_cane", "cat"],
            vec!["!sugar_cane", "apple", "!cat"],
            vec!["cat", "!sugar_cane"],
        ];
        let mut satSolver = SATSolver::parse_cnf(cnf);
        let (satisfiable, solution) = satSolver.solve();
        assert_eq!(satisfiable, true);
        assert_eq!(solution, Some(vec!["apple".to_string(), "cat".to_string()]));
    }
}
