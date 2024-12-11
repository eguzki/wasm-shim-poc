use super::PolicyChain;
use std::rc::Rc;

#[derive(Default)]
pub struct PolicyChainIndex {}

impl PolicyChainIndex {
    pub fn get_longest_match(&self, _domain_name: &str) -> Option<Rc<PolicyChain>> {
        // read index and return policy chain
        todo!();
    }
}
