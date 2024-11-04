1. The user connects to the Websocket endpoint
2. A `WsConn` actor is spawned and started
3. The actor sends a `Connect` message to the `Lobby actor`. The Connect message contains
	1. `WsConn` address
	2. `lobby_id`
	3. `Actor's id`
4. `Lobby actor` adds actors_id to the lobby's `rooms` HashMap state
5. Optional: Lobby actor informs everyone in the the room that a new user has joined.
	1. using the send_message function that gets actor's id as input
	2. the send_message function then gets the socket (actor address) of that user from sessions hashmap in the lobby's actor state
	3. it sends the message to the actor with `do_send`
6. 