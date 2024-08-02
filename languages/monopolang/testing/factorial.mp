proc factorial do
    if n = 0 then
        set return -> 1
    else
        set n -> n - 1
        call factorial
        set n -> n + 1
        set return -> n * return
    end
end

set n -> 20
call factorial
print return // this prints 120
