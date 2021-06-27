import { VertexAttribute, IndexAttribute } from './attributes.ts'
import { Geometry } from './geometry.ts'

export class BoxGeometry extends Geometry {
    constructor(
        private readonly width: number,
        private readonly height: number,
        private readonly depth: number
    ) {
        const indices   = new Uint16Array(1)
        const positions = new Float32Array(3)
        const normals   = new Float32Array(3)
        const uvs       = new Float32Array(2)
        super([
            new VertexAttribute(positions, 4),
            new VertexAttribute(normals, 3),
            new VertexAttribute(uvs, 2)
        ], new IndexAttribute(new Uint16Array([])))
    }
}