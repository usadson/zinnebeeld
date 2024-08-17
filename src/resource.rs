// Copyright (C) 2024 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use std::cell::RefCell;

use dashmap::DashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResourceId {
    namespace: ResourceNamespace,
    id: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ResourceNamespace {
    Image,
}

impl ResourceId {
    #[must_use]
    pub fn new(namespace: ResourceNamespace, id: usize) -> Self {
        Self {
            namespace,
            id,
        }
    }

    #[must_use]
    pub const fn namespace(&self) -> ResourceNamespace {
        self.namespace
    }

    #[must_use]
    pub const fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug)]
pub struct ResourceManager<T> {
    namespace: ResourceNamespace,
    id_counter: RefCell<usize>,
    map: DashMap<usize, T>,
}

impl<T> ResourceManager<T> {
    pub fn new(namespace: ResourceNamespace) -> Self {
        Self {
            namespace,
            id_counter: RefCell::new(0),
            map: DashMap::new(),
        }
    }

    pub fn add(&self, value: T) -> ResourceId {
        let id = self.create_id();
        self.map.insert(id.id, value);
        id
    }

    pub fn with<F: FnOnce(&T)>(&self, id: ResourceId, f: F) {
        debug_assert_eq!(id.namespace, self.namespace);

        let val = self.map.get(&id.id).unwrap();
        f(&val)
    }

    fn create_id(&self) -> ResourceId {
        let id = *self.id_counter.borrow();
        *self.id_counter.borrow_mut() += 1;

        ResourceId::new(self.namespace, id)
    }
}
