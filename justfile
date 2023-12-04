new-puzzle year day:
  mkdir -p puzzles/{{year}}/{{day}}
  rm puzzles/{{year}}/{{day}}/input puzzles/{{year}}/{{day}}/puzzle.md
  (cd puzzles/{{year}}/{{day}} && aoc download -y {{year}} -d {{day}})
