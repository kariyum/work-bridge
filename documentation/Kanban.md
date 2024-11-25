---

kanban-plugin: board

---

## tobe



## p3

- [ ] change id to uuid for entities that needs this change
- [ ] add dynamic postgres schema support
- [ ] fix cors
- [ ] use bson intead of json to obfuscate request responses (not human readable) 
	Consider: 
	- grpc-web
	- MessagePack
- [ ] add role switching


## p2

- [ ] add profile setup
- [ ] add email verification
- [ ] add phone number verification
- [ ] keep header / navbar sticky


## p1

- [ ] add authorization middlewear
- [ ] update websocket messages implementation


## doing

- [ ] update apis for each entity
- [ ] check CRUD capabilities of each entity
- [ ] update project structure
- [ ] add unit tests
- [ ] replace flywaydb with sqlx migrate


## tested



## done

- [ ] fix attributes of each entity
- [ ] add tasks
- [ ] going to /login page should redirect you to the landing page if already logged in
- [ ] fix token validation




%% kanban:settings
```
{"kanban-plugin":"board","list-collapse":[false,false,false,false,false,false,false]}
```
%%