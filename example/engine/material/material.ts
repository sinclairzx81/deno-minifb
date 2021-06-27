import { Texture } from '../texture/mod.ts'
import { Shader } from './shader.ts'

export type Uniform = number | boolean | Texture

export class Material {
    public readonly uniforms: Map<string, Uniform>

    constructor(private readonly shader: Shader) {
        this.uniforms = new Map<string, Uniform>()
    }

    public encode(encoder: GPUCommandEncoder) {
        
    }
}