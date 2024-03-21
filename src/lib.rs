use windows::{Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK}, Win32::System::SystemServices::*,};
use windows::core::s;

#[no_mangle]
#[allow(non_snake_case)]
fn DllMain(_: usize, dw_reason: u32, _: usize) -> i32 {
    match dw_reason {
        DLL_PROCESS_ATTACH => attach(),
        _ => (),
    }

    1
}

fn attach() {
    unsafe {
        MessageBoxA(None, s!("Hello from Rust DLL"), s!("Hello from Rust DLL"), MB_OK);
    }
}