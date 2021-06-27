
import * as Delta from './engine/mod.ts'
import { Window } from '../lib/mod.ts'
const window   = new Window({ width: 800, height: 600, x: 2000, topmost: true })
const renderer = new Delta.Renderer(800, 600)
const geometry = new Delta.BoxGeometry(10, 10, 10)
const material = new Delta.Material(null as any)
const mesh     = new Delta.Mesh(geometry, material)
const scene    = new Delta.Scene()

let i = 0

setInterval(() => { console.log('last', i); i = 0 }, 1000)

window.render(async () => {
    i++
    const buffer = await renderer.render()
    window.submit(buffer)
})
