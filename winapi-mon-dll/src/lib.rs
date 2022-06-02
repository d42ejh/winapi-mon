#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
use tracing::{event, Level};
use winapi::{
    shared::minwindef::{
        BOOL, DWORD, FALSE, HINSTANCE, LPARAM, LPVOID, LRESULT, TRUE, UINT, WPARAM,
    },
    shared::windef::HWND,
    um::consoleapi::AllocConsole,
    um::libloaderapi::DisableThreadLibraryCalls,
    um::libloaderapi::{GetModuleHandleA, GetProcAddress},
    um::wincon::FreeConsole,
    um::winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
};

fn attached_main() -> anyhow::Result<()> {
    unsafe { AllocConsole() };
    ansi_term::enable_ansi_support().unwrap();

    // let file_appender = tracing_appender::rolling::never("log", "winapi-mon.log"); //uncommnet this to use file log
    tracing_subscriber::fmt()
        //    .with_writer(file_appender) //uncommnet this to use file log
        .pretty()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_max_level(tracing::Level::TRACE)
        .init();

    event!(Level::INFO, "Initialized the logger!");

    let detour = winapi_mon_core::fileapi::hook_ReadFile(None)?;
    let detour = detour.write().unwrap();
    unsafe { detour.enable() }?;

    let detour = winapi_mon_core::fileapi::hook_GetFinalPathNameByHandleA(None)?;
    let detour = detour.write().unwrap();
    unsafe { detour.enable() }?;

    let detour = winapi_mon_core::libloaderapi::hook_LoadLibraryA(None)?;
    let detour = detour.write().unwrap();
    unsafe { detour.enable() }?;

    let detour = winapi_mon_core::libloaderapi::hook_LoadLibraryW(None)?;
    let detour = detour.write().unwrap();
    unsafe { detour.enable() }?;

    let detour = winapi_mon_core::libloaderapi::hook_GetProcAddress(None)?;
    let detour = detour.write().unwrap();
    unsafe { detour.enable() }?;

    let detour = winapi_mon_core::fileapi::hook_CreateFileA(None)?;
    let detour = detour.write().unwrap();
    unsafe { detour.enable() }?;

    //let d = winapi_mon_core::synchapi::hook_Sleep(None)?; //provide Some(hook) to use your own hook function
    // winapi_mon_core::memoryapi::hook_VirtualProtect(None)?;

    event!(Level::INFO, "All Done");

    Ok(())
}

#[no_mangle]
#[allow(non_snake_case)]
extern "system" fn DllMain(dll_module: HINSTANCE, call_reason: DWORD, _: LPVOID) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            unsafe { DisableThreadLibraryCalls(dll_module) };
            attached_main().unwrap()
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }
    TRUE
}
