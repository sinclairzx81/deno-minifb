export class Camera {
    constructor(
        private readonly field:  number, 
        private readonly aspect: number,
        private readonly near:   number,
        private readonly far:    number) {
    }
}