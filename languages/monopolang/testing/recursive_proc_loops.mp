proc recurse do
    set n -> n + 1

    if n < 10 then
        range i from 0 to 3 do
            call recurse
        end
    end
end

set n -> 0

while n < 5 do
    call recurse
end
