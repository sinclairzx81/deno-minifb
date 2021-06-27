import { device } from '../device.ts'

export class IndexAttribute {
    private readonly buffer: GPUBuffer

    constructor(private readonly data: Uint16Array) {
        this.buffer = device.createBuffer({ size: 4 * 2, usage: GPUBufferUsage.INDEX, mappedAtCreation: true });
        const writable = new Float32Array(this.buffer.getMappedRange())
        writable.set(data)
        this.buffer.unmap()
    }
    public update(): void {
        
    }
}

export class VertexAttribute {
    private readonly buffer: GPUBuffer
    
    constructor(data: Float32Array, components: number) {
        this.buffer = device.createBuffer({ size: 4 * components, usage: GPUBufferUsage.INDEX, mappedAtCreation: true });
        const writable = new Float32Array(this.buffer.getMappedRange())
        writable.set(data)
        this.buffer.unmap()
    }
}