use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
enum Literal {
    Must,
    MustNot,
    Absent,
}

#[derive(Debug)]
struct Clause {
    disjunction: Vec<Literal>,
}
impl Clause {
    pub fn is_empty(&self) -> bool {
        !self.disjunction.iter().any(|lit| match lit {
            Literal::Must | Literal::MustNot => true,
            _ => false,
        })
    }
}

#[derive(Debug)]
struct SATSolver {
    clauses: Vec<Clause>,
    total_literals: usize,
    literal_to_id: HashMap<String, usize>,
    id_to_literal: HashMap<usize, String>,
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
        }
    }

    fn read_literal(literal: &String) -> (Literal, String) {
        if literal.starts_with('!') {
            (Literal::MustNot, literal[1..].to_string())
        } else {
            (Literal::Must, literal.clone())
        }
    }

    pub fn dpll() {
        todo!();
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
}
