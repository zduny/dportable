use std::{
    cell::RefCell,
    fmt::Display,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub struct Mutex<T>(pub RefCell<T>)
where
    T: ?Sized;

#[derive(Debug)]
pub struct MutexGuard<'a, T>(std::cell::RefMut<'a, T>)
where
    T: ?Sized + 'a;

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Display for MutexGuard<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Mutex(RefCell::new(value))
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        MutexGuard(self.0.borrow_mut())
    }

    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.0.try_borrow_mut().ok().map(MutexGuard)
    }
}

#[derive(Debug)]
pub struct RwLock<T>(pub RefCell<T>)
where
    T: ?Sized;

#[derive(Debug)]
pub struct RwLockReadGuard<'a, T>(std::cell::Ref<'a, T>)
where
    T: ?Sized + 'a;

impl<T> Display for RwLockReadGuard<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T> Deref for RwLockReadGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct RwLockWriteGuard<'a, T>(std::cell::RefMut<'a, T>)
where
    T: ?Sized + 'a;

impl<T> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Display for RwLockWriteGuard<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        RwLock(RefCell::new(value))
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLockReadGuard(self.0.borrow())
    }

    pub fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        self.0.try_borrow().ok().map(RwLockReadGuard)
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        RwLockWriteGuard(self.0.borrow_mut())
    }

    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        self.0.try_borrow_mut().ok().map(RwLockWriteGuard)
    }
}
