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
