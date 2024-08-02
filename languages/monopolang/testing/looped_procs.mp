proc square do
    set x -> x * x
end

set x -> 2

range i from 0 to 5 do
    call square
    print x
end
