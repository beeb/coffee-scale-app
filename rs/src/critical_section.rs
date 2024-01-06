use std::sync::{Mutex, OnceLock};

use esp_idf_svc::hal::interrupt::{IsrCriticalSection, IsrCriticalSectionGuard};

static CS: IsrCriticalSection = IsrCriticalSection::new();
static CS_GUARD: OnceLock<Mutex<Option<IsrCriticalSectionGuard>>> = OnceLock::new();

pub struct EspCriticalSection {}

unsafe impl critical_section::Impl for EspCriticalSection {
    unsafe fn acquire() {
        let mut guard = CS_GUARD.get_or_init(|| Mutex::new(None)).lock().unwrap();
        *guard = Some(CS.enter());
    }

    unsafe fn release(_token: ()) {
        let mut guard = CS_GUARD.get().unwrap().lock().unwrap();
        *guard = None;
    }
}

critical_section::set_impl!(EspCriticalSection);
