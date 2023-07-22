# YAGNI

> You Aint Gonna Need It

This is another phrase I use a lot. It's always tempting to super gold-plate your interfaces, until you have something that supports everything you might ever need. And then years later, you're looking through and notice that nobody ever used some of the options.

Try to specify a narrow target that does what's needed. If there are *obvious* extensions, document them and bear them in mind with your initial design. If there are *obscure* extensions, note them down. But don't spend hours and hours adding "gold plated" features that may never be used.

The inverse of this is that if you *do* provide a kitchen-sink interface, you (or your successor) has to support it. The more options you offer, the more support you have in your future.

Look at Windows. Once something hits the Windows API, it *never* goes away. So you wind up with `CreateWindow`, `CreateWindowExt`, `CreateWindowWithOptions` and a giant combinatorial explosion of the API surface. That's something to avoid is at all possible.