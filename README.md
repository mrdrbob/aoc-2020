# Advent of Code 2020

Hey! I'm doing Advent of Code again. And once again, I'm using Rust. And once again, I don't know Rust, or what I'm doing. Perhaps this year I'll make it past day 12.

## Dev Log

### Day 1 - Part 1

So part one is simple: you get a list of numbers. Two numbers in the list add up to 2020. Find those numbers and multiply them.

So the obvious answer is to loop through the list, then in each iteration, loop through the list and add the two numbers. If they add upto 2020, you've got your answer. But then, you don't need to add anything to know what the other number is. You can loop through the list, subtract the number you're looking at from 2020, and then checking to see if that answer exists elsewhere in the list. Any time I'm checking if something exists in a list, I consider throwing it in a set for quick lookups.

So that's what I do. Read all lines into a `HashSet`, then iterate the `HashSet` checking to see if `(2020 -  that number)` exists in the set. If I find one (and I assume I do), then return `(that number * (2020 - that number))`.

Challenges: I still don't have an intuitive understanding of borrowing in Rust. Luckly, I have the very efficient and intelligent strategy of "try random things until the compiler stops complaining".

Potential edge case: If the input contains only a single `1010`, that can be added to itself, and the "contains" lookup doesn't care if it's looking up the same number. So, the algorithm would incorrectly return `1010 * 1010`, when only one `1010` exists in the list.

