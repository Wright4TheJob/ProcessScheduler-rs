use crate::base::NamedUIDObject;

pub struct NonConcurrentBuffer {
    name: String,
    initial_state: Option<i64>,
    final_state: Option<i64>,
    lower_bound: Option<i64>,
    upper_bound: Option<i64>,
}

impl NonConcurrentBuffer {
    pub fn new(
        name: String, 
        initial_state: Option<i64>,
        final_state: Option<i64>,
        lower_bound: Option<i64>,
        upper_bound: Option<i64>,) -> NonConcurrentBuffer {
            NonConcurrentBuffer {
                name, 
                initial_state,
                final_state,
                lower_bound,
                upper_bound,
            }
        }
}