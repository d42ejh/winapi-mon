use crate::{declare_init_hook, get_detour};
use anyhow::Result;
use detour::{static_detour, Error, GenericDetour, RawDetour, StaticDetour};
use nameof::name_of;
use std::lazy::SyncOnceCell;
use std::sync::{Arc, RwLock};
use tracing::{event, Level};
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{
    BOOL, DWORD, FALSE, HINSTANCE, LPDWORD, LPVOID, LRESULT, PDWORD, TRUE,
};
use winapi::um::winnt::{HANDLE, LPCSTR, LPSTR};
use winapi::um::winuser::{DispatchMessageW, MSG};

//only W is implemented

type FnDispatchMessageW = extern "system" fn(*const MSG) -> LRESULT;

static DispatchMessageWDetour: SyncOnceCell<Arc<RwLock<GenericDetour<FnDispatchMessageW>>>> =
    SyncOnceCell::new();

declare_init_hook!(
    hook_DispatchMessageW,
    FnDispatchMessageW,
    DispatchMessageWDetour,
    "USER32",
    name_of!(DispatchMessageW),
    __hook__DispatchMessageW
);

extern "system" fn __hook__DispatchMessageW(lpmsg: *const MSG) -> LRESULT {
    event!(Level::INFO, "[{}] {:?}", name_of!(DispatchMessageW), lpmsg);
    // call trampoline
    let f = get_detour!(DispatchMessageWDetour);

    unsafe { f.call(lpmsg) }
}
