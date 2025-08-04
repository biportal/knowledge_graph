use std::collections::HashMap;
use log::{info, LevelFilter};
use simple_logger::SimpleLogger;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Constraint {
    Gcd(i32),
}

struct ConstraintStore {
    constraints: HashMap<usize, Constraint>,
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

// A trait for all CHR rules
trait Rule {
    // apply takes the store and returns true if the store was modified
    fn apply(&self, store: &mut ConstraintStore) -> bool;
}

// Rule 1: cleanup @ gcd(0) <=> true
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

// Rule 2: gcd(N) \ gcd(M) <=> 0 < N, N <= M | gcd(M % N)
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


struct CHREngine<'a> {
    rules: Vec<Box<dyn Rule + 'a>>,
    store: ConstraintStore,
}

impl<'a> CHREngine<'a> {
    fn new() -> Self {
        CHREngine {
            rules: Vec::new(),
            store: ConstraintStore::new(),
        }
    }

    fn add_rule(&mut self, rule: Box<dyn Rule + 'a>) {
        self.rules.push(rule);
    }

    fn add_constraint(&mut self, constraint: Constraint) {
        self.store.add_constraint(constraint);
    }

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

    fn get_constraints(&self) -> Vec<&Constraint> {
        self.store.get_constraints()
    }
}

fn main() {
    SimpleLogger::new().with_level(LevelFilter::Info).init().unwrap();
    let mut engine = CHREngine::new();

    engine.add_rule(Box::new(GcdCleanupRule));
    engine.add_rule(Box::new(GcdSimpagationRule));

    engine.add_constraint(Constraint::Gcd(12));
    engine.add_constraint(Constraint::Gcd(8));

    info!("Initial constraints: {:?}", engine.get_constraints());

    engine.run();

    info!("Final constraints: {:?}", engine.get_constraints());
    println!("Final constraints: {:?}", engine.get_constraints());
}
