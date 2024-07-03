loan @balance * 5

// Attempt to work for 300 iterations
range _ from 0 to 300 do
	set initial -> @balance

    if @can_work then
        work
        set new -> @balance
        print "Worked for $" + (new - initial)
    end
end

// Buy as much $HACK as possible with 1/2 of balance
set amount -> @balance / (2 * $HACK)
buy "HACK" amount

print "Bought " + amount + " of $HACK for $" + $HACK + " each"
print "Current balance: " + @balance

sell "HACK" (amount / 2)

print "Sold half of $HACK shares for $" + $HACK + " each"
print "Current balance: " + @balance
