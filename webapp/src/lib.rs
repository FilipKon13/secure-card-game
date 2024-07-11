mod pri_sel;
mod server;

use std::sync::{Arc, Condvar, Mutex};

pub use pri_sel::WebInterface;
use server::start_server;

#[derive(Clone, Debug)]
struct MutCondVarPair {
    mutex_and_cond_var: Arc<(Mutex<Option<usize>>, Condvar)>,
}

impl MutCondVarPair {
    fn mutex(&self) -> &Mutex<Option<usize>> {
        &self.mutex_and_cond_var.0
    }
    fn cond_var(&self) -> &Condvar {
        &self.mutex_and_cond_var.1
    }
}

pub fn get_web_interface(port: u16) -> WebInterface {
    let (handle, mcv, ws) = start_server(port);
    WebInterface { mcv, ws, handle }
}
