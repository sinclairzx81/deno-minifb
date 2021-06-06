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

use super::*;

use minifb::{ MouseButton, WindowOptions, Window, Result };
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Size {
  width:  usize,
  height: usize,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Point {
  x: f32,
  y: f32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mouse {
  position: Point,
  wheel: Point,
  button_left: bool,
  button_middle: bool,
  button_right: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DenoWindowOptions {
  width:        usize, 
  height:       usize,
  x:            isize,
  y:            isize,
  title:        String,
  borderless:   bool,
  cursorless:   bool,
  resize:       bool,
  scale:        String,
  scale_mode:   String,
  topmost:      bool,
  transparency: bool
}

// -------------------------------------------------------------------------
// Deno Window
// -------------------------------------------------------------------------

pub struct Buffer {
  data: Vec<u8>,
  width:  usize,
  height: usize
}

pub struct DenoWindow {
  window: Window,
  buffer: Buffer
}

impl DenoWindow {
  pub fn new(options: DenoWindowOptions) -> Result<DenoWindow> {
    
    let mut window = Window::new(&options.title, options.width, options.height, WindowOptions {
        borderless: options.borderless,
        resize: options.resize,
        title: true,
        topmost: options.topmost,
        transparency: options.transparency,
        none: options.borderless,
        scale: enums::string_to_scale(options.scale),
        scale_mode: enums::string_to_scale_mode(options.scale_mode),
        ..WindowOptions::default()  
      },
    )?;
    window.set_cursor_visibility(!options.cursorless);
    window.set_position(options.x, options.y);
    let buffer = Buffer { width: 0, height: 0, data: vec![] };
    Ok(DenoWindow { window, buffer })
  }

  pub fn submit(&mut self, data: &[u8]) -> Result<()> {
    // Resize buffer if required. Note that minifb
    // size updates after the user drags, so we don't
    // expect high frequency allocations here.
    let (width, height) = self.window.get_size();
    if self.buffer.width != width || self.buffer.height != height {
      let data = vec![0; width * height * 4];
      self.buffer = Buffer { width, height, data };
    }
    // Copy data into buffer rgba -> bgra. We allow
    // buffers both smaller and larger than the windows
    // internal buffer. This is a bit more ergonimic
    // Deno side and encourages the user to keep their
    // buffer in sync with the window.
    for i in (0..self.buffer.data.len()).step_by(4) {
        if i >= data.len() - 4 { break };
        self.buffer.data[i+0] = data[i+2];
        self.buffer.data[i+1] = data[i+1];
        self.buffer.data[i+2] = data[i+0];
        self.buffer.data[i+3] = data[i+3];
    }
    // reinterpret to u32 slice and submit.
    let buffer: &[u32] = unsafe { std::mem::transmute(&self.buffer.data[..]) };
    self.window.update_with_buffer(buffer, width, height)
  }

  pub fn update(&mut self) {
    self.window.update()
  }

  pub fn is_open(&self) -> bool {
    self.window.is_open()
  }

  pub fn position(&mut self, x: isize, y: isize) {
    self.window.set_position(x, y);
  }

  pub fn topmost(&self, value: bool) {
    self.window.topmost(value);
  }

  pub fn background(&mut self, r: usize, g: usize, b: usize) {
    self.window.set_background_color(r, g, b);
  }

  pub fn cursor(&mut self, value: bool) {
    self.window.set_cursor_visibility(value);
  }

  pub fn size(&self) -> Size {
    let (width, height) = self.window.get_size();
    Size { width, height }
  }

  pub fn mouse(&self, mode: String) -> Mouse {
    let mode = enums::string_to_mouse_mode(&mode);
    let (position_x, position_y) = self.window.get_mouse_pos(mode).unwrap_or((0_f32, 0_f32));
    let (wheel_x, wheel_y) = self.window.get_scroll_wheel().unwrap_or((0_f32, 0_f32));
    let button_left = self.window.get_mouse_down(MouseButton::Left);
    let button_middle = self.window.get_mouse_down(MouseButton::Middle);
    let button_right = self.window.get_mouse_down(MouseButton::Right);
    Mouse {
      position: Point {
        x: position_x,
        y: position_y,
      },
      wheel: Point {
        x: wheel_x,
        y: wheel_y,
      },
      button_left,
      button_right,
      button_middle,
    }
  }

  pub fn keys(&self) -> Vec<String> {
    let keys = self.window.get_keys().unwrap_or(vec![]);
    keys.into_iter()
      .map(|key| enums::key_to_string(key))
      .collect::<Vec<String>>()
  }
}

// --------------------------------------------------------------
// Resources
// 
// Note: Had issues leveraging the Deno `OpState` when running
// with WebGPU handles. The following resource handling replicates
// similar functionality. This should be moved to `OpState` after
// some investigation.
// --------------------------------------------------------------

static mut RESOURCES: Option<HashMap<u32, DenoWindow>> = None;
static mut INDEX: u32 = 0;

impl DenoWindow {
  fn resolve_store<'a>() -> &'a mut HashMap<u32, DenoWindow> {
    unsafe {
      match RESOURCES {
        Some(ref mut store) => store,
        None => {
          RESOURCES = Some(HashMap::new());
          DenoWindow::resolve_store()
        }
      }
    }
  }
  pub fn set(window: DenoWindow) -> u32 {
    unsafe {
      let store = DenoWindow::resolve_store();
      let handle = INDEX;
      store.insert(handle, window);
      INDEX += 1;
      handle
    }
  }

  pub fn get<'a>(handle: u32) -> Option<&'a mut DenoWindow> {
    let store = DenoWindow::resolve_store();
    store.get_mut(&handle)
  }

  pub fn delete<'a>(handle: u32) {
    let store = DenoWindow::resolve_store();
    store.remove(&handle);
  }
}
