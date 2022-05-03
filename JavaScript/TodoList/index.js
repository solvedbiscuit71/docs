function eventListener(element, eventType, callback) {
  element.addEventListener(eventType, (event) => {
    event.preventDefault()
    callback(event)
  })
}

function bindValue(element, state, setState) {
  element.value = state
  element.oninput = (event) => {
    setState(event.target.value)
  }
}

let todo = ''
let todoList = []

let setTodo = ($todo) => {
  todo = $todo;
  const input = document.getElementById('todo')
  input.value = todo
}

let setTodoList = ($todoList) => {
  todoList = $todoList;

  const ul = document.getElementById('todo-list')
  ul.innerHTML = ''

  todoList.forEach(item => {
    const li = document.createElement('li')
    li.innerHTML = `<input type="checkbox"> <span>${item}</span>`
    ul.appendChild(li);
  })
}
setTodoList(['Add your first todo 🎉']);

(function(){
  const input = document.getElementById('todo')
  bindValue(input,todo,setTodo)
})();

(function(){
  const form = document.getElementById('form')
  eventListener(form,'submit',(event) => {
    if (todo === '') {
      return
    }

    setTodoList([...todoList, todo])
    setTodo('')
  })
})();