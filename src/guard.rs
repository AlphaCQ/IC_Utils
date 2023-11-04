use std::cell::RefCell;
use std::collections::BTreeSet;
use candid::{CandidType, Deserialize, Principal};

pub struct State {
    pending_requests: BTreeSet<Principal>,
}

thread_local! {
    pub static STATE: RefCell<State> = RefCell::new(State{pending_requests: BTreeSet::new()});
}

#[derive(Deserialize, CandidType, Clone)]
pub struct CallerGuard {
    principal: Principal,
}

impl CallerGuard {
    pub fn new(principal: Principal) -> Result<Self, String> {
        STATE.with(|state| {
            let pending_requests = &mut state.borrow_mut().pending_requests;
            if pending_requests.contains(&principal){
                return Err(format!("Already processing a request for principal {:?}", &principal));
            }
            pending_requests.insert(principal);
            Ok(Self { principal })
        })
    }

    pub fn unlock(principal: &Principal) {
        STATE.with(|state| {
            let flag = state.borrow_mut().pending_requests.remove(principal);
        })
    }
}

impl Drop for CallerGuard {
    fn drop(&mut self) {
        STATE.with(|state| {
            state.borrow_mut().pending_requests.remove(&self.principal);
        })
    }
}
