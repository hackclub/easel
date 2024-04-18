Write a programming language, get a terminal-in-a-box to run your programming language on!

Check out [#langjam](https://app.slack.com/client/T0266FRGM/C06T22ZFQGP) on the Slack to hang out with fellow teenagers hacking on programming languages.

## Step one: learn how to write a programming language (or jump to step two!)

*Orpheus' Hacky Guide to Writing a Programming Language*, complete with a silly plot and silly illustrations.

Divided into five parts, JavaScript-based. Can be done entirely in-browser: every code snippet comes with an interactive playground.

* Orpheus finds an easel in the mail
  * Orpheus receives a package in the mail one day. It comes with a lil' computer, a painting easel of sorts, and a letter. &rarr; She reads the letter, and it tells her to take a closer look at the easel, so she pulls up a microscope and takes a look at the easel. There's tons of little cells! She reads the rest of the letter. It tells her she can become an artist with this easel and the computer, but it's a special kind of easel: she needs to create a language to tell it what colors to show and where.
  * Take a look at Easel, the programming language we're going to write. It'll have: variables, comments, for loops, while loops, functions, if-else statements, a few special boolean operators (`&&` AND, `!` NOT, `||` OR reminiscent of JS), a few special data types (arrays), and a very basic struct-based OOP implementation. The special thing about Easel is that it lets us draw on Orpheus' easel (which is an actual easel with `<canvas>`), with the help of two pre-defined structs (`Color` and `Canvas`, predefined because they're not relevant to learning how to write a programming language. May be added as a bonus/appendix.)
* Part One: Orpheus writes a lexer
  * About half an hour to go through. Orpheus takes the first program in the letter and learns how to break it down into tokens by writing a lexer. 
* Part Two: Orpheus writes a parser
  * About an hour and a half to go through. Orpheus takes the tokens she's gotten from part one and creates a abstract syntax tree from them. She's going to do this by writing a parser to describe what different parts of a program might look like as a node in a tree.
* Part Three: Orpheus writes an interpreter and gets the easel working!
  * About an hour to go through. Orpheus finally gets the easel working!!! She writes an interpreter to take the AST she's generated and go through and run every node, while keeping in track memory being used by the programming language.
* Orpheus decodes the letter
  * Now that it's working, can Orpheus run the second program and figure out what exactly it does? 
  * Other resources and a CTA to step two

Total amount of time for reading should be anywhere from three to five hours.

### Easel

Easel is the programming language you'll get to write! Here's the first program that Orpheus will run once she implements Easel:

```
prepare rows as 50
prepare cols as 20

brush Cell has { x, y, live }

~ Exercise: try setting a custom pattern instead of randomness!
sketch seed {
  prepare cells []
  loop x through (0, rows) {
    loop y through (0, cols) {

    }
  }
}

begin painting
  ~ This loop runs every iteration and must be in every program

end painting
```

(Conway's Game of Life)

## Step two: now write your own!

Criteria for a valid programming language:

* Minimally, should have the following features: variables, looping (think: for/while loops), conditional branching (think: if/else statements) and some form of recursion (think: functions). **Why? These are what make a programming language [Turing-complete](https://stackoverflow.com/questions/7284/what-is-turing-complete)**.
* Bonus points for creativity! Orpheus and her guide should have taught you that this is an ✨ art ✨. 
* Bonus points too if you explain your implementation (especially if you did something different compared to the guide, other Hack Clubbers would love to learn!) 

The gist: write a programming language &rarr; record a demo video with [Asciinema](https://asciinema.org/) and write a quick guide on how to get up and running with your programming language (including build instructions, if any!) &rarr; open a PR with these three things and your name on [Slack](https://hackclub.com/slack).

For a while, we'll also run a Hack Club Programming Lang Jam: until [date] top X favorite programming languages as voted on by the community will get a copy of *Crafting Interpreters*!

You can also (and should) write a programming language with other Hack Clubbers! Same criteria applies. 

## Step three: get your terminal-in-a-box in the mail!

When we receive your PR and merge it, we'll send you a form to fill out asking for your address, etc.! 

And then we'll ship you the same package we shipped to Orpheus in a guide: a box, a nice letter, and an SD card (or some media device). Your box will be pretty small, it'll look like this:

![](https://hackclub.com/stickers/macintosh.svg)

You insert the flash drive into the box, connect the box to an outlet, click the bootup button, and then connect it to a monitor and a keyboard, and boom! a terminal opens. you can write code for your programming language in this terminal.

The coolest thing is that you can run other people's programming languages on your terminal-in-a-box too, by replacing the contents of your flash drive.

### How it works

Hardware wise, an Raspberry Pi Zero 2W which has HDMI out. Outer shell styled similarly to the Hack Club sticker above, most likely 3D printed, holes for HDMI out, power out, and keyboard out.

Software wise, every flash drive will come flashed with a [TODO]

@Shawn and @Thomas helped significantly with this.

### Cost wise

Looking at a hard limit of $40 - $45 based on time it would take to write a programming language. Currently solidly under limit.

* $15 for [Pi](https://www.adafruit.com/product/5291?gad_source=1)
* Shell can be 3D printed - recall seeing a replication of the sticker somewhere around the office - like that, but thicker
* Extra wires - HDMI out ($5.95), power cables that come with Sprig

While @Shawn was here, we also considered doing something like [this](https://github.com/ncrawforth/VT2040) but it requires more consideration cost-wise.
