# Idea
Let’s write a small grep clone. That is a tool that we can give a string and a path and it’ll print only the lines that contain the given string. Let’s call it grrs (pronounced “grass”).

# Parsing CLI

` grrs foobar test.txt `
We expect our program to look at test.txt and print out the lines that contain foobar. But how do we get these two values?

The text after the name of the program is often called the “command-line arguments”, or “command-line flags”
Internally, the operating system usually represents them as a list of strings – roughly speaking, they get separated by spaces.

There are many ways to think about these arguments, and how to parse them into something more easy to work with.

You will also need to tell the users of your program which arguments they need to give and in which format they are expected.

## Getting CLI arugements

The standard library contains the function std::env::args() that gives you an iterator of the given arguments.

The first entry (at index 0) will be the name your program was called as (e.g. grrs), the ones that follow are what the user wrote afterwards.

`let pattern = std::env::args().nth(1).expect("no pattern given");`
`let path = std::env::args().nth(2).expect("no path given");`

## CLI Arguments as Data Types

Instead of thinking about them as a bunch of text, it often pays off to think of CLI arguments as a custom data type that represents the inputs to your program.

Look at grrs foobar test.txt: There are two arguments, first the pattern (the string to look for), and then the path (the file to look in).
