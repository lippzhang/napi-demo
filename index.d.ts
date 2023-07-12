/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

export function sum(a: number, b: number): number
export const DEFAULT_COST: number
export const enum Kind {
  Duck = 0,
  Dog = 1,
  Cat = 2
}
export function coolFunction(aArg: number): number
export function getUndefined(): void
export function log(n: number): void
export function getNull(): null
export function getEnv(env: string): string | null
export interface TypeDemo {
  name: string
}
export function read(value: string | TypeDemo): string
export interface FnObject {
  name: string
  nameFn: (error, string) => string
}
export function read2(value: string | FnObject): void
export function callThreadsafeFunction(callback: (...args: any[]) => any): void
export function isGood(): boolean
export function keys(obj: object): Array<string>
export function createObj(): object
/** #[napi(object)] requires all struct fields to be public */
export interface PackageJson {
  name: string
  version: string
  dependencies?: Record<string, string>
  devDependencies?: Record<string, string>
}
export function logPackageName(packageJson: PackageJson): void
export function readPackageJson(): PackageJson
export function arrLen(arr: unknown[]): number
export function getTupleArray(): unknown[]
export function vecLen(nums: Array<number>): number
export function getNums(): Array<number>
export function convertU32Array(input: Uint32Array): Array<number>
export function createExternalTypedArray(): Uint32Array
export function mutateTypedArray(input: Float32Array): void
export class Animal {
  name: string
  kind: number
  constructor(name: string, kind: number)
  changeName(newName: string): void
}
