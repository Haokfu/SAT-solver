In this assignment, we will be using Rust to make a SAT solver.
Given a boolean formula, a SAT solver determines whether there exists an assignment of variables to
true or false, such that the boolean formula evaluates to true.
SAT solving is proven to be NP-complete, so in worst case, we do not have any algorithms that solve
it in less than exponential time in the worst case. There is a polynomial-time reduction from arbitrary
boolean formulas to boolean formulas in conjunctive normal form. These boolean formulas are written as:
Atom := v
| not v
Clause := Atom_1 \/ ... \/ Atom_n
Formula := Clause_1 /\ ... /\ Clause_n
Formulas are a list of “Clauses” and-ed together. Clauses are lists of “Atoms” and-ed together. Atoms
are either variables, or the negation of variables.
In the context of Rust, Variables are represented as chars. Atoms are represented as the enum
enum Atom {Base(Variable), Not(Variable). Clauses are represented by vectors of atoms. Full formulas
are represented by vectors of clauses.
Thus, the Rust data: vec![vec![Base(b),Not(c)],vec![Base(c)]] represents the boolean formula
(b ∨ ¬c) ∧ c.
What does an empty formula represent? The Rust data vec![] represents the formula true. What
does an empty clause represent? The Rust data vec![vec![Base(a),Base(b)],vec![]] represents the
formula (a ∨ b) ∧ false.
While we do not have polynomial time algorithms, there exist algorithms that do relatively well in the
average case, that operate over CNF boolean formulas. One of the first such algorithms is DPLL. In this
assignment, we will be writing a DPLL algorithm.
The fundamental DPLL algorithm works as follows:
procedure DPLL(Φ)
UnitPropogate(Φ)
AssignPureVars(Φ)
If Φ is empty, return true
If Φ contains an empty clause, return false
v ← ChooseVariable(Φ)
return DPLL(Φ ∧ {v}) or DPLL(Φ ∧ {¬v})
Note that DPLL returns true when Φ is empty because an empty Φ is just true. Note that DPLL returns
false when Φ contains an empty clause because an empty clause is just false, so the conjunction of anything
and false is false.
In this assignment, you will write some portions of the UnitPropogate procedure, some portions of
the AssignPureVars procedure, and the full DPLL loop.
You’ve been provided a partial test suite. You can run these tests by running cargo test. You can also
build your own formula, and get a result of whether or not it is satisfiable by running cargo run.
