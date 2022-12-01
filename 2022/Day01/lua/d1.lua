local sums = {0}

for line in io.lines("../input") do
    if line ~= "" then
        sums[#sums] = sums[#sums] + tonumber(line)
    else
        table.insert(sums, 0)
    end
end

table.sort(sums)
print("part 1 " .. sums[#sums])
print("part 2 " .. sums[#sums] + sums[#sums-1] + sums[#sums-2])
