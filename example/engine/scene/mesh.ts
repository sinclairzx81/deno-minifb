import { Geometry } from '../geometry/mod.ts'
import { Material } from '../material/mod.ts'
import { Object3D } from './object.ts'

export class Mesh extends Object3D {
    constructor(
        private readonly geometry: Geometry,
        private readonly material: Material
    ) {
        super()
    }
}