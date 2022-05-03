let array = localStorage.getItem('array') || []
console.log('array',array)

// -- change it to false -- //
let isComplete = true;
let callback = () => {
  console.log('callback called!')
  return 200
}
console.log(isComplete && callback())

// -- try changing it to another number -- //
let num = 234;
console.log(num % 2 == 0 ? 'even' : 'odd')

/*
== -> does type conversion and then checks (use it only if you know the type)
=== -> doesn't do type conversion (use it)
*/
if (1 == '1') {
  console.log("1 == '1'")
}

if (1 === '1') {
  console.log("1 === '1'")
}else {
  console.log("=== is strict in typeof")
}

/*
Why are we getting 'zero'?? even if it's not zero
*/
switch (num) {
  case num > 0:
    console.log('greater')
    break;
  case num < 0:
    console.log('lesser')
    break;
  default:
    console.log('zero')
}

/*
Try, removing the
- '= 0' from for loop
- then, try 'let' from for loop
*/
let i = 0;
for(let i = 0; i < 5; i++) {
  console.log('inside loop',i)
}
console.log(i)