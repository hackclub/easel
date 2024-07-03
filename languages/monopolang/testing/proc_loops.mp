proc do_something do
    set n -> n * 2

    range i from 0 to 10 do
        set n -> n + n
        print n
    end
end

set n -> 1
call do_something
print n
