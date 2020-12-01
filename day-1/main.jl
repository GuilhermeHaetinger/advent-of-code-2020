nums = (x -> parse(Int64, x)).(readlines("input.txt"))

# Part 1
completions = (x -> 2020 - x).(nums)
found_map = Dict()

for i in 1:length(nums)
    if haskey(found_map, completions[i])
        println(completions[i] * nums[i]) 
        break
    end
    found_map[nums[i]] = 1
end

# Part 2
for i in 1:length(nums)
    completions_tri = (x -> completions[i] - x).(nums)
    found_map_tri = Dict()

    for j in 1:length(nums)
        if i == j; continue; end;
        if haskey(found_map_tri, completions_tri[j])
            println(completions_tri[j] * nums[j] * nums[i]) 
            nums = []; break;
        end
        found_map_tri[nums[j]] = 1
    end
end
