use std::collections::HashMap;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

/// Represents a constraint in the CHR system.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Constraint {
    /// The GCD constraint, representing gcd(n).
    Gcd(i32),
}

/// The constraint store, which holds all the active constraints.
struct ConstraintStore {
    /// The constraints are stored in a HashMap, mapping a unique ID to each constraint.
    constraints: HashMap<usize, Constraint>,
    /// The next available ID for a new constraint.
    next_id: usize,
}

impl ConstraintStore {
    fn new() -> Self {
        ConstraintStore {
            constraints: HashMap::new(),
            next_id: 0,
        }
    }

    fn add_constraint(&mut self, constraint: Constraint) -> usize {
        let id = self.next_id;
        info!("Adding constraint: {:?} with id {}", constraint, id);
        self.constraints.insert(id, constraint);
        self.next_id += 1;
        id
    }

    fn remove_constraint(&mut self, id: usize) {
        info!("Removing constraint with id {}", id);
        self.constraints.remove(&id);
    }

    fn get_constraints_with_ids(&self) -> Vec<(usize, &Constraint)> {
        self.constraints.iter().map(|(id, c)| (*id, c)).collect()
    }

    fn get_constraints(&self) -> Vec<&Constraint> {
        self.constraints.values().collect()
    }
}

/// A trait for all CHR rules.
trait Rule {
    /// Applies the rule to the given constraint store.
    /// Returns `true` if the store was modified, `false` otherwise.
    fn apply(&self, store: &mut ConstraintStore) -> bool;
}

/// The `cleanup @ gcd(0) <=> true` rule.
/// This is a simplification rule that removes any `gcd(0)` constraint.
struct GcdCleanupRule;

impl Rule for GcdCleanupRule {
    fn apply(&self, store: &mut ConstraintStore) -> bool {
        let mut changed = false;
        let mut ids_to_remove = Vec::new();

        for (id, constraint) in store.get_constraints_with_ids() {
            if let Constraint::Gcd(0) = constraint {
                ids_to_remove.push(id);
                changed = true;
            }
        }

        for id in ids_to_remove {
            store.remove_constraint(id);
        }

        changed
    }
}

/// The `gcd(N) \ gcd(M) <=> 0 < N, N <= M | gcd(M % N)` rule.
/// This is a simpagation rule that, given two constraints `gcd(N)` and `gcd(M)`,
/// if the guard `0 < N, N <= M` holds, it removes `gcd(M)` and adds `gcd(M % N)`.
struct GcdSimpagationRule;

impl Rule for GcdSimpagationRule {
    fn apply(&self, store: &mut ConstraintStore) -> bool {
        let constraints = store.get_constraints_with_ids();
        let mut id_to_remove = None;
        let mut constraint_to_add = None;

        for (id1, constraint1) in &constraints {
            for (id2, constraint2) in &constraints {
                if id1 == id2 {
                    continue;
                }

                let (Constraint::Gcd(n), Constraint::Gcd(m)) = (constraint1, constraint2);
                if *n > 0 && *n <= *m {
                    id_to_remove = Some(*id2);
                    constraint_to_add = Some(Constraint::Gcd(*m % *n));
                    break;
                }
            }
            if id_to_remove.is_some() {
                break;
            }
        }

        if let Some(id) = id_to_remove {
            store.remove_constraint(id);
            store.add_constraint(constraint_to_add.unwrap());
            true
        } else {
            false
        }
    }
}


/// The CHR engine, which manages the rules and the constraint store.
struct CHREngine<'a> {
    /// The rules to be applied.
    rules: Vec<Box<dyn Rule + 'a>>,
    /// The constraint store.
    store: ConstraintStore,
}

impl<'a> CHREngine<'a> {
    /// Creates a new CHR engine.
    fn new() -> Self {
        CHREngine {
            rules: Vec::new(),
            store: ConstraintStore::new(),
        }
    }

    /// Adds a rule to the engine.
    fn add_rule(&mut self, rule: Box<dyn Rule + 'a>) {
        self.rules.push(rule);
    }

    /// Adds a constraint to the engine's store.
    fn add_constraint(&mut self, constraint: Constraint) {
        self.store.add_constraint(constraint);
    }

    /// Runs the CHR engine.
    /// It repeatedly applies the rules to the constraint store until no more rules can be applied.
    fn run(&mut self) {
        let mut changed = true;
        while changed {
            changed = false;
            for rule in &self.rules {
                if rule.apply(&mut self.store) {
                    info!("Applied a rule. Store is now: {:?}", self.store.get_constraints());
                    changed = true;
                }
            }
        }
    }

    /// Returns a vector of the constraints in the store.
    fn get_constraints(&self) -> Vec<&Constraint> {
        self.store.get_constraints()
    }
}

fn main() {
    // Initialize the logger to see the execution trace.
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();

    // Create a new CHR engine.
    let mut engine = CHREngine::new();

    // Add the GCD rules to the engine.
    engine.add_rule(Box::new(GcdCleanupRule));
    engine.add_rule(Box::new(GcdSimpagationRule));

    // Add the initial constraints to the engine.
    engine.add_constraint(Constraint::Gcd(12));
    engine.add_constraint(Constraint::Gcd(8));

    info!("Initial constraints: {:?}", engine.get_constraints());

    // Run the engine to solve the constraints.
    engine.run();

    // Print the final constraints.
    info!("Final constraints: {:?}", engine.get_constraints());
    println!("Final constraints: {:?}", engine.get_constraints());
}
