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

## Part Two

You're sure that's the right password, but the door won't open. You knock, but nobody answers. You build a snowman while you think.

As you're rolling the snowballs for your snowman, you find another security document that must have fallen into the snow:

> "Due to newer security protocols, please use password method 0x434C49434B until further notice."

You remember from the training seminar that **method 0x434C49434B** means you're actually supposed to count the number of times any click causes the dial to point at **0**, regardless of whether it happens during a rotation or at the end of one.

Following the same rotations as in the above example, the dial points at zero a few extra times during its rotations:

- The dial starts by pointing at 50.
- The dial is rotated L68 to point at 82; during this rotation, it points at 0 once.
- The dial is rotated L30 to point at 52.
- The dial is rotated R48 to point at 0.
- The dial is rotated L5 to point at 95.
- The dial is rotated R60 to point at 55; during this rotation, it points at 0 once.
- The dial is rotated L55 to point at 0.
- The dial is rotated L1 to point at 99.
- The dial is rotated L99 to point at 0.
- The dial is rotated R14 to point at 14.
- The dial is rotated L82 to point at 32; during this rotation, it points at 0 once.

In this example, the dial points at 0 three times at the end of a rotation, plus three more times during a rotation. So, in this example, the new password would be **6**.

**Be careful:** if the dial were pointing at 50, a single rotation like R1000 would cause the dial to point at 0 ten times before returning back to 50!

Using password method 0x434C49434B, what is the password to open the door?
