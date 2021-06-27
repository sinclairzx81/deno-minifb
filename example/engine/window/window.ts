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

import * as interop from './native/interop.js'

// --------------------------------------------------------------------------
// Support Types
// --------------------------------------------------------------------------

export type KeyRepeat = 'Yes' | 'No' 

export type MouseMode = 'Pass' | 'Clamp' | 'Discard'

export type Scale = 'FitScreen' | 'X1' | 'X2' | 'X4' | 'X8' | 'X16' | 'X32'

export type ScaleMode = 'Stretch' | 'AspectRatio' | 'Center' | 'UpperLeft'

export type Point = { x: number, y: number }

export type Size = { width: number, height: number }

export type Key =
    | 'Key0' | 'Key1' | 'Key2' | 'Key3' | 'Key4' | 'Key5' | 'Key6' | 'Key7' | 'Key8' | 'Key9'
    | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O'
    | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z' | 'F1' | 'F2' | 'F3' 
    | 'F4' | 'F5' | 'F6' | 'F7' | 'F8' | 'F9' | 'F10' | 'F11' | 'F12' | 'F13' | 'F14' | 'F15'
    | 'Down' | 'Left' | 'Right' | 'Up' | 'Apostrophe' | 'Backquote' | 'Backslash' | 'Comma' 
    | 'Equal' | 'LeftBracket' | 'Minus' | 'Period' | 'RightBracket' | 'Semicolon' | 'Slash' 
    | 'Backspace' | 'Delete' | 'End' | 'Enter' | 'Escape' | 'Home' | 'Insert' | 'Menu' 
    | 'PageDown' | 'PageUp' | 'Pause' | 'Space' | 'Tab' | 'NumLock' | 'CapsLock' | 'ScrollLock' 
    | 'LeftShift' | 'RightShift' | 'LeftCtrl' | 'RightCtrl' | 'NumPad0' | 'NumPad1' | 'NumPad2' 
    | 'NumPad3' | 'NumPad4' | 'NumPad5' | 'NumPad6' | 'NumPad7' | 'NumPad8' | 'NumPad9' 
    | 'NumPadDot' | 'NumPadSlash' | 'NumPadAsterisk' | 'NumPadMinus' | 'NumPadPlus' | 'NumPadEnter' 
    | 'LeftAlt' | 'RightAlt' | 'LeftSuper' | 'RightSuper' | 'Unknown' | 'Count'

export interface Mouse {
    position: Point,
    wheel: Point
    button_left: boolean,
    button_middle: boolean,
    button_right: boolean,
}

// --------------------------------------------------------------------------
// Support Types
// --------------------------------------------------------------------------

export interface WindowOptions {
    /** The initial width of the window. (default: 640) */
    width:        number, 
    /** The initial height of the window. (default: 480) */
    height:       number, 
    /** The initial x position of the window. (default: 128) */
    x:            number,
    /** The initial y position of the window. (default: 128) */
    y:            number,
    /** The title of the window. */
    title:        string,
    /** If true, will remove the border from the window. (default: false) */
    borderless:   boolean,
    /** If true, will hide the mouse cursor. (default: false) */
    cursorless:   boolean,
    /** If true, will allow this window to be resized. (default: false) */
    resize:       boolean,
    /** The scale of the window. (default: 'X1') */
    scale:        Scale,
    /** The scale mode of the window. (default: 'UpperLeft') */
    scale_mode:   ScaleMode,
    /** If true, the window will always be on top. (default: false) */
    topmost:      boolean,
    /** If true, enables transparency (default: false) */
    transparency: boolean
}


export type WindowRenderFunction = () => any

export class Window {
    private readonly handle: { rid: number }
    private onRender?: WindowRenderFunction
    
    /** Creates a new window. */
    constructor(options: Partial<WindowOptions> = {}) {
        this.handle = interop.window_create({ options: { 
            width: 640, 
            height: 460,
            x: 128,
            y: 128,
            title: 'Deno-MiniFB',
            borderless: false,
            resize: false,
            scale: 'X1',
            scale_mode: 'UpperLeft',
            topmost: false,
            transparency: false,
            ...options
        }})
    }

    /** Returns the width of this window. */
    public get width(): number {
        const { value } = interop.window_size({...this.handle })
        const { width } = value
        return width
    }

    /** Returns the height of this window. */
    public get height(): number {
        const { value } = interop.window_size({...this.handle })
        const { height } = value
        return height
    }

    /** Returns true if this window is still open. */
    public isOpen(): boolean {
        const { value } = interop.window_is_open({...this.handle })
        return value
     }

    /** Updates this window. Required for keyboard and mouse events. */
    public update() {
        interop.window_update({...this.handle })
    }

    /** Closes this window. */
    public close(): void {
        interop.window_close({...this.handle })
    }

 
    /** Sets the position of this window. */
    public position(x: number, y: number): void {
        interop.window_position({...this.handle, x, y })
    }

    /** Sets if the cursor should be visible. False to hide. */
    public cursor(value: boolean): void {
        interop.window_cursor({...this.handle, value })
    }

    /** Submits a gpuBuffer to this window. */
    public submit(buffer: Uint8Array): void {
        interop.window_submit({...this.handle }, buffer)
    }

    /** Returns the mouse and button state. */
    public mouse(mode: MouseMode = 'Clamp'): Mouse {
        const { value } = interop.window_mouse({...this.handle, mode })
        return value
    }

    /** Returns the size of this window. */
    public size(): Size {
        const { value } = interop.window_size({...this.handle })
        return value
    }

    /** Returns an array of pressed keys. */
    public keys(): Key[] {
        const { value } = interop.window_keys({...this.handle })
        return value
    }

    /** 
     * Creates a render loop for this window. This function will automatically
     * update the window and fire the callback at the specified rate. This 
     * function also keeps the deno process running until the window is closed.
     */
    public render(func: WindowRenderFunction, rate: number = 16) {
        if(this.onRender !== undefined) throw Error('The render function can only be called once.');
        this.onRender = func
        const loop = async () => {
            this.update()
            await this.onRender!()
            if(!this.isOpen()) return
            setTimeout(() => loop(), rate)
        }
        loop();
    }
}
