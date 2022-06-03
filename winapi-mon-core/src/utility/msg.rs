use winapi::{shared::windef::POINT, um::winuser::MSG};

pub fn MSG_to_string(msg: MSG) -> String {
    format!(
        "[MSG hwnd {}, message {}, wParam {}, lParam {}, time {}, pt {}]",
        msg.hwnd as usize,
        msg.message,
        msg.wParam,
        msg.lParam,
        msg.time,
        format!("x {}, y {}", msg.pt.x, msg.pt.y)
    )
}
