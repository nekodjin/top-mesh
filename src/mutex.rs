use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::{Mutex, MutexGuard};


pub struct TopMesh<K, V> {
    boundary: HashMap<K, Weak<Mutex<TopMesh<K, V>>>>, // size == rank
    extrusions: HashMap<K, Rc<Mutex<TopMesh<K, V>>>>,
    value: V,
}
