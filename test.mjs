import { sum, DEFAULT_COST, Animal, read, getEnv, callThreadsafeFunction, read2 } from './index.js'
 
console.log('From native', sum(40, 2))
console.log('From native DEFAULT_COST =>', DEFAULT_COST)

let animal =  new Animal('cat', 3)
console.log('From native =>', animal.name)
animal.changeName('dog')
console.log('From native =>', animal.name)

const a1 = read('lippzhang')
const a2 = read({ name: 'lippzhang2'})
console.log('a1:', a1)
console.log('a2:', a2)

// console.log('From native =>',getEnv(null))
process.env.production = "lippzhang"
console.log('getEnv From native =>',getEnv('production'))

const cb = (err, result) => { 
  console.log(err, result)
}
// callThreadsafeFunction(cb)
read2({
  name: 'lippzh',
  nameFn: (error, string) => {
    console.log(error, string)
  }
})