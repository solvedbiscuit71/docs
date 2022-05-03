/* we will bind this state variable to input's state variable */
let email = ''
function setEmail(newEmail) {
  const log = document.getElementById('email-log')
  email = newEmail
  log.innerText = email;
}

/*
  There are 2 steps in two-way binding
  1. init the value
  2. add event listener (oninput)
*/
function bind(state, setState, id) {
  const input = document.getElementById(id)
  input.value = state;

  input.oninput = (event) => {
    setState(event.target.value)
  }

  /* handling reset */
  const form = document.getElementById('form')
  form.addEventListener('reset', (event) => {
    setState('')
  })
}
bind(email, setEmail, 'email-input')

/*
  onsubmit event handler on form
  1. by default, when submit a form the page refresh
  2. to avoid default behaviour call: event.preventDefault();
*/
const form = document.getElementById('form')
form.onsubmit = (event) => {
  event.preventDefault();

  /*
    in a form element, all it's input element will be stored,
    form.elements = {
      name: refs
    }

    where, name is the value set in `name=""` attribute.
  */
  const output = document.getElementById('email-submit')
  output.innerText = event.target.elements.email.value;

  /*
  Or use
  output.innerText = email;
  */
}