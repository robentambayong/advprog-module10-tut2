## Experiment 2.1: Original code of broadcast chat

**Server:**
![Server Output](Experiment%202.1%20(server).png)

**Client 1:**
![Client 1 Output](Experiment%202.1%20(client1).png)

**Client 2:**
![Client 2 Output](Experiment%202.1%20(client2).png)

**Explanation:**
To run this application, I opened three terminal windows. In the first, I started the server, which began listening on port 2000. In the other two, I started the clients, and you can see the server successfully logged their connections. 

When I typed "Hello" into the first client, `stdin.next_line()` captured it and sent it over the websocket. The server received it via `ws_stream.next()`, sent it into the `broadcast::channel`, and both clients received the message through their `bcast_rx.recv()` and printed it. The same process happened successfully when I typed "Hi" from the second client.

## Experiment 2.2: Modifying the websocket port

**Server:**
![Server Output 8080](Experiment%202.2%20(server).png)

**Client 1:**
![Client 1 Output 8080](Experiment%202.2%20(client1).png)

**Client 2:**
![Client 2 Output 8080](Experiment%202.2%20(client2).png)

**Explanation:**
For this experiment, I modified the application to run on port `8080` instead of `2000`. 
1. In `server.rs`, I changed the bind address to `TcpListener::bind("127.0.0.1:8080")` so the server actively listens for incoming handshakes on the new port. 
2. In `client.rs`, I updated the connection target to `ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))`.

As seen in the screenshots, the server successfully starts on port 8080. Both clients are able to connect to this new port, establish the WebSocket (`ws://`) connection, and seamlessly broadcast messages to each other just like before.