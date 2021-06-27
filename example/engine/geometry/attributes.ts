import { device } from '../device.ts'

export class IndexAttribute {
    private readonly buffer: GPUBuffer

    constructor(private readonly data: Uint16Array) {
        const size = (Uint16Array.BYTES_PER_ELEMENT) * this.data.length
        const usage = GPUBufferUsage.INDEX
        this.buffer = device.createBuffer({ size, usage, mappedAtCreation: true });
        const mapped = new Uint16Array(this.buffer.getMappedRange())
        mapped.set(this.data)
        this.buffer.unmap()
    }
}

export class VertexAttribute {
    private readonly buffer: GPUBuffer
    constructor(private readonly data: Float32Array, public readonly components: number) {
        const size = (Float32Array.BYTES_PER_ELEMENT * this.components) * data.length
        const usage = GPUBufferUsage.VERTEX
        this.buffer = device.createBuffer({ label: 'VertexBuffer', size, usage, mappedAtCreation: true })
        const mapped = new Float32Array(this.buffer.getMappedRange())
        mapped.set(this.data)
        this.buffer.unmap()
    }
}