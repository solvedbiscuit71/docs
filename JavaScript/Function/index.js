/*
Hoisting: 
  moving the declaration (only) of variables & function
  to the top of the file.
*/

console.log(userName) // undefined (not Reference Error)
var userName = 'praveen';

/*
Variables defined with 'var' are hoisted.
NOTE: assigment will work only after the assignment is done.

To aviod hoisting variables use 'let'
*/

console.log(say('praveen')) /* Eventhough, it function is declared below it's actually hoisted to top. */
function say(name = 'solvedbiscuit71') {
  return `my name is ${name}`
}

/*
Think what will happen in these cases?
*/
console.log(typeof sayName) // output ??
var sayName = function (name = 'solvedbiscuit71') {
  return `my name is ${name}`
}
console.log(sayName())

console.log(typeof sayName2) // output??
let sayName2 = name => `my name is ${name}`