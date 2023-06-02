use crate::locks::spin::{SpinLock, SpinLockGuard};
use core::fmt;
use core::ops::{Deref, DerefMut};

pub struct Mutex<T> {
    mutex: SpinLock<T>
}

pub struct MutexGuard<'a, T: 'a> {
    guard: SpinLockGuard<'a, T>,
}

impl<T> Mutex<T> {
    #[inline(always)]
    pub const fn new(value: T) -> Self {
        Self {
            mutex: SpinLock::new(value),
        }
    }

    pub fn lock(&self) -> MutexGuard<T> {
        MutexGuard {
            guard: self.mutex.lock(),
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for Mutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.mutex, f)
    }
}

unsafe impl<T> Sync for Mutex<T> {}


impl<'a, T: fmt::Debug> fmt::Debug for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&*self, f)
    }
}

impl<'a, T: fmt::Display> fmt::Display for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&*self, f)
    }
}

impl<'a, T:> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        &*self.guard
    }
}

impl<'a, T:> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut *self.guard
    }
}
