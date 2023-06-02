use crate::locks::spin::{SpinLock, SpinLockGuard};
use core::fmt;
use core::ops::{Deref, DerefMut};

pub struct Mutex<T: ?Sized> {
    mutex: SpinLock<T>
}

pub struct MutexGuard<'a, T: 'a + ?Sized> {
    guard: SpinLockGuard<'a, T>,
}

impl<T> Mutex<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            mutex: SpinLock::new(value),
        }
    }
}

impl<T: ?Sized> Mutex<T> {
    pub fn lock(&self) -> MutexGuard<T> {
        MutexGuard {
            guard: self.mutex.lock(),
        }
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.mutex, f)
    }
}

unsafe impl<T: ?Sized> Sync for Mutex<T> {}


impl<'a, T: ?Sized + fmt::Debug> fmt::Debug for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a, T: ?Sized + fmt::Display> fmt::Display for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.guard
    }
}

impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.guard
    }
}
