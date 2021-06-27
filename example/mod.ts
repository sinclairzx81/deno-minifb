
import * as Acid  from './engine/mod.ts'
import { Window } from '../lib/mod.ts'
const window   = new Window({ width: 800, height: 600, x: 2000, topmost: true })
const renderer = new Acid.Renderer(800, 600)
const geometry = new Acid.BoxGeometry(10, 10, 10)
const material = new Acid.Material(null as any)
const mesh     = new Acid.Mesh(geometry, material)
const scene    = new Acid.Scene()

let i = 0

setInterval(() => { console.log('last', i); i = 0 }, 1000)

window.render(async () => {
    i++
    const buffer = await renderer.render()
    window.submit(buffer)
})
