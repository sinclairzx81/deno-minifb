import { Matrix } from '../math/mod.ts'

export class Object3D {
    public matrix:  Matrix
    public objects: Object3D[]
    constructor() {
        this.matrix  = Matrix.identity()
        this.objects = []
    }
}