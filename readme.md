<div align='center'>

<h1>Deno-MiniFB</h1>

<p>Render 32-bit RGBA Buffers to Desktop Windows</p>

</div>

```typescript
import { Window } from '../lib/mod.ts'

const buffer = new Uint8Array(800 * 600 * 4)

const window = new Window({ width: 800, height: 600 })

// ... loop

window.render(() => window.submit(buffer)) 
```

## Overview

Deno-MiniFB is a plugin for the Rust [MiniFB](https://github.com/emoon/rust_minifb) windowing library. It enables a Deno process to spawn windows and submit raw 32-bit RGBA pixel data to them. It also allows Deno to obtain user keyboard and mouse interactions with the window.

Deno-MiniFB was primarily written as a way to present graphics buffers produced by Deno's WebGPU implementation. It is built against Deno `deno 1.10.3` and tested on Windows and Ubuntu 20.x.

Licence MIT

## Build

You will need both Deno and the Rust toolchains installed locally. This project also uses an additional build tool to automate workflow. You can install it from `npm`.

```bash
$ npm install @sinclair/hammer -g
```
Once installed, you can start the `example` project with the following command.
```
$ hammer task start
```

### Linux

To build Deno-MiniFB on Ubuntu 20.x. Ensure you have the following dependencies.

```bash
$ sudo apt-get update
$ sudo apt-get install build-essential
$ sudo apt-get install libxkbcommon-x11-dev
```


