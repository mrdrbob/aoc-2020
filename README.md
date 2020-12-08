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

### Day 3 - Part 2

I do appreciate these challenges where you build a simple algorithm and then must generalize it. I didn't do this the most efficient way possible, I ended up iterating the entire file for each rule being processed. If I'd build my `Rule` structure to keep the `x` and `y` state for itself, I probably could calculate all the values while iterating the file only once. But the file is not large and I'm aiming for simplicity.

I also opted to include the first point (0, 0) in the total, though I don't think it's super clear if that was required. Should a tree at (0, 0) be counted? I thought so, so I put my logic for moving to the next positions *after* counting, which makes it read a little backwards.

### Day 4 - Part 1

So for this one there's a bunch of records. Sometimes the records have newlines, but records are separated by an empty line. Again, I kind of took the easy route. I read the entire file into memory, then split on double new lines. Then I split on single new-lines and flatted the results to build a simple key-value pair representation of the record.

Once that was done, it just checking that all the required keys existed for each record. I wanted my list of required keys to be data, so I attempted to make a `const` array of those keys. Not 100% certain I did that correctly.

### Day 4 - Part 2

I won't lie, this part of the challenge felt a lot like work. Probably because I do a fair amount of validation at my day job, so this wasn't much different. I did decide not to use regexes. The most challenging part was just getting all the little details right, and translating the C# `Where`/`Select`/etc in my head to similar constructs in Rust.

### Day 5 - Part 1

Reading the description for today's problem I couldn't help but think, "well that's just binary with extra steps." So I popped open my trusted programmers calculator, converted the characters into the ones and zeros, and sure enough, it was.

Now, I probably could have done the conversion a bit more manually (instead of relying on `from_str_radix`), but it's Saturday and I'm a web developer; I can count on `0b0101` fingers how many times I've needed to do binary math in my daily life.

### Day 5 - Part 2

This part was a relatively simple variation of part one. I just throw all the known seats into a `HashSet` (for quick lookup), and then iterate all possible seat numbers (the seats are made of a ten bit number, so one through `2 pow 10`), looking for a seat that does not exist, but where the previous and next ID do. As with many of these puzzles, I'm left feeling like there's probably a more clever way to do this... but `¯\_(ツ)_/¯`

### Day 6 - Part 1

Once again, I'm just using a `HashSet` to get unique items. Then I'm returning the length of that set.

### Day 6 - Part 2

So my solve for this was probably a bit on the "hacky" side, but it works. I'm sure a more clever Rust programer would do something with intersecting sets, or at least using map/reduce in place of the nested `for` loops I opted for. But it's Sunday, this works, it's good enough.

### Day 7 - Part 1

Parsing the input was kind of annoying on this one, and I'll be the first to admit my code is anything but elegant. It finds the solution, albeit pretty slowly. I imagine if I'd keep my data in a dictionary (instead of a Vec), and cache calculated results, this would be quite a bit faster.

**Addendum** - Well, I tweaked it a bit. I didn't add the cache, but I did switch the structure to using a `HashMap` instead of a `Vec`. I also encapsulated the collection in a struct so I don't have to pass around quite so many parameters.

### Day 7 - Part 2

Part two is a relatively simple variation on the recursive function from part one. The logic in the aggregate function is easy to get wrong (`accumulator + count + (count * recursive count)`) if you're like me and you just start pounding on the keyboard rather than thinking through things.
