use super::Policy;
use std::rc::Rc;

#[derive(Default)]
pub struct PolicyChainIndex {}

impl PolicyChainIndex {
    pub fn get_longest_match(&self, _domain_name: &str) -> Option<Rc<[Policy]>> {
        // read index and return policy chain
        todo!();
    }
}
