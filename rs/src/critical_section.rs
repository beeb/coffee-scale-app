//! Critical section implementation for ESP-IDF
//!
//! This module provides a critical section implementation that uses `IsrCriticalSection`, which prevents any interrupt
//! from happening while the critical section is acquired.
//!
//! This is necessary, because the hx711 reading must happen with precise timing, and any interrupt could disrupt it.
use std::sync::Mutex;

use esp_idf_svc::hal::interrupt::{IsrCriticalSection, IsrCriticalSectionGuard};

static CS: IsrCriticalSection = IsrCriticalSection::new();
static CS_GUARD: Mutex<Option<IsrCriticalSectionGuard>> = Mutex::new(None);

pub struct EspCriticalSection {}

unsafe impl critical_section::Impl for EspCriticalSection {
    unsafe fn acquire() {
        let mut guard = CS_GUARD.lock().unwrap();
        *guard = Some(CS.enter());
    }

    unsafe fn release(_token: ()) {
        let mut guard = CS_GUARD.lock().unwrap();
        *guard = None;
    }
}

critical_section::set_impl!(EspCriticalSection);
