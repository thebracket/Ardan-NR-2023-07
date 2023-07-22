# Don't Emulate Object Oriented Programming

With traits, it's easy to think of Rust as an object-oriented programming language. Traits share *some* characteristics - they are basically interfaces. But there's no inheritance. At all.

If you're used to some OOP-style systems, it might be tempting to do something like this:

```rust
struct Employee;

impl Name for Employee { .. }
impl Address for Employee { .. }
impl Salary for Employee { .. }
```

You wind up with an employee object, and with the `Any` system you *can* cast it into the type you need. Doing so will make your life miserable.

You'll find yourself with an abundance of "if person has a Name trait", "if person has an address trait" - and then if you need to alter BOTH, the borrow checker makes your life painful. You can't mutably borrow the same base object for each trait, so you wind up writing a cycle of "look up data, note the new data, apply each in turn". That works, but it's big and messy.

Instead, favor *composition*:

```rust
struct Name;
struct Address;
struct Salary;

struct Employee {
    name: Option<Name>,
    address: Option<Address>,
    salary: Option<Salary>,
}
```

Now a function at `Employee` level can gain mutable access to each of the properties.

## Think in terms of Ownership

On top of that, it's beneficial to think in terms of ownership from Rust's perspective. If you have a big list of employees, and want to transfer a red swingline stapler from one person to another - you need to find both people, find out if they have the stapler, and then move it. In a textbook OOP setup, you'd have methods at each level - and probably pass one person to another for the transfer. Rust will be much happier if you implement the operation at the top level, take the time to check each precondition (and return an appropriate error).

Your code will be easier to test, too.