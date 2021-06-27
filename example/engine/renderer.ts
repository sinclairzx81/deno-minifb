import { device }        from './device.ts'
import { getRowPadding } from './padding.ts'

export class Renderer {
    private readonly pipelineLayout: GPUPipelineLayout
    private readonly renderPipeline: GPURenderPipeline
    private readonly shaderModule:   GPUShaderModule
    private readonly gpuBuffer:      GPUBuffer
    private readonly gpuTexture:     GPUTexture
    private readonly outputBuffer:   Uint8Array

    constructor(private readonly width: number, private readonly height: number) {
        
        const { padded, unpadded } = getRowPadding(this.width)
        const size = unpadded * this.height
        this.outputBuffer = new Uint8Array(size)
        this.gpuBuffer    = device.createBuffer({
            label: 'RenderTarget',
            size: padded * this.height,
            usage: GPUBufferUsage.MAP_READ | GPUBufferUsage.COPY_DST
        })
        this.gpuTexture  = device.createTexture({
            label: 'RenderTarget',
            size: { width: this.width, height: this.height },
            format: "rgba8unorm-srgb",
            usage: GPUTextureUsage.RENDER_ATTACHMENT | GPUTextureUsage.COPY_SRC,
        })
        this.shaderModule = device.createShaderModule({
            code: (`
            [[stage(vertex)]]
            fn vs_main([[builtin(vertex_index)]] in_vertex_index: u32) -> [[builtin(position)]] vec4<f32> {
                let x = f32(i32(in_vertex_index) - 1);
                let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
                return vec4<f32>(x, y, 0.0, 1.0);
            }
            [[stage(fragment)]]
            fn fs_main() -> [[location(0)]] vec4<f32> {
                return vec4<f32>(0.3, 0.3, 0.3, 1.0);
            }`),
        })
        this.pipelineLayout = device.createPipelineLayout({
            bindGroupLayouts: [],
        })
        this.renderPipeline = device.createRenderPipeline({
            layout: this.pipelineLayout,
            vertex: {
                module: this.shaderModule,
                entryPoint: "vs_main",
            },
            fragment: {
                module: this.shaderModule,
                entryPoint: "fs_main",
                targets: [
                    {
                        format: "rgba8unorm-srgb",
                    },
                ],
            }
        })
    }

    private copyTextureToBuffer(encoder: GPUCommandEncoder): void {
        const { padded } = getRowPadding(this.width)
        encoder.copyTextureToBuffer({ 
            texture:      this.gpuTexture
        }, {
            buffer:       this.gpuBuffer,
            bytesPerRow:  padded,
            rowsPerImage: 0,
        }, {
            width:  this.width,
            height: this.height
        })
    }

    private async copyBufferToOutput() {
        await this.gpuBuffer.mapAsync(1)
        const inputBuffer = new Uint8Array(this.gpuBuffer.getMappedRange())
        const { padded, unpadded } = getRowPadding(this.width)
        for (let i = 0; i < this.height; i++) {
            const slice = inputBuffer.subarray(i * padded, (i + 1) * padded).subarray(0, unpadded)
            this.outputBuffer.set(slice, i * unpadded)
        }
        this.gpuBuffer.unmap()
    }

    public async render() {
        const encoder = device.createCommandEncoder()
        const colorAttachment: GPURenderPassColorAttachment = {
            view:      this.gpuTexture.createView(),
            storeOp:   "store",
            loadValue: [0.1, 0.1, 0.1, 1]
        }
        const renderPass = encoder.beginRenderPass({
            colorAttachments: [colorAttachment],
        })
        renderPass.setPipeline(this.renderPipeline)
        renderPass.draw(3, 1)
        renderPass.endPass()

        this.copyTextureToBuffer(encoder)
        await this.copyBufferToOutput()
        device.queue.submit([ encoder.finish() ])
        return this.outputBuffer
    }
}