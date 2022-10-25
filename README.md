# client

Tests a server by sending lots of parallel requests! 

1. `payload.json` is the payload that will be send to server.
2. `address` contains an int indicating number of parallel requests, and in the next line the address of server.

`cargo run --release`

For OS configs about lots of parallel request see [this](http://woshub.com/too-many-open-files-error-linux/)


Note: only works on linux.
