// This is code from the yew_hooks crate
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

use yew::prelude::*;

/// State handle for the [`use_map`] hook.
pub struct UseMapHandle<K, V> {
    inner: Rc<RefCell<HashMap<K, V>>>,
    update: Rc<dyn Fn()>,
}

impl<K, V> UseMapHandle<K, V> {
    /// Get immutable ref to the map.
    ///
    /// # Panics
    ///
    /// Panics if the value is currently mutably borrowed
    pub fn current(&self) -> Ref<HashMap<K, V>> {
        self.inner.borrow()
    }

    /// Set the hash map.
    pub fn set(&self, map: HashMap<K, V>) {
        *self.inner.borrow_mut() = map;
        (self.update)();
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&self, k: K, v: V) -> Option<V>
    where
        K: Eq + Hash,
    {
        let v = self.inner.borrow_mut().insert(k, v);
        (self.update)();
        v
    }

    /// Update key-value pair.
    pub fn update(&self, k: &K, v: V)
    where
        K: Eq + Hash,
    {
        if let Some(value) = self.inner.borrow_mut().get_mut(k) {
            *value = v;
        }
        (self.update)();
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    pub fn remove(&self, k: &K) -> Option<V>
    where
        K: Eq + Hash,
    {
        let v = self.inner.borrow_mut().remove(k);
        (self.update)();
        v
    }

    /// Retains only the elements specified by the predicate.
    pub fn retain<F>(&self, f: F)
    where
        K: Eq + Hash,
        F: FnMut(&K, &mut V) -> bool,
    {
        self.inner.borrow_mut().retain(f);
        (self.update)();
    }

    /// Clears the map, removing all key-value pairs. Keeps the allocated memory for reuse.
    pub fn clear(&self) {
        self.inner.borrow_mut().clear();
        (self.update)();
    }
}

impl<K, V> Clone for UseMapHandle<K, V> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            update: self.update.clone(),
        }
    }
}

impl<K, V> PartialEq for UseMapHandle<K, V>
where
    K: Eq + Hash,
    V: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}


#[hook]
pub fn use_map<K, V>(initial_value: HashMap<K, V>) -> UseMapHandle<K, V>
where
    K: 'static,
    V: 'static,
{
    let inner = use_mut_ref(|| initial_value);
    let update = use_update();

    UseMapHandle { inner, update }
}

#[hook]
pub fn use_update() -> Rc<dyn Fn()> {
    let state = use_state(|| 0);

    Rc::new(move || {
        state.set((*state + 1) % 1_000_000);
    })
}