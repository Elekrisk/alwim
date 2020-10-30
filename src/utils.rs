use std::{any::{Any, TypeId}, collections::HashMap, ops::Index};


#[derive(Default)]
pub struct TypeContainer {
    inner: HashMap<TypeId, Box<dyn Any>>
}

impl TypeContainer {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new()
        }
    }

    pub fn set<T: 'static>(&mut self, item: T) -> Option<T> {
        self.inner.insert(TypeId::of::<T>(), Box::new(item)).map(|i| i.downcast().ok()).flatten().map(|b| *b)
    }

    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.inner.remove(&TypeId::of::<T>()).map(|b| b.downcast().ok()).flatten().map(|b| *b)
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.inner.get(&TypeId::of::<T>()).map(|b| b.downcast_ref()).flatten()
    }

    pub fn get_or_add_default<T: Default + 'static>(&mut self) -> &T {
        if self.inner.contains_key(&TypeId::of::<T>()) {
            if self.get::<T>().is_some() {
                self.get().unwrap()
            } else {
                self.set(T::default());
                self.get().unwrap()
            }
        } else {
            self.set(T::default());
            self.get().unwrap()
        }
    }

    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.inner.get_mut(&TypeId::of::<T>()).map(|b| b.downcast_mut()).flatten()
    }

    pub fn get_mut_or_add_default<T: Default + 'static>(&mut self) -> &mut T {
        if self.inner.contains_key(&TypeId::of::<T>()) {
            if self.get_mut::<T>().is_some() {
                self.get_mut().unwrap()
            } else {
                self.set(T::default());
                self.get_mut().unwrap()
            }
        } else {
            self.set(T::default());
            self.get_mut().unwrap()
        }
    }
}

pub trait Ignore {
    fn ignore(self);
}

impl<T> Ignore for T {
    fn ignore(self) {
        
    }
}


pub struct Storage<T> {
    inner: Vec<Option<T>>,
    deleted: Vec<usize>,
    next: usize
}

pub struct Id<T> {
    _marker: std::marker::PhantomData<T>,
    id: usize
}

impl<T> Id<T> {
    fn new(id: usize) -> Self {
        Self {
            _marker: std::marker::PhantomData,
            id
        }
    }
}

impl<T> Storage<T> {
    pub fn new() -> Self {
        Self {
            inner: vec![],
            deleted: vec![],
            next: 0
        }
    }

    pub fn add(&mut self, item: T) -> Id<T> {
        let id = if self.deleted.len() > 0 {
            self.deleted.pop().unwrap()
        } else {
            let id = self.next;
            self.next += 1;
            id
        };
        self.inner[id] = Some(item);
        Id::new(id)
    }

    pub fn remove(&mut self, id: Id<T>) -> Option<T> {
        self.deleted.push(id.id);
        self.inner[id.id].take()
    }

    pub fn get(&self, id: Id<T>) -> Option<&T> {
        self.inner.get(id.id).map(|o| o.as_ref()).flatten()
    }

    pub fn get_mut(&mut self, id: Id<T>) -> Option<&mut T> {
        self.inner.get_mut(id.id).map(|o| o.as_mut()).flatten()
    }
}


pub struct Keeper<T, I: Iterator<Item=T>> {
    iter: I,
    buffer: Vec<T>,
    index: usize,
    index_stack: Vec<usize>
}

impl<T, I: Iterator<Item=T>> Keeper<T, I> {
    pub fn new(iter: I) -> Self {
        Self {
            iter,
            buffer: vec![],
            index: 0,
            index_stack: vec![]
        }
    }

    pub fn save_loc(&mut self) {
        self.index_stack.push(self.index);
    }

    pub fn load_loc(&mut self) {
        self.index = self.index_stack.pop().unwrap();
    }

    pub fn discard_loc(&mut self) {
        self.index_stack.pop();
    }

    pub fn next(&mut self) -> Option<&T> {
        self.index += 1;
        let ret = self.get(self.index - 1);
        ret
    }

    pub fn back(&mut self) {
        self.index -= 1;
    }

    pub fn get(&mut self, i: usize) -> Option<&T> {
        if i < 0 { return None; }
        while i >= self.buffer.len() {
            self.buffer.push(self.iter.next()?);
        }

        Some(&self.buffer[i])
    }

    pub fn get_mut(&mut self, i: usize) -> Option<&mut T> {
        if i < 0 { return None; }
        while i >= self.buffer.len() {
            self.buffer.push(self.iter.next()?);
        }

        Some(&mut self.buffer[i])
    }
}
