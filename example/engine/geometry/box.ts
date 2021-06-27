import { Vector3, Vector4, Vector2 } from '../math/mod.ts'
import { VertexAttribute, IndexAttribute } from './attributes.ts'
import { Geometry } from './geometry.ts'

interface Vertex {
    position: Vector4,
    normal: Vector3,
    uv: Vector2
}

export class BoxGeometry extends Geometry {
    constructor(
        private readonly width: number,
        private readonly height: number,
        private readonly depth: number
    ) {
        const w = width / 2
        const h = height / 2
        const d = depth / 2
        const positions = new Float32Array([
            // top (0, 0, 1)
            ...[-w, -h, d, 1],
            ...[w, -h, d, 1],
            ...[w, h, d, 1],
            ...[-w, h, d, 1],
            // bottom (0, 0, -1)
            ...[-w, h, -d, 1],
            ...[w, h, -d, 1],
            ...[w, -h, -d, 1],
            ...[-w, -h, -d, 1],
            // right (1, 0, 0)
            ...[w, -h, -d, 1],
            ...[w, h, -d, 1],
            ...[w, h, d, 1],
            ...[w, -h, d, 1],
            // left (-1, 0, 0)
            ...[-w, -h, d, 1],
            ...[-w, h, d, 1],
            ...[-w, h, -d, 1],
            ...[-w, -h, -d, 1],
            // front (0, 1, 0)
            ...[w, h, -d, 1],
            ...[-w, h, -d, 1],
            ...[-w, h, d, 1],
            ...[w, h, d, 1],
            // back (0, -1, 0)
            ...[w, -h, d, 1],
            ...[-w, -h, d, 1],
            ...[-w, -h, -d, 1],
            ...[w, -h, -d, 1],
        ])
        const normals = new Float32Array([
            // top (0, 0, 1)
            ...[0, 1, 0],
            ...[0, 1, 0],
            ...[0, 1, 0],
            ...[0, 1, 0],
            // bottom (0, 0, -1)
            ...[0, -1, 0],
            ...[0, -1, 0],
            ...[0, -1, 0],
            ...[0, -1, 0],
            // right (1, 0, 0)
            ...[1, 0, 0],
            ...[1, 0, 0],
            ...[1, 0, 0],
            ...[1, 0, 0],
            // left (-1, 0, 0)
            ...[-1, 0, 0],
            ...[-1, 0, 0],
            ...[-1, 0, 0],
            ...[-1, 0, 0],
            // front (0, 1, 0)
            ...[0, 0, 1],
            ...[0, 0, 1],
            ...[0, 0, 1],
            ...[0, 0, 1],
            // back (0, -1, 0)
            ...[0, 0, -1],
            ...[0, 0, -1],
            ...[0, 0, -1],
            ...[0, 0, -1],
        ])
        const uvs = new Float32Array([
            // top (0, 0, 1)
            ...[0, 0],
            ...[1, 0],
            ...[1, 1],
            ...[0, 1],
            // bottom (0, 0, -1)
            ...[1, 0],
            ...[0, 0],
            ...[0, 1],
            ...[1, 1],
            // right (1, 0, 0)
            ...[0, 0],
            ...[1, 0],
            ...[1, 1],
            ...[0, 1],
            // left (-1, 0, 0)
            ...[1, 0],
            ...[0, 0],
            ...[0, 1],
            ...[1, 1],
            // front (0, 1, 0)
            ...[1, 0],
            ...[0, 0],
            ...[0, 1],
            ...[1, 1],
            // back (0, -1, 0)
            ...[0, 0],
            ...[1, 0],
            ...[1, 1],
            ...[0, 1],
        ])
        const indices = new Uint16Array([
            0, 1, 2, 2, 3, 0, // top
            4, 5, 6, 6, 7, 4, // bottom
            8, 9, 10, 10, 11, 8, // right
            12, 13, 14, 14, 15, 12, // left
            16, 17, 18, 18, 19, 16, // front
            20, 21, 22, 22, 23, 20, // back
        ])
        super([
            new VertexAttribute(positions, 4),
            new VertexAttribute(normals, 3),
            new VertexAttribute(uvs, 2)
        ], new IndexAttribute(indices))

    }
}