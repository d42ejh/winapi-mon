# winapi-mon
Monitor winapi functions by injecting dll.  

# winapi-mon-core
Core library.  

# winapi-mon-dll
Sample dll.  

# Usage
Please forget about the unwrap() s.  
## Log winapi function with tracing library.
```Rust
// import 
use winapi_mon_core::synchapi::{hook_Sleep,SleepDetour};

fn usage(){
    let h = hook_Sleep(None, true).unwrap();
    let h=h.write();

    // disable the hook
    unsafe{h.disable()}.unwrap();
    
    // enable the hook
    unsafe{h.enable()}.unwrap();

    debug_assert!(h.is_enabled());

    
}

fn your_main_function(){
    unsafe { AllocConsole() };// need to call this to enable console
    
    // enable nice colored console
    ansi_term::enable_ansi_support().unwrap();

    // init tracing
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_max_level(tracing::Level::TRACE)
        .init();

    usage();
}
```

hook_XXX's ( in this case hook_Sleep ) first argument is an optional your hook.  
If you provide None, default hook is used.  
If you provide Some(your_hook), your_hook is used instead of a default one.  
  
hook_XXX's second argument is bool.  
Provide true to enable the 'hook' right after the hooking.  
Provide false to not to enable.  (You can manually enable the hook later)


## Use custom hook
```Rust
// import 
use winapi_mon_core::winuser::{hook_PeekMessageA,PeekMessageADetour};
use winapi_mon_core::{
    caller_address, get_detour, utility::return_address
};
// stdcall (so system)
// refer https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions
// PeekMessageA: https://docs.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-peekmessagea
// Our custom PeekMessageA hook function.
extern "system" fn __hook__PeekMessageA(
    lpMsg: LPMSG,
    hWnd: HWND,
    wMsgFilterMin: UINT,
    wMsgFileterMax: UINT,
    wRemoveMsg: UINT,
) -> BOOL{
    // In order to get a message, we need to call PeekMessageA function.
    // So call saved trampoline to archive that.
    
    // Get detour with the convenience macro.
    let detour = get_detour!(PeekMessageADetour);

    // Call trampoline
    let return_val = unsafe { f.call(lpMsg, hWnd, wMsgFilterMin, wMsgFileterMax, wRemoveMsg) };

    // Profit here.
    let ret_address=caller_address!();
    println!("PeekMessageA called! return address: {:p}",ret_address);


    // Return the value
    return_val
}

fn usage(){
    let _ = hook_PeekMessageA(
        Some(__hook__PeekMessageA),//Provide our custom hook.
        true,
    )?;
}

fn your_main_function(){
    unsafe { AllocConsole() };// need to call this to enable console
    
    // enable nice colored console
    ansi_term::enable_ansi_support().unwrap();

    // init tracing
    tracing_subscriber::fmt()
        .pretty()
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_max_level(tracing::Level::TRACE)
        .init();

    usage();
}
```

## I want to hook XXX but XXX is not in the winapi-mon-core yet.
Implement new hook is easy. (and lazy)  
Here's how to do it.  

## Step 1
todo