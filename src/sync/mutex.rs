use std::collections::HashMap;
use std::sync::{Arc, Weak};
use std::sync::{Mutex, MutexGuard};


pub struct TopMesh<K, V> {
    boundary: HashMap<K, Weak<Mutex<TopMesh<K, V>>>>, // size == rank
    extrusions: HashMap<K, Arc<Mutex<TopMesh<K, V>>>>,
    value: V,
}
