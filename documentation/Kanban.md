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
- [ ] make richtexteditor buttons unfocused with TAB
- [ ] Assign tasks to applicants after accepting the proposal
- [ ] ritchtexteditor buttons do not react when the cursor is on H1 H2 text while before the cursor was writing in P


## p2

- [ ] add email verification
- [ ] add phone number verification
- [ ] keep header / navbar sticky
- [ ] deleting a project makes the screen refresh because of the asynchronous behavior. Fix that.


## p1

- [ ] update websocket messages implementation


## doing

- [ ] add profile setup
- [ ] add complete profile setup


## tested



## done

- [ ] upon logging in route the user to home page
- [ ] remove accessing writeable store in the +layout.ts page /
- [ ] deleting a project must delete a task!
- [ ] check CRUD capabilities of each entity
- [ ] rename assign to assign_id column
- [ ] update apis for each entity
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