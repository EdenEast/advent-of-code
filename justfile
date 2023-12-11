new-puzzle year day:
  mkdir -p puzzles/{{year}}/{{day}}
  (cd puzzles/{{year}}/{{day}} && aoc download -o -y {{year}} -d {{day}})
