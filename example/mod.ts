import * as Engine from './engine/mod.ts'
import { Window } from '../lib/mod.ts'

const window   = new Window({ width: 800, height: 600, x: 2000, topmost: true })
const renderer = new Engine.Renderer(800, 600)
const camera   = new Engine.Camera(90, 800 / 600, 0.1, 1000.0)
const geometry = new Engine.BoxGeometry(10, 10, 10)
const material = new Engine.Material(null as any)
const mesh     = new Engine.Mesh(geometry, material)
const light    = new Engine.Light()
const scene    = new Engine.Scene()

scene.objects.push(mesh)
scene.objects.push(light)

window.render(async () => {
    const buffer = await renderer.render(camera, scene)
    window.submit(buffer)
})
