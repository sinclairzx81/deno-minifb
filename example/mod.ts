
import * as Engine from './engine/mod.ts'
import { Window } from '../lib/mod.ts'
const window   = new Window({ width: 800, height: 600, x: 2000, topmost: true })
const renderer = new Engine.Renderer(800, 600)
const geometry = new Engine.BoxGeometry(10, 10, 10)
const material = new Engine.Material(null as any)
const mesh     = new Engine.Mesh(geometry, material)
const scene    = new Engine.Scene()

let i = 0

setInterval(() => { console.log('last', i); i = 0 }, 1000)

window.render(async () => {
    i++
    const buffer = await renderer.render()
    window.submit(buffer)
})
