loan 1250

set looping -> true
while looping do
    print @balance + " : " + @debt

    if @can_work then
        work
    end

    if @balance > 100000 then
        set looping -> false
    end
end
