# FOSSil Chat Protocol 
FOSSil Chat currently uses four packet types:
- Packet::Join()
- Packet::Leave()
- Packet::Message()
- Packet::Error()
## Packet::Join()
This packet is sent when a user joins, taking one String argument (the username).
## Packet::Leave()
This packet is sent when a user leaves, taking one String argument (the username).
## Packet::Message()
This packet is sent when a user sends a message, taking two String arguments: the username, and the message content.
## Packet::Error()
This packet is sent when the server finds erroneous data in the packets sent by the client. This tells the client to close the connection and inform the user of the issue.