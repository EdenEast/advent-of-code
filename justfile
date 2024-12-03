puzzle day year="2024":
  #!/usr/bin/env bash
  mkdir -p data/{{year}}/{{day}} && cd data/{{year}}/{{day}}
  aoc download -o -y {{year}} -d {{day}}

