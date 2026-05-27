# Inumaki

Programming language made for [Hack Club's langjam](https://langjam.hackclub.com) based off the speech of the Jujutsu Kaisen character [Toge Inumaki](https://jujutsu-kaisen.fandom.com/wiki/Toge_Inumaki). Though the original is of course Japanese, I'll be using the English translations of his vocabulary for the sake of my own sanity.

# Vocabulary
These are the phrases which will be used in place of normal keywords in this language. Not all of them will be used and I'll try to use them in line with their original meaning to have some sense come out of all this.

## Safe Words 
| Word | Original use | Language use |
| --- | ---| --- |
| Salmon | Affirmation | True (Boolean) | 
| Bonito Flakes | Negation | False (Boolean) |
| Kelp | Greeting | Comments |
| Mustard Leaf | Concern/Worry | Conditionals |
| Salmon Roe | "Well, well" | |
| Caviar | Expletive | Exceptions |
| Spicy Cod Roe | Motivational | |
| Tuna | Call attention | Print |
| Tuna Mayo | Do something | Function definitions |

## Cursed Speech
| Word | Language use|
| --- | --- |
| Explode | Else |
| Twist | For loops |
| Crush | |
| Plummet | While loops |
| Stop | |
| Sleep | |
| Return | Return from function |
| Run | |
| Blast | |

# Syntax Examples

## Comments
```
Kelp <This is a comment>
```

## Variables
```
Tuna <variableName> Tuna <value>
```

## Conditionals
```
Mustard_Leaf Tuna <condition> Tuna {
    <main body>
} Explode {
    <else body>
}
```
There are no elifs so multiple cascading Explode..Mustard_Lead statements are required in that case.

## For loops
```
Twist Tuna <iteration var> Tuna <inital value> Tuna <stopping condition> Tuna <increment statement> Tuna {
    <main body>
}
```
The increment statement usually takes a structure like `Tuna <iteration var> Tuna <iteration var> + 1`

## While loops
```
Plummet Tuna <condition> Tuna {
    <main body>
}
```
The `Stop` cursed keyword may be used in future to break out of loops but until that is implemented, the condition must be made false to exit the loop.

## Function definitions
```
Tuna_Mayo <function name> Tuna <parameters space-separated> Tuna {
    <main body>
    Return <space separated values>
}
```
Parameters are optional in which case there will be `Tuna Tuna`. Return is also optional.

# Standard Library
Tuna_Tuna for print, str and float all directly map to the python builtin functions.

# Note on flexible keywords
In order to allow the programming language to be more chaotic and greater resembling Inumaki's speech keywords which aren't crucial to telling the parser what the current statement is do not have to be the same. For example, in a while loop

```
Plummet Tuna <condition> Tuna {
    <main body>
}
```

Once the parse sees `Plummet` it knows it's a while loop and I just need the `Tuna` to sandwich the condition as an easy way of knowing when to stop eating the condition. Therefore, in most of the places that `Tuna` has been used in the above examples, any other keyword can be used with no functional difference apart from cursedness. The exception is the assignment statement where `Tuna` is the primary indicator.

# Cursedness
All token is this language are either cursed or not depending on wether they are one of the words in the cursed table above. Any use of these words wether necessarily in a code statement or even in a string literal will increase the interpreter's internal cursed counter. Over use of cursed speech will result in throat irritation and may cause sever damage to Inumaki's throat (an exception will be thrown). To prevent this, `Cough_Syrup` should be administered at appropriate intervals to alleviate symptoms.