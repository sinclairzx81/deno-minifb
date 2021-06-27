import { device } from '../device.ts'

export class Shader {
    public readonly shader: GPUShaderModule
    constructor(private readonly shaderCode: string) {
        this.shader = device.createShaderModule({ code: shaderCode })
    }
}