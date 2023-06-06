let socket = new WebSocket("ws://localhost:8080/b535ecbf-c971-4328-a4da-4c7484904905");
setInterval(() => {
  socket.send("Hello, world");
}, 3000)

socket.onmessage = (e) => {
  const d = document.createElement("div")
  d.innerHTML = e.data
  document.body.append(d)
}
