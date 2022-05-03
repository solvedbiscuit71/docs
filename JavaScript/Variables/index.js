/* 
What a block ?

- anything within a parenthesis is a block.
- if {}, for {}, ..
*/

/* --comment line below-- */
var profileName = 'praveen';
if (true) {
  var profileName = 'teju';
}
console.log(profileName)

/* --comment line below-- */
let userName = 'praveen';
if (true) {
  let userName = 'teju'
}
console.log(userName)

/*
What about function ?

in function both var and let behaves the same
since, function is also a 'block'
*/

/* --comment line below-- */
var yourAge = 17;
function getYourAge() {
  var yourAge = 20;
}
getYourAge()
console.log(yourAge)

// -- try comment the line below --
let myAge = 17;
function getMyAge() {
  let yourAge = 20;
}
getMyAge()
console.log(myAge)