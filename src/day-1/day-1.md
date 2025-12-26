## Day 1: Secret Entrance

The Elves have good news and bad news.

The good news is that they've discovered project management! This has given them the tools they need to prevent their usual Christmas emergency. For example, they now know that the North Pole decorations need to be finished soon so that other critical tasks can start on time.

The bad news is that they've realized they have a different emergency: according to their resource planning, none of them have any time left to decorate the North Pole!

To save Christmas, the Elves need you to finish decorating the North Pole by **December 12th**.

### Puzzle Rules

Collect stars by solving puzzles. Two puzzles will be made available on each day; the second puzzle is unlocked when you complete the first. Each puzzle grants one star. Good luck!

### The Situation

You arrive at the secret entrance to the North Pole base ready to start decorating. Unfortunately, the password seems to have been changed, so you can't get in. A document taped to the wall helpfully explains:

> Due to new security protocols, the password is locked in the safe below.
> Please see the attached document for the new combination.

### The Safe

The safe has a dial with only an arrow on it. Around the dial are the numbers **0 through 99** in order. As you turn the dial, it makes a small click noise as it reaches each number.

The attached document (your puzzle input) contains a sequence of rotations, one per line, which tell you how to open the safe.

Each rotation consists of:

- A direction: **L** (left, toward lower numbers) or **R** (right, toward higher numbers)
- A distance value indicating how many clicks to rotate

#### Examples

- If the dial is pointing at **11**, a rotation of **R8** causes it to point at **19**.
- After that, a rotation of **L19** causes it to point at **0**.

Because the dial is circular:

- Turning left from **0** moves to **99**
- Turning right from **99** moves to **0**

So:

- From **5**, a rotation of **L10** points to **95**
- Then a rotation of **R5** points to **0**

The dial always starts pointing at **50**.

### The Twist

You could follow the instructions, but your recent required official North Pole secret entrance security training seminar taught you that the safe is actually a decoy.

The **actual password** is the number of times the dial is left pointing at **0** after any rotation in the sequence.

### Example Walkthrough

Suppose the attached document contained these rotations:

- L68
- L30
- R48
- L5
- R60
- L55
- L1
- L99
- R14
- L82

Following the rotations:

1. The dial starts at **50**
2. L68 → **82**
3. L30 → **52**
4. R48 → **0**
5. L5 → **95**
6. R60 → **55**
7. L55 → **0**
8. L1 → **99**
9. L99 → **0**
10. R14 → **14**
11. L82 → **32**

The dial points at **0** a total of **three times**, so the password in this example is **3**.

### Your Task

Analyze the rotations in your attached document.

**What is the actual password to open the door?**
