import { Texture } from '../texture/mod.ts'
import { Shader }  from './shader.ts'
import { device }  from '../device.ts'

export type Uniform = number | boolean | Texture

export class Material {
    public readonly pipelineLayout: GPUPipelineLayout
    public readonly uniforms: Map<string, Uniform>

    constructor(private readonly shader: Shader) {
        this.uniforms = new Map<string, Uniform>()
        this.pipelineLayout = device.createPipelineLayout({
            bindGroupLayouts: [],
        })
    }

    public encode(encoder: GPUCommandEncoder) {
        
    }
}