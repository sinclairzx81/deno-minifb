/*--------------------------------------------------------------------------

Deno-MiniFB

The MIT License (MIT)

Copyright (c) 2021 Haydn Paterson (sinclair) <haydn.developer@gmail.com>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

---------------------------------------------------------------------------*/



use minifb::{ Key, MouseButton, MouseMode, CursorStyle, Scale, ScaleMode, KeyRepeat };

// ----------------------------------------------------------
// KeyRepeat
// ----------------------------------------------------------
#[allow(dead_code)] pub fn string_to_key_repeat(n: String) -> KeyRepeat {
    match &*n {
        "Yes" => KeyRepeat::Yes,
        "No"  => KeyRepeat::No,
        _ => panic!("Invalid KeyRepeat"),
    }
}


#[allow(dead_code)] pub fn key_repeat_to_str(n: KeyRepeat) -> String {
    match n {
        KeyRepeat::Yes => "Yes",
        KeyRepeat::No  => "No",
    }.to_string()
}

// ----------------------------------------------------------
// MouseMode
// ----------------------------------------------------------


pub fn string_to_mouse_mode(n: &str) -> MouseMode {
    match &*n {
        "Pass"    => MouseMode::Pass,
        "Clamp"   => MouseMode::Clamp,
        "Discard" => MouseMode::Discard,
        _ => panic!("Invalid MouseMode"),
    }
}

#[allow(dead_code)] pub fn mouse_mode_to_string(n: MouseMode) -> String {
    match n {
        MouseMode::Pass    => "Pass",
        MouseMode::Clamp   => "Clamp",
        MouseMode::Discard => "Discard",
    }.to_string()
}

// ----------------------------------------------------------
// MouseButton
// ----------------------------------------------------------


#[allow(dead_code)] pub fn string_to_mouse_button(n: String) -> MouseButton {
    match &*n {
        "Left"   => MouseButton::Left,
        "Middle" => MouseButton::Middle,
        "Right"  => MouseButton::Right,
        _ => panic!("Invalid MouseButton"),
    }
}


#[allow(dead_code)] pub fn mouse_button_to_string(n: MouseButton) -> String {
    match n {
        MouseButton::Left   => "Left",
        MouseButton::Middle => "Middle",
        MouseButton::Right  => "Right",
    }.to_string()
}

// ----------------------------------------------------------
// CursorStyle
// ----------------------------------------------------------

#[allow(dead_code)] pub fn string_to_cursor_style(n: String) -> CursorStyle {
    match &*n {
        "Arrow"           => CursorStyle::Arrow,
        "Ibeam"           => CursorStyle::Ibeam,
        "Crosshair"       => CursorStyle::Crosshair,
        "ClosedHand"      => CursorStyle::ClosedHand,
        "OpenHand"        => CursorStyle::OpenHand,
        "ResizeLeftRight" => CursorStyle::ResizeLeftRight,
        "ResizeUpDown"    => CursorStyle::ResizeUpDown,
        "ResizeAll"       => CursorStyle::ResizeAll,
        _ => panic!("Invalid CursorStyle"),
    }
}

#[allow(dead_code)] pub fn cursor_style_to_string(n: CursorStyle) -> String  {
    match n {
        CursorStyle::Arrow           => "Arrow",
        CursorStyle::Ibeam           => "Ibeam",
        CursorStyle::Crosshair       => "Crosshair",
        CursorStyle::ClosedHand      => "ClosedHand",
        CursorStyle::OpenHand        => "OpenHand",
        CursorStyle::ResizeLeftRight => "ResizeLeftRight",
        CursorStyle::ResizeUpDown    => "ResizeUpDown",
        CursorStyle::ResizeAll       => "ResizeAll",
    }.to_string()
}

// ----------------------------------------------------------
// Scale
// ----------------------------------------------------------

#[allow(dead_code)] pub fn string_to_scale(n: String) -> Scale {
    match &*n {
        "FitScreen" => Scale::FitScreen,
        "X1"        => Scale::X1,
        "X2"        => Scale::X2,
        "X4"        => Scale::X4,
        "X8"        => Scale::X8,
        "X16"       => Scale::X16,
        "X32"       => Scale::X32,
        _ => panic!("Invalid Scale"),
    }
}

#[allow(dead_code)] pub fn scale_to_string(n: Scale) -> String {
    match n {
        Scale::FitScreen => "FitScreen",
        Scale::X1        => "X1",
        Scale::X2        => "X2",
        Scale::X4        => "X4",
        Scale::X8        => "X8",
        Scale::X16       => "X16",
        Scale::X32       => "X32"
    }.to_string()
}

// ----------------------------------------------------------
// ScaleMode
// ----------------------------------------------------------

#[allow(dead_code)] pub fn string_to_scale_mode(n: String) -> ScaleMode {
    match &*n {
        "Stretch"            => ScaleMode::Stretch,
        "AspectRatioStretch" => ScaleMode::AspectRatioStretch,
        "Center"             => ScaleMode::Center,
        "UpperLeft"          => ScaleMode::UpperLeft,
        _ => panic!("Invalid ScaleMode"),
    }
}

#[allow(dead_code)] pub fn scale_mode_to_string(n: ScaleMode) -> String {
    match n {
        ScaleMode::Stretch            => "Stretch",
        ScaleMode::AspectRatioStretch => "AspectRatioStretch",
        ScaleMode::Center             => "Center",
        ScaleMode::UpperLeft          => "UpperLeft"
    }.to_string()
}


// -----------------------------------------------------
// Key
// -----------------------------------------------------

#[allow(dead_code)] pub fn key_to_string(n: Key) -> String {
    match n {
        Key::Key0           => "Key0",
        Key::Key1           => "Key1",
        Key::Key2           => "Key2",
        Key::Key3           => "Key3",
        Key::Key4           => "Key4",
        Key::Key5           => "Key5",
        Key::Key6           => "Key6",
        Key::Key7           => "Key7",
        Key::Key8           => "Key8",
        Key::Key9           => "Key9",
        Key::A              => "A",
        Key::B              => "B",
        Key::C              => "C",
        Key::D              => "D",
        Key::E              => "E",
        Key::F              => "F",
        Key::G              => "G",
        Key::H              => "H",
        Key::I              => "I",
        Key::J              => "J",
        Key::K              => "K",
        Key::L              => "L",
        Key::M              => "M",
        Key::N              => "N",
        Key::O              => "O",
        Key::P              => "P",
        Key::Q              => "Q",
        Key::R              => "R",
        Key::S              => "S",
        Key::T              => "T",
        Key::U              => "U",
        Key::V              => "V",
        Key::W              => "W",
        Key::X              => "X",
        Key::Y              => "Y",
        Key::Z              => "Z",
        Key::F1             => "F1",
        Key::F2             => "F2",
        Key::F3             => "F3",
        Key::F4             => "F4",
        Key::F5             => "F5",
        Key::F6             => "F6",
        Key::F7             => "F7",
        Key::F8             => "F8",
        Key::F9             => "F9",
        Key::F10            => "F10",
        Key::F11            => "F11",
        Key::F12            => "F12",
        Key::F13            => "F13",
        Key::F14            => "F14",
        Key::F15            => "F15",
        Key::Down           => "Down",
        Key::Left           => "Left",
        Key::Right          => "Right",
        Key::Up             => "Up",
        Key::Apostrophe     => "Apostrophe",
        Key::Backquote      => "Backquote",
        Key::Backslash      => "Backslash",
        Key::Comma          => "Comma",
        Key::Equal          => "Equal",
        Key::LeftBracket    => "LeftBracket",
        Key::Minus          => "Minus",
        Key::Period         => "Period",
        Key::RightBracket   => "RightBracket",
        Key::Semicolon      => "Semicolon",
        Key::Slash          => "Slash",
        Key::Backspace      => "Backspace",
        Key::Delete         => "Delete",
        Key::End            => "End",
        Key::Enter          => "Enter",
        Key::Escape         => "Escape",
        Key::Home           => "Home",
        Key::Insert         => "Insert",
        Key::Menu           => "Menu",
        Key::PageDown       => "PageDown",
        Key::PageUp         => "PageUp",
        Key::Pause          => "Pause",
        Key::Space          => "Space",
        Key::Tab            => "Tab",
        Key::NumLock        => "NumLock",
        Key::CapsLock       => "CapsLock",
        Key::ScrollLock     => "ScrollLock",
        Key::LeftShift      => "LeftShift",
        Key::RightShift     => "RightShift",
        Key::LeftCtrl       => "LeftCtrl",
        Key::RightCtrl      => "RightCtrl",
        Key::NumPad0        => "NumPad0",
        Key::NumPad1        => "NumPad1",
        Key::NumPad2        => "NumPad2",
        Key::NumPad3        => "NumPad3",
        Key::NumPad4        => "NumPad4",
        Key::NumPad5        => "NumPad5",
        Key::NumPad6        => "NumPad6",
        Key::NumPad7        => "NumPad7",
        Key::NumPad8        => "NumPad8",
        Key::NumPad9        => "NumPad9",
        Key::NumPadDot      => "NumPadDot",
        Key::NumPadSlash    => "NumPadSlash",
        Key::NumPadAsterisk => "NumPadAsterisk",
        Key::NumPadMinus    => "NumPadMinus",
        Key::NumPadPlus     => "NumPadPlus",
        Key::NumPadEnter    => "NumPadEnter",
        Key::LeftAlt        => "LeftAlt",
        Key::RightAlt       => "RightAlt",
        Key::LeftSuper      => "LeftSuper",
        Key::RightSuper     => "RightSuper",
        Key::Unknown        => "Unknown",
        Key::Count          => "Count",
    }.to_string()
}

#[allow(dead_code)] pub fn string_to_key(n: String) -> Key {
    match &*n {
        "Key0"           => Key::Key0,
        "Key1"           => Key::Key1,
        "Key2"           => Key::Key2,
        "Key3"           => Key::Key3,
        "Key4"           => Key::Key4,
        "Key5"           => Key::Key5,
        "Key6"           => Key::Key6,
        "Key7"           => Key::Key7,
        "Key8"           => Key::Key8,
        "Key9"           => Key::Key9,
        "A"              => Key::A,
        "B"              => Key::B,
        "C"              => Key::C,
        "D"              => Key::D,
        "E"              => Key::E,
        "F"              => Key::F,
        "G"              => Key::G,
        "H"              => Key::H,
        "I"              => Key::I,
        "J"              => Key::J,
        "K"              => Key::K,
        "L"              => Key::L,
        "M"              => Key::M,
        "N"              => Key::N,
        "O"              => Key::O,
        "P"              => Key::P,
        "Q"              => Key::Q,
        "R"              => Key::R,
        "S"              => Key::S,
        "T"              => Key::T,
        "U"              => Key::U,
        "V"              => Key::V,
        "W"              => Key::W,
        "X"              => Key::X,
        "Y"              => Key::Y,
        "Z"              => Key::Z,
        "F1"             => Key::F1,
        "F2"             => Key::F2,
        "F3"             => Key::F3,
        "F4"             => Key::F4,
        "F5"             => Key::F5,
        "F6"             => Key::F6,
        "F7"             => Key::F7,
        "F8"             => Key::F8,
        "F9"             => Key::F9,
        "F10"            => Key::F10,
        "F11"            => Key::F11,
        "F12"            => Key::F12,
        "F13"            => Key::F13,
        "F14"            => Key::F14,
        "F15"            => Key::F15,
        "Down"           => Key::Down,
        "Left"           => Key::Left,
        "Right"          => Key::Right,
        "Up"             => Key::Up,
        "Apostrophe"     => Key::Apostrophe,
        "Backquote"      => Key::Backquote,
        "Backslash"      => Key::Backslash,
        "Comma"          => Key::Comma,
        "Equal"          => Key::Equal,
        "LeftBracket"    => Key::LeftBracket,
        "Minus"          => Key::Minus,
        "Period"         => Key::Period,
        "RightBracket"   => Key::RightBracket,
        "Semicolon"      => Key::Semicolon,
        "Slash"          => Key::Slash,
        "Backspace"      => Key::Backspace,
        "Delete"         => Key::Delete,
        "End"            => Key::End,
        "Enter"          => Key::Enter,
        "Escape"         => Key::Escape,
        "Home"           => Key::Home,
        "Insert"         => Key::Insert,
        "Menu"           => Key::Menu,
        "PageDown"       => Key::PageDown,
        "PageUp"         => Key::PageUp,
        "Pause"          => Key::Pause,
        "Space"          => Key::Space,
        "Tab"            => Key::Tab,
        "NumLock"        => Key::NumLock,
        "CapsLock"       => Key::CapsLock,
        "ScrollLock"     => Key::ScrollLock,
        "LeftShift"      => Key::LeftShift,
        "RightShift"     => Key::RightShift,
        "LeftCtrl"       => Key::LeftCtrl,
        "RightCtrl"      => Key::RightCtrl,
        "NumPad0"        => Key::NumPad0,
        "NumPad1"        => Key::NumPad1,
        "NumPad2"        => Key::NumPad2,
        "NumPad3"        => Key::NumPad3,
        "NumPad4"        => Key::NumPad4,
        "NumPad5"        => Key::NumPad5,
        "NumPad6"        => Key::NumPad6,
        "NumPad7"        => Key::NumPad7,
        "NumPad8"        => Key::NumPad8,
        "NumPad9"        => Key::NumPad9,
        "NumPadDot"      => Key::NumPadDot,
        "NumPadSlash"    => Key::NumPadSlash,
        "NumPadAsterisk" => Key::NumPadAsterisk,
        "NumPadMinus"    => Key::NumPadMinus,
        "NumPadPlus"     => Key::NumPadPlus,
        "NumPadEnter"    => Key::NumPadEnter,
        "LeftAlt"        => Key::LeftAlt,
        "RightAlt"       => Key::RightAlt,
        "LeftSuper"      => Key::LeftSuper,
        "RightSuper"     => Key::RightSuper,
        "Unknown"        => Key::Unknown,
        "Count"          => Key::Count,
        _ => panic!("Invalid Key")
    }
}