use tracing::{event, Level};
use winapi::um::consoleapi::AllocConsole;
use winapi::um::synchapi::{Sleep, SleepEx};
use winapi_mon_core::sys::hook_Sleep;

//Had to write this to identify sleep hook bug.
fn main() {
    unsafe { AllocConsole() };
    ansi_term::enable_ansi_support().unwrap();

    tracing_subscriber::fmt()
        .pretty()
        .with_thread_ids(true)
        .with_thread_names(true)
        // enable everything
        .with_max_level(tracing::Level::TRACE)
        // sets this to be the default, global collector for this application.
        .init();
    println!("Sleep");
    std::thread::spawn(|| {
        let mut i = 0;
        loop {
            i += 1;
            event!(Level::INFO, "{}", i);
            unsafe { Sleep(77) };
        }
    });
    let h = hook_Sleep(None).unwrap();
    unsafe { Sleep(10000) };
}
