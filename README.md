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

### Day 15 - Part 1

I think there are two important things to consider when doing today's puzzle:

1. You don't need to know the entire history, just when the last time each number was spoken.
2. You need to note a number's history AFTER you've considered it for this round. I did this by processing the first n-1 numbers in the initial list, then "speaking" the last number in the list without yet adding its history.

### Day 15 - Part 2

So in my (admittedly limited) experience when the challenge is taking your solution from the previous day and running it for a lot longer, the idea is that your first solution is probably too slow, or that the results stabilize into a pattern, which you can then use to extrapolate the correct answer (without having to actually run all the iterations that otherwise would've been required).

In my case, I ran my solution in debug mode and closed it after like five seconds -- too slow. But then, on a whim, I ran it in release mode and it came back with the answer in like 1 second. I guess it's fast enough? Maybe I was supposed to find a smarter solution, but this worked, I'm going with it.

### Day 16 - Part 1

A *little* late. After talking with a colleague, I was reminded of AOC and thought I should revisit the challenges I hadn't gotten to. So I did. For part one, I immediately see two possible strategies:

1. Going through all the tickets and attempting to apply all possible rules to each, looking for rules that don't fit.
2. Pre-calculate all possible valid numbers, then iterate through all tickets looking for any number that isn't in the set.

I went with strategy two.

### Day 16 - Part 2

So part 2 largely ended up being a rewrite, taking more advantage of structures to organize code and hopefully make the main logic a little clearer. This one was a bit tricky, but really boils down to a few steps:

1. Iterate through all tickets and remove any for which no field is valid. This is basically part 1.
2. Next I build a collection of *possible* field-to-column mappings. This takes a few loops, which can make it hard to wrap your head around. But in short: Loop 1: Iterate through all the fields. Loop 2: For each field, iterate through all the columns. Loop 3: For each column, loop through all the tickets and keep the column mappings where all the values in *this* column are valid for *this* field.
3. Now each field has a list of possible column mappings. Many fields have multiple possible mappings, but at least one has only one possible mapping, so we know that one is correct.
4. We save the mapping we know to be correct, and then remove it from all possible mappings. Then we iterate again. And again. We repeat step 3 and 4 until all of the possible mappings have been found.
5. Finally, now that we know the columns that correspond to the departure fields, we can find and multiply the values from my ticket.

Important reminder: when trying to figure out the possible column mappings, don't forget to **include your own ticket**. If you forget that, you may be staring at your code for a very long time wondering why it doesn't work. Or at least, that's what I assume would happen if you forget. Which I totally didn't.

### Day 17 - Part 1

So for this one, the 3D space we're representing is infinite. So rather than doing something like 3 dimensional array, I did (basically) a three dimensional hashmap, sort of? The X plane, rather than being an array, is a hashmap where the key is the X coordinate, and the value is the Y plane. The Y plane is also a hashmap, where the key is the Y coordinate, and the value is the Z plane. The Z plane is just a hashset of values. You can think of it as a tree structure, the root is the X plane, the Y plan is a branch of X, and Z is a leaf of X.

I still feel like my pattern of removing an item from the map so I can mutate it, and the re-inserting is probably idiomatically wrong.

The big question then is: will this scale for part 2???

### Day 17 - Part 2

Spoiler: it did scale, after just manually adding another dimension to my structures. It does take a few seconds to calculate the final total, but technically works. I'm sure there are a lot of places I could optimize this. I allocate a vector to get a list of neighboring points, and that could probably just be an iterator instead. I check points multiple times when iterating through possibilities. I also remove/insert a lot of items from the world space, when I probably could be mutating those items directly instead. But, to put my feelings on this matter in technical terms: `¯\_(ツ)_/¯`.

It would be interesting to take the structures (`Point4d`, `Space4d`) and abstract out the number of dimensions, allowing for an arbitrary number of dimensions. I'm certain it could be made to work, but that's for someone with more free time and skill than me.

*Addendum*: Looking back on this, it occurs to me that I did that thing I love to do: needlessly complicated things. The idea of representing the dimensions as a kind of tree-like structure is kind of *neat*, but really, I could have just kept track of all the points in a flat set and achieved the same goal with quite a bit less fanfare. Hindsight, or maybe lack of foresight. Either way, I'm going to leave it.

### Day 18 - Part 1

My approach here was to tokenize the input, then process the tokens from left to right in a truly hideous, recursive state machine. The "nice" thing about it is that because I never build a syntax tree, I don't need to use a self-referential strut/enum, which can be problematic in Rust. I mean, it's not problematic for people who know what they're doing, but for *me* it was. At some point I will have to learn to box "`Box`" up a better solution for these kinds of problems… but the state machine technically works for now.

As for the state machine, probably the easiest way to understand it is to manually go through a simple equation, thinking about what state you start and end with for each character/token. For example: `5 + 2 * 2`

1. At the first token, `5`, you start with nothing. You're in an initial, or `Empty` state. But now you have a number! So you leave this token in a `Value` state.
2. The next token: `+`. You start with a value, and now you have an operation. You can't do anything else, so just keep the value and operation in mind. You leave this state in a `ValueWithOperation` state.
3. The next token: `2`. You start with a value and operation, and now you have another value, so can complete this operation. `5 + 2 = 7`. Since you've processed the values, you can leave this state with just your current value in mind: a `Value` state with a value of `7`.
4. The next token: `*`. As before, you're in a `Value` state, tack on the operation just like in 2. Leave with a `ValueWithOperation` state.
5. The next token: `2`. Just like in 3, you start in a `ValueWithOperation` state, complete the operation, and exit in a `Value` state.
6. No more input, you should expect to be in a `Value` state. If so, the operation was a success!

The parenthesis are handled recursively. The function for parsing basically calls itself. For example: `1 + (2 * 2) + 1`:

1. Process the `1` and the `+` as usual.
2. When you see reach the `(` character, push the current state (`ValueWithOperation`) on the stack, call the parser with a new, `Empty` state. Process that state in the usual fashion until the state machine hits the `)` character. At that point, the state machine should be in a `Value` state. Return that state.
3. Pop the `ValueWithOperation` state off the stack and apply the `Value` state from the recursive call. Continue processing.

The above describes roughly what's happening at a conceptual level. In reality, I'm not looping, but doing everything via recursion. I'm also keeping track of my location in the token stream. But the above is roughly the idea.

Also, for building state machines, Rust's enums and pattern matching are… *chef's kiss*.

### Day 18 - Part 2

This was a relatively small adjustment / hack to make the modified order of operations work. Currently, when the state machine encounters an operation, it expects to have a value in mind (i.e. it should enter in the `ValueOnly` state), and then it "remembers" the operation with the value (exit with the `ValueAndOperation` state), and then next time it finds value, it executes the remembered operator against the remembered value and new value. This effectively makes the machine work left-to-right.

Now when the machine encounters an operation, it branches based on the operation. Addition works exactly the same as before, it becomes a `ValueAndOperation` and moves on. For multiplication, it creates a new, `Empty` state and then recursively processes the next token, forcing the "right" side of this operation to be solved *first*, and *then* applies the multiplication. This effectively makes multiplication the last in line for order of operations.

Frankly, I have mixed feelings about this. On the one hand, it's clever. On the other, the *intent* of the code (process plus operations first, then multiplication) isn't super clear from the code. In a sense, I'm using what looks like a side-effect as core logic. Meh.

### Day 19 - Part 1

So what the challenge describes (at least to my mind) is effectively a parser: a collection of rules that are applied to a stream of tokens. So naturally I relied heavily on my [former experience](https://github.com/mrdrbob/parsing) and architected a solution with that in mind.

So in short, I consider each line of in put a stream of tokens. Then I based my solution on a series of possible rules:

* Match `Character` - Matches the current token to a particular character.
* Match `Rule` - Executes a rule at a  particular index for the current token.
* Match `Any` - Executes a collection of rules against the current token until one of them matches.
* Match `Each` - Executes a collection of rules, requiring each to match the next token in the stream.
* Match `End` - A special rule I use at the end of the stream to verify that the end of the stream has been met.

The token stream I represent as a vector of characters, and a `Position`, which just bundles up some of the logic for moving through the stream. Instead of returning either a character or `None` like a `chars()` iterator would, `Position` returns either the current position and a new `Position` representing the remaining stream, or `None` if we're at the end of the stream. Because `Position` is immutable, it makes back tracking when a rule doesn't pass (in `Any` for example) sort of automatic, by using the stack to track the position.

So then I execute the stream of tokens for each message against the rule tree I've built and count up the ones that succeed.

### Day 19 - Part 2

This one stumped me for a while. The wrinkle that's introduced is the fact that the new rules introduce sequences that could potentially match the left OR right side, and the right side is effectively a loop meaning the match could happen multiple times. The problem is that the `Any` rule is greedy, meaning it will accept whichever rule matches first, and then consumes that input.

To break it down a bit:

```
	 0: 8 11
     8: 42 | 42 8
	 11: 42 31 | 42 11 31
```

* Rule 0 is effectively "match rule 8, then rule 11"
* Rule 8 is effectively "match rule 42 between one and infinite times"
* Rule 11 is effectively "match rule 42 between one and infinite times then rule 31 between one and infinity times"

Rule 11 is interesting, because the left half is "42 followed by 31", the right side is "42, then myself, then 31." Making it effectively 42 one or more times, then 31 one or more times.  So if rule 42 was "a" and 31 was "b", then "aaabbb" could be a valid message.

The problem is that the rule resolution (as it is now) is going to match "42 followed by 31" and accept/consume those characters, then failing on the second "b". That's the eager consumption. But had it tried "42, myself, then 31", it would have worked.

The most general way to fix this would be to update rule parsing to backtrack to Any rules that matched but had other branches they could attempt, and attempt those branches in the event of failure further up the line. Like brute forcing a maze. But that would be… complicated. Also computationally expensive.

Instead, I manually implemented rules 0, 8, and 11.

Because rule 8 will consume all input matching 42, and rule 11 requires at least one 42 match, I run rule 8 until it no longer works, then verify that it succeeded at least twice. 

Then rule 11 is basically "42 at least once, then 31 at least once", and I know that all the 42-matching input has been consumed, I just run rule 31 multiple times, making sure it succeeds at least once.

Of course, this feels super hacky. Change two rules, then basically write code to *simulate* those two rules, ignoring the actual rules completely.
