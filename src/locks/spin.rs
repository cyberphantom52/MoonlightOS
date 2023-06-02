use core::cell::UnsafeCell;
use core::sync::atomic::{AtomicBool,Ordering};
use core::fmt;
use core::ops::{Deref, DerefMut};

pub struct SpinLock<T> {
    lock: AtomicBool,
    data: UnsafeCell<T>
}

pub struct SpinLockGuard<'a, T: 'a> {
    mutex: &'a SpinLock<T>,
}

impl<T> SpinLock<T> {
    #[inline(always)]
    pub const fn new(data: T) -> Self {
        Self {
            lock: AtomicBool::new(false),
            data: UnsafeCell::new(data),
        }
    }

    #[inline(always)]
    pub fn lock(&self) -> SpinLockGuard<T> {
        loop {
            if !self.lock.compare_exchange(
                false,
                true,
                Ordering::Acquire,
                Ordering::Relaxed
                ).is_err()
            {
                break SpinLockGuard {
                    mutex: self
                }
            }
        }
    }

    #[inline(always)]
    pub fn try_lock(&self) -> Option<SpinLockGuard<T>> {
        if self
            .lock
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_ok()
        {
            Some(SpinLockGuard {
                mutex: self,
            })
        } else {
            None
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for SpinLock<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.try_lock() {
            Some(guard) => write!(f, "Mutex {{ data: ")
                .and_then(|()| (&guard).fmt(f))
                .and_then(|()| write!(f, "}}")),
            None => write!(f, "Mutex {{ <locked> }}"),
        }
    }
}

unsafe impl<T> Sync for SpinLock<T> {}

impl<T> Drop for SpinLockGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.lock.store(false, Ordering::Release);
    }
}

impl<'a, T: fmt::Debug> fmt::Debug for SpinLockGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&*self, f)
    }
}

impl<'a, T: fmt::Display> fmt::Display for SpinLockGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&*self, f)
    }
}

impl<'a, T> Deref for SpinLockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe {
            &*self.mutex.data.get()
        }
    }
}

impl<'a, T> DerefMut for SpinLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.data.get() }
    }
}

unsafe impl<T> Sync for SpinLockGuard<'_, T> {}