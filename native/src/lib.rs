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

use serde::{ Serialize, Deserialize };
use window::{ DenoWindow, DenoWindowOptions, Mouse, Size };

use deno_core::op_sync;
use deno_core::error::AnyError;
use deno_core::Extension;
use deno_core::OpState;
use deno_core::ResourceId;
use deno_core::ZeroCopyBuf;

mod window;

// -----------------------------------------------------------------
// Contract
// -----------------------------------------------------------------

#[derive(Debug, Deserialize)] struct WindowCreateRequest {
  options: DenoWindowOptions
}

#[derive(Debug, Serialize)] struct WindowCreateResponse {
  rid: ResourceId
}

#[derive(Debug, Deserialize)] struct WindowUpdateRequest {
  rid: ResourceId,
}

#[derive(Debug, Serialize)] struct WindowUpdateResponse {
  rid: ResourceId
}

#[derive(Debug, Deserialize)] struct WindowSubmitRequest {
  rid: ResourceId,
}

#[derive(Debug, Serialize)] struct WindowSubmitResponse {
  rid: ResourceId
}


#[derive(Debug, Deserialize)] struct WindowCloseRequest {
  rid: ResourceId
}

#[derive(Debug, Serialize)] struct WindowCloseResponse {
  rid: ResourceId
}

#[derive(Debug, Deserialize)] struct WindowOpenRequest {
  rid: ResourceId
}

#[derive(Debug, Serialize)] struct WindowOpenResponse {
  rid: ResourceId,
  value: bool
}

#[derive(Debug, Deserialize)] struct WindowTopmostRequest {
  rid: ResourceId,
  value: bool
}

#[derive(Debug, Serialize)] struct WindowTopmostResponse {
  rid: ResourceId,
}

#[derive(Debug, Deserialize)] struct WindowPositionRequest {
  rid: ResourceId,
  x: isize,
  y: isize
}

#[derive(Debug, Serialize)] struct WindowPositionResponse {
  rid: ResourceId,
}

#[derive(Debug, Deserialize)] struct WindowBackgroundRequest {
  rid: ResourceId,
  r: usize,
  g: usize,
  b: usize
}

#[derive(Debug, Serialize)] struct WindowBackgroundResponse {
  rid: ResourceId,
}

#[derive(Debug, Deserialize)] struct WindowCursorRequest {
  rid: ResourceId,
  value: bool
}

#[derive(Debug, Serialize)] struct WindowCursorResponse {
  rid: ResourceId,
}

#[derive(Debug, Deserialize)] struct WindowMouseRequest {
  rid: ResourceId,
  mode: String
}

#[derive(Debug, Serialize)] struct WindowMouseResponse {
  rid: ResourceId,
  value: Mouse
}

#[derive(Debug, Deserialize)] struct WindowSizeRequest {
  rid: ResourceId
}

#[derive(Debug, Serialize)] struct WindowSizeResponse {
  rid: ResourceId,
  value: Size
}

#[derive(Debug, Deserialize)] struct WindowKeysRequest {
  rid: ResourceId
}

#[derive(Debug, Serialize)] struct WindowKeysResponse {
  rid: ResourceId,
  value: Vec<String>
}

// -----------------------------------------------------------------
// Operations
// -----------------------------------------------------------------

fn op_minifb_window_create(_state: &mut OpState, request: WindowCreateRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowCreateResponse, AnyError> {
  let window = DenoWindow::new(request.options)?;
  let rid = DenoWindow::set(window);
  let response = WindowCreateResponse { rid };
  Ok(response)
}

fn op_minifb_window_update(_state: &mut OpState, request: WindowUpdateRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowUpdateResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  window.update();
  Ok(WindowUpdateResponse { rid: request.rid })
}

fn op_minifb_window_submit(_state: &mut OpState, request: WindowSubmitRequest, zero_copy: Option<ZeroCopyBuf>) -> Result<WindowSubmitResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  if let Some(mut buffer) = zero_copy { window.submit(&mut buffer[..])?; }
  Ok(WindowSubmitResponse { rid: request.rid })
}

fn op_minifb_window_close(_state: &mut OpState, request: WindowCloseRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowCloseResponse, AnyError> {
  DenoWindow::delete(request.rid);
  Ok(WindowCloseResponse { rid: request.rid })
}

fn op_minifb_window_is_open(_state: &mut OpState, request: WindowOpenRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowOpenResponse, AnyError> {
  if let Some(window) = DenoWindow::get(request.rid) {
    let value = window.is_open();
    Ok(WindowOpenResponse { rid: request.rid, value })
  } else {
    Ok(WindowOpenResponse { rid: request.rid, value: false })
  }
}

fn op_minifb_window_topmost(_state: &mut OpState, request: WindowTopmostRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowTopmostResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  window.topmost(request.value);
  Ok(WindowTopmostResponse { rid: request.rid })
}

fn op_minifb_window_position(_state: &mut OpState, request: WindowPositionRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowPositionResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  window.position(request.x, request.y);
  Ok(WindowPositionResponse { rid: request.rid })
}


fn op_minifb_window_background(_state: &mut OpState, request: WindowBackgroundRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowBackgroundResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  window.background(request.r, request.g, request.b);
  Ok(WindowBackgroundResponse { rid: request.rid })
}

fn op_minifb_window_cursor(_state: &mut OpState, request: WindowCursorRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowCursorResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  window.cursor(request.value);
  Ok(WindowCursorResponse { rid: request.rid })
}

fn op_minifb_window_mouse(_state: &mut OpState, request: WindowMouseRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowMouseResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  let value = window.mouse(request.mode);
  Ok(WindowMouseResponse { rid: request.rid, value })
}

fn op_minifb_window_size(_state: &mut OpState, request: WindowSizeRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowSizeResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  let value = window.size();
  Ok(WindowSizeResponse { rid: request.rid, value })
}

fn op_minifb_window_keys(_state: &mut OpState, request: WindowKeysRequest, _zero_copy: Option<ZeroCopyBuf>) -> Result<WindowKeysResponse, AnyError> {
  let window = DenoWindow::get(request.rid).unwrap();
  let value = window.keys();
  Ok(WindowKeysResponse { rid: request.rid, value })
}

// -----------------------------------------------------------------------
// Interface
// -----------------------------------------------------------------------

#[no_mangle] pub fn init() -> Extension {
  Extension::builder()
    .ops(vec![
      ("op_minifb_window_create", op_sync(op_minifb_window_create)),
      ("op_minifb_window_is_open", op_sync(op_minifb_window_is_open)),
      ("op_minifb_window_update", op_sync(op_minifb_window_update)),
      ("op_minifb_window_close", op_sync(op_minifb_window_close)),
      ("op_minifb_window_submit", op_sync(op_minifb_window_submit)),
      ("op_minifb_window_position", op_sync(op_minifb_window_position)),
      ("op_minifb_window_topmost", op_sync(op_minifb_window_topmost)),
      ("op_minifb_window_background", op_sync(op_minifb_window_background)),
      ("op_minifb_window_cursor", op_sync(op_minifb_window_cursor)),
      ("op_minifb_window_mouse", op_sync(op_minifb_window_mouse)),
      ("op_minifb_window_size", op_sync(op_minifb_window_size)),
      ("op_minifb_window_keys", op_sync(op_minifb_window_keys)),
    ])
    .build()
}
