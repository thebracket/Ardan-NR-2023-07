# Domain Boundaries and Network Calls

Whether you have a monolith or a host of small services (I prefer starting with a modular monolith and scaling out services if demand requires it), there's a tendency to compartmentalize code. Overall, this is a good thing. Teams or individuals can take ownership of sections of the program, and with sufficient testing to ensure that they do what they say they will do, you have a working system and can still scale up the developer pool as you grow.

Problems *always* crop-up at domain boundaries.

## Defensive Programming

You can avoid a LOT of bugs by:

* Minimize the publicly accessible portion of your code. Expose high-level functionality that "fans out" into implementation.
* At the interface, check your preconditions. Not just user input! If your system only works with a range of 0 to 10, check that! Make sure your unit tests check that trying it produces an error, too. This can be a great use-case for the builder pattern with `?`.
* Use strong typing for inputs to ensure that external callers *can't* give you completely inappropriate input by accident.
* Use `debug` and `info` tracing liberally. It'll make tracking down what went wrong a lot easier.

## FFI

When you bind some external code, there's a few things to consider:

* You need to validate the code's inputs, unless you *really* trust the code you are wrapping.
* When you call into Go or Python code, the other language's FFI interface includes some marshaling that is MUCH slower than a pure C call (which is effectively free). So always make sure that the Go code you are calling does enough in one call to warrant the delay. Favor calls that do a lot, over lots of tiny calls---because you are paying for each call.

## Network Calls

> Aside: I once worked with a team that had a distributed architecture (this was back before the word "microservices" was popular). A request came in, which triggered a call to the authenticator and the service locator. The call was then passed to the appropriate service---which in turn called the authenticator again, and used the service locator to locate the calls it required. In one case, retrieving a balance required over 30 different network calls. Latency was, to say the least, painful.

It's *really* common to call services, which may call other services. It's very popular to do this with HTTP(s) calls. If you trace performance on these calls, the single largest time consumer is often... opening the network TCP connection. After that, TCP "slow start" prevents the first few bytes from arriving instantly. Even a call to localhost over TCP can take microseconds to open the connection---even if the call resolves in nanoseconds.

You can mitigate this a bit:
* If you can, run calls concurrently.
* Where possible, open a TCP connection and use a binary protocol. KEEP the connection open and stream requests/replies through it.
* Ask the hard question: does the service actually need to be running remotely?

