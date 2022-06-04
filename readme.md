# winapi-mon
Monitor winapi functions by injecting dll.  

Please use at your own risk.  
I cannot guarantee anything.  
Only few hooks are available for now.  

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

For the example sake, i'll implement [CreateThread](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread) hook.  
  

## [Step 1] Research about the target function.  

[CreateThread(microsoft doc)](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)  

The function definition is.  

```C++
HANDLE CreateThread(
    LPSECURITY_ATTRIBUTES   lpThreadAttributes,
    SIZE_T                  dwStackSize,
    LPTHREAD_START_ROUTINE  lpStartAddress,
    LPVOID lpParameter,
    DWORD                   dwCreationFlags,
    LPDWORD                 lpThreadId
);
```

So our hook's type is
```Rust
type FnCreateThread = extern "system" fn(LPSECURITY_ATTRIBUTES, SIZE_T, LPTHREAD_START_ROUTINE, LPVOID, DWORD, LPDWORD) -> HANDLE;  
```
  
If your are not sure about what is extern "system", please refer https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions  

Addition: [stdcall(microsoft doc)](https://docs.microsoft.com/en-us/cpp/cpp/stdcall)  

## [Step 2] Prepare source files.

Check the document([CreateThread](https://docs.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createthread)) and see in which header file the function is defined.  
In this case the fuction is defined in 'processthreadsapi.h'.

Check if there is a folder with the header file name in winapi-mon-core/src . (In this case 'processthreadsapi')  

If there is no such folder, please create a folder and mod.rs in it.
Then include created module by adding it in 'winapi-mon-core/src/lib.rs'.

In the folder create TargetFunctionName.rs (so CreateThread.rs). (At the time of writing, I use snake_case for source file names. I will replace all source file names with CamelCase later.)  

Add the created module(CreateThread) to mod.rs.
```Rust
mod CreateThread;
```

## [Step 3] Write a default hook.
In the created source file. (CreateThread.rs)  

We need to write a hook first.  

TODO!



# TODO
- [ ] Eliminate the compiler warnings. 
- [ ] Complete the readme.