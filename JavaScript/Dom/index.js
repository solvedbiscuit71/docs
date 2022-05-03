let count = 0;
function setCount(newCount) {
  const span = document.getElementById('output')
  count = newCount
  span.innerText = count;
}

(function () {
  const button = document.getElementById('button')
  button.onclick = () => {
    setCount(count + 1)
  }
})()

/*
  Other approach,

  let count = 0
  let span = document.getElementById('output')
  let button = document.getElementById('button')

  function updateCount() {
    count++
    span.innerText = count;
  }

  button.addEventListener('click',(event) => {
    updateCount();
  })
*/