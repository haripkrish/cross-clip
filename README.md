# cross_clip



user journey :
1. Application Start:
    a. Create new session
        i. Generate New key pair.
    b. Join a session
        i. Input a seed phrase
        ii. Convert to key pair
2. Message store object.
    Create
    Update if join successful
3. Subscribe to listen to the messages across devices
Consider : Topic name = pub-key
4. Listen to events and store on the otherside
Decide on the attributes.

If a new device joins, broadcast messages/Fetch latest message.


# Dev Notes 
Run command `flutter_rust_bridge_codegen generate --watch`
