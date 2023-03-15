//use hash::hash;
use uuid::Uuid;
use z3::SortKind;
use std::fmt;

struct NamedUIDObject {
    uid: u128,
    name: String,
    kind: ObjectKind,
    z3_assertions: Vec<SortKind>,
    z3_assertion_hashes: Vec<SortKind>
}

impl NamedUIDObject {
    pub fn new(name: Option<String>, kind: Option<ObjectKind>) -> NamedUIDObject{
        let new_uid = Uuid::new_v4();
        NamedUIDObject {
            uid: new_uid.as_u128(),
            name: match name {
                Some(n) => n,
                None => match kind {
                    Some(k) => k.name.clone() + "_" + &new_uid.to_string()[..8],
                    None => new_uid.to_string()[..8].to_string(),
                },
            },
            kind: ObjectKind { name: String::from("Default") },
            z3_assertions: Vec::new(),
            z3_assertion_hashes: Vec::new(),
        }
    }
    pub fn hash(&self) -> u128 {
        self.uid
    }
    
    pub fn eq(&self, other: &NamedUIDObject) -> bool {
        self.uid == other.uid
    }

    pub fn repr(&self) -> String {
        let mut string_to_return = format!("{}({})\n{} assertion(s):\n", self.name, self.kind.name, 1);//self.z3_assertions.len());
        //for assert in self.z3_assertions{
        //    string_to_return += format!("{}",assert.to_string());
        //}
        string_to_return
    }

/*
    pub fn append_z3_assertion(&mut self, z3_assertion: Bool) {
        let assertion_hash = hash(z3_assertion);
        self.z3_assertions.push(z3_assertion);
        self.z3_assertion_hashes.push(assertion_hash);
    }
    pub fn get_z3_assertion(&self) -> Vec<Bool> {
        self.z3_assertions
    } */
}

impl fmt::Display for NamedUIDObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_to_return = format!("{}({}) has {} assertion(s)", self.name, self.kind.name, 1);//self.z3_assertions.len());
        write!(f, "{}",string_to_return)
    }
}

struct ObjectKind {
    name: String,
}