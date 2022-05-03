console.log('-- array --')
let array = [1,2,3,4,5,'a','b','c',true,[6,7]];

console.log(array[1])
console.log(array[-1]) // Here, negative indexing doesn't work
console.log(array[array.length - 1][0])

/*
  Note: pop() returns the value that's removed.
*/
console.log(array.pop())

/*
  for-in loop : used for iterating over keys of an iterable object.
  for-of loop : used for iterating over values of an iterable object.
*/
let output = []
for (let i in array) {
  output.push(+i) // Here, +i means convert to number if possible else NaN
}
console.log(output)

output = []
for (let x of array) {
  output.push(x)
}
console.log(output)

/*
  forEach & other advanced function takes an callback function.
  that will be called for every item in the array.

  parameter of callback:
  - item
  - index
  - array itself.
*/
output = []
array.forEach((x,i) => {
  output.unshift([x,i])
})
console.log(output)

/*
  find, findIndex & filter take a callback function with which it applys condition.
  1. if cb() returns 'Truthy' for an item => condition satisfied
  2. else cb() returns 'Falsy' for an item => item doesn't satisfy the condition
*/
const onlyNum = array.filter(item => {
  if (typeof item == 'number') {
    return true
  }
})
console.log(onlyNum)

/*
  map function expects the callback to return the value that will take it's place
  Ex:
    function cb(item) {
      return item * 2
    }

    for (let x of original) {
      mapped.push(cb(x))
    }
    return mapped
*/
const multi2 = onlyNum.map(item => item * 2)
console.log(multi2)

/*
  reduce function is used to compute some value based on all item in array.
  say, sum of all element in array
  - sum = 0
  - iter over items -> sum += item
  - return sum
  
  function cb(value, item) {
    return value + item
  }

  sum = 0
  for (let x of array) {
    sum = cb(sum, x)
  }
  return sum
*/
console.log(multi2.reduce((value, item) => {
  return value + item;
},0))

console.log('-- object --')

let person = {
  name: 'praveen',
  'full-name': 'praveen perumal',
  age: 18,
}

console.log(person.name)
console.log(person["full-name"])

/*
  Object.entries() returns an iterator of [key,value] pairs as it's value
  Also, use Object.keys() & Object.values()
*/
for (let [key,value] of Object.entries(person)) {
  console.log(`${key}: ${value}`)
}

console.log('-- object destructuring --')
let { name: firstName, age } = person
console.log(firstName, age)

let arr1 = [1,2]
let arr2 = [3,4]

console.log(arr1 + arr2)
/*
Here, type conversion takes place.
*/

console.log([...arr1, ...arr2, 5])

const getName = function({ 'full-name': name }) {
  return name
}
console.log(getName(person))