proc factorial do
    if n = 0 then
        set return -> 1
    else
        if @can_work then
            work
        end

        set n -> n - 1
        call factorial
        set n -> n + 1
        set return -> n * return
    end
end

loan @balance * 5

set n -> 6
call factorial
print return // this prints 720
