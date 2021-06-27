import { Matrix } from '../math/mod.ts'
export class Camera {
    public projection: Matrix
    public view:       Matrix
    constructor(
        private readonly field:  number, 
        private readonly aspect: number,
        private readonly near:   number,
        private readonly far:    number
    ) {
        this.projection = Matrix.perspectiveFov(field, aspect, near, far)
        this.view       = Matrix.identity()
    }
}