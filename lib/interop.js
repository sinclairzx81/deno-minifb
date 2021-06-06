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

// ----------------------------------------------------------------------
// Entry
// ----------------------------------------------------------------------

const entry = Deno.build.os === 'linux' 
 ? new URL(import.meta.url.replace('interop.js', 'native/libnative.so')).pathname
 : new URL(import.meta.url.replace('interop.js', 'native/native.dll')).pathname.slice(1)


Deno.openPlugin(entry)

// ----------------------------------------------------------------------
// Interop
// ----------------------------------------------------------------------

export function window_create(options) {
    return Deno.core.opSync('op_minifb_window_create', options)
}

export function window_update(options) {
    return Deno.core.opSync('op_minifb_window_update', options)
}

export function window_submit(options, buffer) {
    return Deno.core.opSync('op_minifb_window_submit', options, buffer)
}

export function window_close(options) {
    return Deno.core.opSync('op_minifb_window_close', options)
}

export function window_is_open(options) {
    return Deno.core.opSync('op_minifb_window_is_open', options)
}

export function window_topmost(options) {
    return Deno.core.opSync('op_minifb_window_topmost', options)
}

export function window_position(options) {
    return Deno.core.opSync('op_minifb_window_position', options)
}

export function window_background(options) {
    return Deno.core.opSync('op_minifb_window_background', options)
}

export function window_cursor(options) {
    return Deno.core.opSync('op_minifb_window_cursor', options)
}

export function window_mouse(options) {
    return Deno.core.opSync('op_minifb_window_mouse', options)
}

export function window_size(options) {
    return Deno.core.opSync('op_minifb_window_size', options)
}

export function window_keys(options) {
    return Deno.core.opSync('op_minifb_window_keys', options)
}
