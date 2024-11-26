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
- [ ] don't show "loading" when on the overview page!
- [ ] add integration tests


## p2

- [ ] add profile setup
- [ ] add email verification
- [ ] add phone number verification
- [ ] keep header / navbar sticky


## p1

- [ ] update websocket messages implementation


## doing

- [ ] update apis for each entity
- [ ] check CRUD capabilities of each entity


## tested



## done

- [ ] add errors.rs and map each error to our own custom error
- [ ] add authorization middlewear
	- Done with extractors instead of middlewear, the better known approach
- [ ] replace flywaydb with sqlx migrate
- [ ] add unit tests
- [ ] update project structure
- [ ] fix attributes of each entity
- [ ] add tasks
- [ ] going to /login page should redirect you to the landing page if already logged in
- [ ] fix token validation




%% kanban:settings
```
{"kanban-plugin":"board","list-collapse":[false,false,false,false,false,false,false]}
```
%%