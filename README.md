# learning_rust
learning rust

based on https://doc.rust-lang.org/book



# Inconsistent moving for pattern _

In pat4.rs and pat4_1.rs (which corresponds to The Book 18.3 #Ignoring an Unused Variable ...)

- pat4_1.rs (which is listing 18-21 and 18-22 combined) demonstrates that the value is not moved when the pattern uses '_'

- pat4.rs demonstrates that the second value of fn foo() is moved despite being called '_'
