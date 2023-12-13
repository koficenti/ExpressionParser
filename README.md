# ExpressionParser

Made with "Rust" using "Recursive Descent"

Last night (12/12/2023), I was programming in Elixir as one does but all the dynamic typing made me a little sick.

Nothing wrong with Elixir, just sometimes it requires more thought and oh well the error messages are pretty bad.

And so I made this in Rust real quick. 

---

The following is the output of the test, works well enough.

Does in fact need some error handling so feel free to add that 😉

```
Assert (2 * 2) + (5) == 9
Assert 3 + 4 - 2 == 5
Assert 10 / 2 * 3 == 15
Assert (7 - 3) * 2 == 8
Assert 6 / (2 + 1) == 2
Assert (4 * 2) + (6 / 3) == 10
Assert 5 * (3 + 2) - 8 == 17
Assert (9 / 3) + (2 * 2) == 7
Assert (5 + 2) * (8 - 3) == 35
Assert 12 / 4 + (6 - 2) == 7
Assert 4 - 2 + 7 * 3 == 23
Assert 8 / 2 - 1 + 5 == 8
Assert (3 * 2) / (1 + 1) == 3
Assert (5 + 3) * (2 - 1) == 8
Assert 9 * 2 / (6 - 3) == 6
Assert 10 / (2 * 5) + 1 == 2
Assert 7 * (4 - 2) + 6 / 3 == 16
Assert (6 + 2) * (9 - 3) / 2 == 24
Assert (4 * 3) + 2 - 5 / 1 == 9
Assert 8 / 2 * (4 + 1) == 20
Wowz, it works
```


## Grammar
```
expr       -> term ( ('+' | '-') term )*
term       -> factor ( ('*' | '/') factor )*
factor     -> '(' expr ')' | NUMBER
NUMBER     -> [0-9]+
```

I think I used the right notation 🤔



## Thanks for checking my repo out!

Enjoy.
