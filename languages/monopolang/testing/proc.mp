proc hello do
    print "Hello, " + name + "!"
end

set name -> "world"
call hello

proc square do
    set return -> x * x
end

set x -> 5
call square
print return
