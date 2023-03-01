use z3::BoolRef;
use hash::hash;
use uuid;

struct NamedUIDObject {
    uid: i64,
    name: String,
    z3_assertions: Vec<BoolRef>,
    z3_assertion_hashes: Vec<BoolRef>
}

impl NamedUIDObject {
    pub fn append_z3_assertion(&mut self, z3_assertion: BoolRef) {
        let assertion_hash = hash(z3_assertion);
        self.z3_assertions.push(z3_assertion);
        self.z3_assertion_hashes.push(assertion_hash);

    }
    pub fn get_z3_assertion(&self) -> Vec<BoolRef> {
        self.z3_assertions
    }
}