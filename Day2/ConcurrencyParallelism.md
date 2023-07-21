# Terminology Time: Concurrency vs Parallelism

Concurrency and Parallelism are related, but they are not the same thing. Parallelism refers to running multiple tasks at a time, on different CPU cores. Concurrency refers to engaging in multiple tasks at a time. You can concurrently talk to a number of customers on the same CPU core---but without parallelism, you won't talk to any of them at the exact same moment.

We're going to start today by talking about system threads: which are a parallel structure. We'll then move on to green threads, which are a concurrent structure. We'll then talk about how to mix the two.
