import { VertexAttribute, IndexAttribute } from './attributes.ts'

export class Geometry {
    constructor(
        private readonly attributes:     VertexAttribute[],
        private readonly indexAttribute: IndexAttribute
    ) {}
}