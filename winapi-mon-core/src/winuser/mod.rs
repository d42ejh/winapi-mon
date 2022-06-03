mod dispatch_message;
mod peek_message;
pub use dispatch_message::{
    hook_DispatchMessageA, hook_DispatchMessageW, DispatchMessageADetour, DispatchMessageWDetour,
};
pub use peek_message::{
    hook_PeekMessageA, hook_PeekMessageW, PeekMessageADetour, PeekMessageWDetour,
};
