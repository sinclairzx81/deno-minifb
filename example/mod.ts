import { Window } from '../lib/mod.ts'

function noise(buffer: Uint8Array) {
    for(let i = 0; i < buffer.length; i+=4) {
        const random = Math.floor(Math.random() * 256) 
        buffer[i+0] = random
        buffer[i+1] = random 
        buffer[i+2] = random
        buffer[i+3] = random
    }
}

const buffer = new Uint8Array(800 * 600 * 4)

const window = new Window({ width: 800, height: 600, topmost: true })

window.render(() => {

    noise(buffer)

    window.submit(buffer)
})
