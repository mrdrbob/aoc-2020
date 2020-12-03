# Advent of Code 2020

Hey! I'm doing Advent of Code again. And once again, I'm using Rust. And once again, I don't know Rust, or what I'm doing. Perhaps this year I'll make it past day 12.

## Dev Log

### Day 1 - Part 1

So part one is simple: you get a list of numbers. Two numbers in the list add up to 2020. Find those numbers and multiply them.

So the obvious answer is to loop through the list, then in each iteration, loop through the list and add the two numbers. If they add upto 2020, you've got your answer. But then, you don't need to add anything to know what the other number is. You can loop through the list, subtract the number you're looking at from 2020, and then checking to see if that answer exists elsewhere in the list. Any time I'm checking if something exists in a list, I consider throwing it in a set for quick lookups.

So that's what I do. Read all lines into a `HashSet`, then iterate the `HashSet` checking to see if `(2020 -  that number)` exists in the set. If I find one (and I assume I do), then return `(that number * (2020 - that number))`.

Challenges: I still don't have an intuitive understanding of borrowing in Rust. Luckily, I have the very efficient and intelligent strategy of "try random things until the compiler stops complaining".

Potential edge case: If the input contains only a single `1010`, that can be added to itself, and the "contains" lookup doesn't care if it's looking up the same number. So, the algorithm would incorrectly return `1010 * 1010`, when only one `1010` exists in the list.

### Day 1 - Part 2

Foiled! My brilliant scheme has been destroyed by the introduction of a third factor. So I went back to the original plan of just brute forcing the list--it's not that long. I must admit, I cringe when I see `for` loops nested three deep.

My one solace is that I don't attempt to add numbers to themselves. So if 1000 and 20 were in the list, it wouldn't consider adding 1000 + 1000 + 20, since 1000 only appears once. At least I *think* I got that filtering correct.

### Day 2 - Part 1

Part one is pretty straight forward. Read a list of password rules and see if the corresponding password satisfies the rule. The hard part (for me) was still figuring out the idiosyncrasies of Rust. Something like `line.as_ref().unwrap().as_str()` is probably the Rust equivalent of something like `((string)name.ToString()) as String;` in C#.

### Day 2 - Part 2

This one is just a small tweak to the password check function. I chose to sum each character that matched and then check that the total was `1`. I suppose I could have also done a `if`/`else` tree, or maybe something with a `XOR` and an `OR`.

On a side-note, in a language where I'm comfortable with delegates/lambdas, I might have done something a bit more functional and created delegates for each rule. Then executed the delegates on the input. Something like:

```
Func<string, bool> CreatePasswordTest(int min, int max, char character)
	=> (password) => password.Count(x => x == character) <= max && password.Count(x => x == character) >= min;

var rule = CreatePasswordTest(min, max, c);
var succeeds = rule(password);
```

(I didn't test this.) I don't think there's really any benefit in this case, other than looking kind of cool, I guess.

### Day 3 - Part 1

Not much to write about this one. I'm taking advantage of the fact that the `y` position always moves exactly 1, so I just iterate the lines to do that. Then just rotating the `x` position along the number line based on the length of the line.
