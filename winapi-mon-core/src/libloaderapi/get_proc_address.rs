use crate::{declare_init_hook, get_detour};
use anyhow::Result;
use detour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use lazy_static::lazy_static;
use nameof::name_of;
use std::iter::Once;
use std::lazy::SyncOnceCell;
use std::sync::Mutex;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{
    BOOL, DWORD, FALSE, FARPROC, HINSTANCE, HMODULE, LPDWORD, LPVOID, PDWORD, TRUE,
};
use winapi::shared::ntdef::NULL;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA, LoadLibraryW};
use winapi::um::minwinbase::LPOVERLAPPED;
use winapi::um::winnt::{LPCSTR, LPCWSTR};

//https://docs.microsoft.com/en-us/windows/win32/api/libloaderapi/nf-libloaderapi-getprocaddress

type FnGetProcAddress = extern "system" fn(HMODULE, LPCSTR) -> FARPROC;

static GetProcAddressDetour: SyncOnceCell<Arc<RwLock<GenericDetour<FnGetProcAddress>>>> =
    SyncOnceCell::new();

declare_init_hook!(
    hook_GetProcAddress,
    FnGetProcAddress,
    GetProcAddressDetour,
    "kernel32",
    name_of!(GetProcAddress),
    __hook__GetProcAddress
);

pub extern "system" fn __hook__GetProcAddress(hModule: HMODULE, lpProcName: LPCSTR) -> FARPROC {
    // call trampoline

    let f = get_detour!(GetProcAddressDetour);

    let ret = unsafe { f.call(hModule, lpProcName) };
    event!(
        Level::INFO,
        "[{}] {} {:?} {} {:?} Return Value: {:x}",
        name_of!(GetProcAddress),
        name_of!(hModule),
        hModule,
        name_of!(lpProcName),
        lpProcName,
        ret as usize
    );
    ret
}
