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

### Day 8 - Part 1

Part one was pretty straight forward, kind of fun to write. Just a simple interpreter and then a `HashSet` to keep track of any instructions that had been executed.

### Day 8 - Part 2

Another fun one to work out, though I spent a *lot* of time fighting borrowing mechanics. I'm not blaming borrowing here--this is 100% on me for not understanding it well enough. I finally put most of the ownership up in a `VirtualMachine` struct, and was able to stumble through the syntax enough to allow it to keep a reference to the list of instructions, but own the rest of the information.

Basically, here was my algorithm:

1. I create a VM and execute the instructions as-is until it detects an infinite loop (mostly code from day 1). Keep a list of every instruction that was visited.
2. Go back through all the instructions and check every `jmp` and `nop` and see if any (when swapped) hit an instruction that was not previously visited.
3. For every swapped instruction that ended up visiting an instruction that wasn't previously visited (in my case, there are about 284), execute the VM again, but with that instruction flipped, and see if it terminates correctly.

Of the 284 possible flips, only one did not end in an infinite loop.

### Day 9 - Part 1

So I think there are two key parts to this part of the challenge. The first is you need some kind of rolling buffer (that's probably not technically the right term). I'm using a `VecDeque`. So it's relatively simple to keep track of the last 25 seen numbers.

The second part is just scanning that buffer for matches. The one "optimization" I do here is skip comparisons I've already done. So I check the **first** number against the second, third, forth, etc. Then I check the **second** against the third, forth, fifth, etc. I don't compare second against first (since that comparison has already happened).

That said, lines like `n2.clone() + number.clone() == this_number.clone()` make me suspect I'm fundamentally doing this all wrong.

### Day 9 - Part 2

So for this one we need to find a continuous set of numbers that sum-up to the value found in the part 1. I won't lie, this one took a long time to get right. I had the basic concept, but getting the details exactly right took WAY too long.

Here's my strategy:

1. Keep two pointers, one to the end of the set and one to the front of the set.
2. Calculate the sum of the numbers between those indices. I do this in real-time by adding or subtracting values as I change the indices.
3. If my current sum is too small, move the end pointer ahead one and add the number it's pointing at.
4. If my current sum is too large, move the start pointer ahead one and subtract the number it was previously pointing at.
5. Iterate until the correct sum is found.

### Day 10 - Part 1

The devil is in the details. I ended up making this one more complicated than it is, but the key for me was to re-read the instructions and realize I need to include the difference between the 0 jolt port and the first adapter *and* the difference between the last adapter and the device (I cheated here in that the device is always 3 more than the highest joltage, so I just add 1 to the 3 jolts count).

### Day 10 - Part 2

So for today's challenge, the brute-force method of basically attempting every possibility would be prohibitively slow, not to mention complicated. For me, the key insight is to think of the list as sorted, but iterate through it from the largest to the smallest. For each item, you can look at the next three or so adapters and check which ones would be possible jumps. Then take the sum of their possible jumps. It's like recursion in reverse.

### Day 11 - Part 1

The worst part of this puzzle is the missed opportunity to call the seats "thrones" and make a game of life/thrones pun.

This one is mostly about getting all the details correct, and treating the map as immutable. If you make changes to the map as you calculate each node, you'll get the wrong answers. You need to calculate a **new** map for each iteration. While I do that, I also check if any value has changed to make detecting the loop easier.

### Day 11 - Part 2

So obviously in this one I had to rework the code that looked at the adjacent seats into code that could scan in the 8 directions. I did this by using an x and y offset and recursively calling itself if the spot being examined is floor. My `move_point` function is absolutely hideous and I'm quite certain there are far better ways to write that. But, it works.

### Day 12 - Part 1

Starting to fall behind, but such is the holidays, even in a COVID world. I cheaped out on this one and did giant logic trees for turning directions. Instead, I should have probably modeled directions and used maths to resolve the left and right turns, etc. What I did is not very... elegant. I just like `match`.

### Day 12 - Part 2

This is a relatively straight-forward modification of the existing algorithm. Rather than calculate rotations for 180 and 270, I just repeat the left and right rotations. The hardest part for me was keeping all the positive and negative values for the directions straight in my head.

### Day 13 - Part 1

This one isn't bad if you know how to use modulo. You're really just figuring out the remainder between the earliest possible departure time and each bus id. Then find the smallest of those.

### Day 13 - Part 2

I admit this one was tough to reason out. The actual programming was simple, but it took my poor old tired brain a while to get there. Here's what I'd give as hints, if you're trying to figure this one out:

1. The simplest thing is to iterate through all possible timestamps and check if all the buses (with their offset) could possibly land on that spot.
2. Quote: "surely the actual earliest timestamp will be larger than `100000000000000`". This gives you a good place to start.
3. You don't need to iterate *all* timestamps. Let's say your timestamp is 50 and the bus id is 19. You know 51, 52, 53.. through 68 will not possibly work, so skip them.  If multiple bus ids (for example: 2, 3, 5) all match, you know the next offset will also have to match all 3 of those ids. What's the next number that iterating 2 at a time, 3 at a time, and 5 at a time will hit?
4. Finally, there's something special about all the bus ids.

I won't spoil it here this time. You'll have to read the code.

### Day 14 - Part 1

I feel like part 1 is pretty straight forward if you know about bitwise operations. I think the key insight hint I'd give on this one is: treat the mask as two operations, a set and an unset.

### Day 14 - Part 2

This was an interesting one, in that you need to build apply all possible permutations of a set of bits. I decided to calculate those variations up front and store them with the instruction. The tricky part was generating (and to a lesser extent, applying) the variations. I did this by using a stack to effectively simulate recursion. I tried to document my process in the code.

As for applying the variations, I use an inverted mask of all the floating bits to clear all the corresponding bits on the address (`&`ing the `!` version of the mask). Then I set the bits according to the variant (by `|`ing the variant).
