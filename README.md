# ExpressionParser
Made with "Rust" using "Recursive Descent"

Last night (12/12/2023), I was programming in Elixir as one does but all the dynamic typing made me a little sick.
So I made this in Rust real quick.



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
