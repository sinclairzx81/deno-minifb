import { Geometry } from '../geometry/mod.ts'
import { Material } from '../material/mod.ts'
import { Object3D } from './object.ts'

export class Mesh extends Object3D {
    constructor(
        public readonly geometry: Geometry,
        public readonly material: Material
    ) {
        super()
    }
}