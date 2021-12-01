
# Input parsing

depths = []
with open('01.txt', 'r', encoding='utf8') as file:
  for line in file:
    depths.append(int(line))

# Part 1

depthIncreases = 0
prev = 0

for depth in depths:
  if prev != 0 and prev < depth:
    depthIncreases += 1
  prev = depth

print('Part 1:', depthIncreases)


# Part 2

groupDepthIncreases = 0
prev = 0

for group in zip(depths[:-2], depths[1:-1], depths[2:]):
  groupDepth = sum(group)
  if prev != 0 and prev < groupDepth:
    groupDepthIncreases += 1
  prev = groupDepth

print('Part 2:', groupDepthIncreases)
