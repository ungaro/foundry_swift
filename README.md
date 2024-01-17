# foundry-swift
## To Run

```
git clone https://github.com/chinedufn/swift-bridge
put foundry_swift repo under examples/foundry_swift

cd examples/foundry_swift
./build.sh
./main
```

Here's some example output:

```sh
We're in Swift about to call our async Rust function.
Starting HTTP request from the Rust side...
HTTP request complete. Returning the value to Swift...
Now we're in Swift again. IP address: 123.4.56.7
```

[reqwest]: https://github.com/seanmonstar/reqwest
