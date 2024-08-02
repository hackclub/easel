# Words

## Simple Stack Operations
`dup` duplicates the item on the top of the stack

`rotate` rotates n items on the stack

`swap` swaps the top two items on the stack

`over` duplicates the second to top item on the stack

`drop` removes the top stack item

## Math
`abs` returns the absolute value of the stacks top item

`random` gives a random number between a range

## IO
`read` reads a file (string item) to a string in the stack

`write` writes to a file a given string (file name must be pushed first)

`syscall` runs a syscall based on the top stack number

`print` prints the top string on the stack

`rawmode` allows the terminal to enter rawmode, useful for games

`sleep` sleeps for n milliseconds

## List Control
`concan` combines two stacks into one

`flatten` flattens any children's stacks into the current one

`map` runs an expression for each item of a stack

`filter` filters items in the stack depending on an epression

`shear` removes a stacks children

`empty` removes this stacks children

`range` creates a range between a min and max

`size` returns the size of the current stack

`left` returns the amount left on the stack until you cannot pop

`eval` runs a string as an expression

## Control Flow
`if { expr } else { expr }`

`while { expr }`

if, else and while work off the stack value being "truthy", or not 0

## Functions 
`fn function_name { expr }`

allows for creating new words/functions by combining a list of them, essentially the same as `"expr" eval`
