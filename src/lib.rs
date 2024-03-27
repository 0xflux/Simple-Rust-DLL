use windows::{Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK}, Win32::System::SystemServices::*,};
use windows::core::s;
use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::LibraryLoader::FreeLibraryAndExitThread;
use windows::Win32::System::Threading::{CreateThread, LPTHREAD_START_ROUTINE, THREAD_CREATION_FLAGS};
use windows::Win32::UI::WindowsAndMessaging::MB_ICONERROR;

static mut HMODULE_INSTANCE: HINSTANCE = HINSTANCE(0); // handle to the module instance of the injected dll

#[no_mangle]
#[allow(non_snake_case)]
fn DllMain(hmod_instance: HINSTANCE, dw_reason: u32, _: usize) -> i32 {
    match dw_reason {
        DLL_PROCESS_ATTACH => unsafe {
            HMODULE_INSTANCE = hmod_instance; // set a handle to the module for a clean unload
            attach(); // actual entrypoint into the implant, there you may wish to block execution
            spawn_thread_for_unloading_dll(); // finally, unload the DLL when it's finished doing its routines.
        },
        _ => (),
    }

    1
}

/// Entrypoint to the actual implant once execution goes into DLL_PROCESS_ATTACH. Think of this as
/// calling a function to start something from main().
fn attach() {
    unsafe {
        MessageBoxA(None, s!("Hello from Rust DLL"), s!("Hello from Rust DLL"), MB_OK);
    }
}

/// Spawn a new thread in the current injected process, calling a function pointer to some code which
/// will unload this very same DLL from the process. A popup box confirms either success or failure.
fn spawn_thread_for_unloading_dll() {
    unsafe {
        // convert unload_thread to a start routine
        let thread_start: LPTHREAD_START_ROUTINE = Some(unload_dll);

        // create a thread to unload the DLL from the current process
        let thread_handle = CreateThread(
            None,
            0,
            thread_start,
            None,
            THREAD_CREATION_FLAGS(0),
            None,
        );

        match thread_handle {
            Ok(_) => {MessageBoxA(None, s!("Unloaded"), s!("Unloaded"), MB_OK);}
            Err(_) => {MessageBoxA(None, s!("Could not unload"), s!("Could not unload"), MB_ICONERROR);}
        }
    }
}

#[no_mangle]
/// Unload the DLL by its handle, so that there is no live evidence of hte DLL in memory after its
/// finished its business, plus allows for loading multiple of the same DLL into the same process
unsafe extern "system" fn unload_dll(_lpthread_param: *mut core::ffi::c_void) -> u32 {
    FreeLibraryAndExitThread(HMODULE_INSTANCE, 1);
}