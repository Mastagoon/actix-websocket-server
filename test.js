let socket = new WebSocket("ws://localhost:8080/b535ecbf-c971-4328-a4da-4c7484904905");
setInterval(() => {
  const obj = {
    event: "NewClip",
    msg: "Test"
  }
  socket.send(JSON.stringify(obj));
}, 3000)

socket.onmessage = (e) => {
  const data = JSON.parse(e.data)
  console.log(data)
  const d = document.createElement("div")
  d.innerHTML = data.msg
  document.body.append(d)
}
