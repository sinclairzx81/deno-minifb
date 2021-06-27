import { VertexAttribute, IndexAttribute } from './attributes.ts'

export class Geometry {
    constructor(
        public readonly attributes: VertexAttribute[],
        public readonly indices:    IndexAttribute
    ) {}
}