let file = JSON.parse(require("fs").readFileSync(0));

let initial = file[0];
let moves = file.slice(1).reduce(
  ([current, moves], val) => {
    for (i in val.blocks) {
      let current_block = current.blocks[i];
      let new_block = val.blocks[i];

      if (
        new_block.position[0] != current_block.position[0] ||
        new_block.position[1] != current_block.position[1]
      ) {
        moves.push([+i, new_block.position]);
        break;
      }
    }

    return [val, moves];
  },
  [initial, []]
)[1];

let out = {
  initial,
  moves
};

console.log(JSON.stringify(out));
