export class CheckError {
    readonly status = 400
    constructor(public message: unknown) {}
}