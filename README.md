# Rust Workshop - New Relic

![](/images/ardanlabs-logo.png)

Presented by [Ardan Labs](https://www.ardanlabs.com/)

---

## Requirements

* Rust installed via RustUp
* Git installed and working
* Basic Rust knowledge - variables, functions, control flow, etc.

## Outline

* Day 1
    * [Introduction](./Day1/Introduction.md)
    * [Rust Setup](./Day1/SetupRust.md)
    * [Ownership & Borrowing - Concepts](./Day1/OwnershipConcepts.md)
    * [Ownership & Borrowing - Sharing Data](./Day1/SharingData.md)
    * [Understanding Traits](./Day1/Traits.md)
* Day 2
    * [Concurrency & Parallelism](./Day2/ConcurrencyParallelism.md)
    * [System Threads](./Day2/SystemThreads.md)
    * [Green Threads and Async](./Day2/AsyncAwait.md)
    * [Tokio](./Day2/Tokio.md)
    * [Blocking](./Day2/Blocking.md)
    * [Channels, and talking to Threads](./Day2/AsyncChannels.md)
    * [Build a Web Service in 20 Minutes](./Day2/AxumService.md)
    * [High-Performance File Streaming and Processing](./Day2/Files.md)
    * [Calling External Programs](./Day2/ExternalPrograms.md)
* Day 3
    * [As Requested - Killing Child Processes](./Day3/ChildProcess.md)
    * [Databases](./Day3/Databases.md)
    * [Dependency Injection and Shared State in Axum Services](./Day3/SharedState.md)
    * [Error Handling](./Day3/ErrorHandling.md)
    * [Unit Testing](./Day3/UnitTesting.md)
    * [Tracing](./Day3/Tracing.md)
    * [Benchmarking](./Day3/Benchmark.md)
* Day 4
    * [Unit Testing 2 - Mocking/Property Testing](./Day3/UnitTesting.md)
    * QA
    * **Best Practices: Tooling**
        * [Formatting](./Day4/Formatting.md)
        * [Clippy (the linter)](./Day4/Clippy.md)
        * [Documentation](./Day4/Documentation.md)
        * [Understanding Dependencies](./Day4/Dependencies.md)
        * [Managing Your Own Dependencies](./Day4/ManageDependencies.md)
        * [Checking for Vulnerabilities](./Day4/Audit.md)
        * [Check for Outdated Dependencies](./Day4/Outdated.md)
        * [Denying Dependencies by License](./Day4/Deny.md)
        * [Build Profiles and Smaller Binaries](./Day4/BuildProfiles.md)
    * **Code Best Practices**
        * [Favor Iterators](./Day4/Iterators.md)
        * [Minimize Cloning](./Day4/Clone.md)
        * [Don't Emulate OOP](./Day4/OOPs.md)
        * [Don't Reference Count Everything](./Day4/Rc.md)
        * [Favor Small Functions](./Day4/SmallFunctions.md)
        * [Clever Code](./Day4/Cleverness.md)
        * [Let the Type System Help You](./Day4/TypeSystem.md)
        * [Floating Point Numbers](./Day4/Floats.md)
        * [Platform- and Feature- Specific Code](./Day4/PlatformSpecific.md)
    * **General Best Practices**
        * [TANSTAAFL: There Aint No Such Thing As A Free Lunch](./Day4/TANSTAAFL.md)
        * [YAGNI: You Aint Gonna Need It](./Day4/YAGNI.md)
        * [Domain Boundaries and Network Calls](./Day4/DomainBoundaries.md)
        * [Taming Compile Times](./Day4/CompileTimes.md)
    * Wrap-Up