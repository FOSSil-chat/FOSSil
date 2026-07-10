# FOSSil Chat Architecture
FOSSil Chat currently follows a simple client-server architecture, communicating using TCP (Transmission Control Protocol).
## Components
- Client: The program the user runs to send messages.
- Server: The program that receives these messages and logs them.
- Shared library: Contains Packet and Message, which the client and server need to both be able to compare.

We chose Rust as our main language for its memory safety, speed, and potential. In addition, the project uses Tokio for multi-threading and JSON over TCP for clean, simple packet transmission.
## Design
The user (client) is asked to enter their name. Once they submit it, it is sent to the server using JSON over TCP - if the username already exists, the server sends back an Error packet, closing the connection, and if the username is original, the server accepts the oncoming connection. Then, the user is able to type messages - when they submit a message, it is converted to JSON and sent to the server, where it checks for erroneous content and logs it. If the user types !exit, the client does not send it as a packet, but instead sends a Leave packet to the server, deleting the user from the server state and cleanly exiting the connnection. In case user closes the program unexpectedly, the server requires keepalive packets, closing the connection if the keepalive is not reponded to.