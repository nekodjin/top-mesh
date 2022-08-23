use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};


pub struct TopMesh<K, V> {
    boundary: HashMap<K, Weak<RwLock<TopMesh<K, V>>>>, // size == rank
    extrusions: HashMap<K, Rc<RwLock<TopMesh<K, V>>>>,
    value: V,
}
