def count_valid_lines(lines, func)
  count = 0
  lines.each{|line| count += 1 if method(func).call(line)}
  puts count
end

def parse_line(line)
  range, letter, password = line.split(" ")
  fst, scn = range.split("-").map(&:to_i)
  letter = letter[0]
  [fst, scn, letter, password]
end

def is_line_valid_part1?(line)
  min_range, max_range, letter, password = parse_line(line)
  num_occur = password.count(letter)
  num_occur >= min_range && num_occur <= max_range
end

def is_line_valid_part2?(line)
  fst, scn, letter, password = parse_line(line)
  (password[fst-1] == letter) ^ (password[scn-1] == letter)
end

# ------

def part1(lines)
  count_valid_lines(lines, :is_line_valid_part1?)
end

def part2(lines)
  count_valid_lines(lines, :is_line_valid_part2?)
end

lines = File.readlines("./input.txt")
part1 lines
part2 lines



