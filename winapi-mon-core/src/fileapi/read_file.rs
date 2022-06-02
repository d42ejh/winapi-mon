use crate::{declare_init_hook, get_detour};
use anyhow::Result;
use detour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use nameof::name_of;
use std::lazy::{SyncLazy, SyncOnceCell};
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::minwindef::{BOOL, DWORD, FALSE, HINSTANCE, LPDWORD, LPVOID, TRUE};
use winapi::um::fileapi::ReadFile;
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::um::winnt::HANDLE;
type FnReadFile = extern "system" fn(HANDLE, LPVOID, DWORD, LPDWORD, LPOVERLAPPED) -> BOOL;

static ReadFileDetour: SyncOnceCell<Arc<RwLock<GenericDetour<FnReadFile>>>> = SyncOnceCell::new();
declare_init_hook!(
    hook_ReadFile,
    FnReadFile,
    ReadFileDetour,
    "kernel32",
    name_of!(ReadFile),
    __hook__ReadFile
);

//tfw no decltype
pub extern "system" fn __hook__ReadFile(
    hFile: HANDLE,
    lpBuffer: LPVOID,
    nNumberOfBytesToRead: DWORD,
    lpNumberOfBytesRead: LPDWORD,
    lpOverlapped: LPOVERLAPPED,
) -> BOOL {
    event!(
        Level::INFO,
        "[{}] {} {:?}, {} {}",
        name_of!(ReadFile),
        name_of!(lpBuffer),
        lpBuffer,
        name_of!(nNumberOfBytesToRead),
        nNumberOfBytesToRead
    );

    // call trampoline
    let f = get_detour!(ReadFileDetour);

    unsafe {
        f.call(
            hFile,
            lpBuffer,
            nNumberOfBytesToRead,
            lpNumberOfBytesRead,
            lpOverlapped,
        )
    }
}
