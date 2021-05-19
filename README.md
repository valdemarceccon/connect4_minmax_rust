### Connect 4 AI using MinMax with AlphaBeta pruning
This project is being done as learning as a way to learn rust.

### Goals
#### Game and AI
In the end I would expect to have a module to deal with board logic, game rules
logic, AI logic and one binary as CLI user interface. I am planning to create a
GUI as well, but it is not a sure thing for now.

There is some other `utils` modules. One that I've already done is used by the
game rules module, as validator of the board state, to check if the game has
ended, and if there is a winner or the game has tied. That seems to be game logic
responsibility and may be moved to game rules modules in the future.
Since the game rules modules was getting too big, I decided to break it into smaller ones.

- [x] Board Logic
  - [x] Dynamic board size
  - [x] Moves done by column
  - [x] `Enums` to each player
  - [x] Custom error for when a move is tried on a full column
  - [x] Custom error for `out of bounds` move
- [x] Game Rules Logic
  - [x] Toggle players when a successful move is made
  - [x] Check if the game has ended
    - [x] Check if there is a winner or the game has tied
  - [ ] Custom rules
    - [ ] Dynamic number of pieces in a row for the game to end
- [ ] AI
  - [ ] MinMax algorithm
  - [ ] AlphaBeta pruning
- [x] User interface
  - [x] Simple CLI
    - [x] Draw board
    - [x] Color players
  - [ ] Option\<GUI>
- [ ] Code documentation

#### CI
I am planning on creating a continous integration process. Since there are `unit` tests,
it can be useful to keep unproven experimental code away from `main` branch.

#### Documentation
For now, this README is the only documentation. However this should change in the next
couple of commits.

### Breakdown
#### Board
The board module is going to be responsible to validate and maintain a healthy state.
It won't allow for arbitrary piece placement. A move is only allowed through
the `play` method.

The `play` method receives a column, and places it on the correct row. If the row
is full, an error is returned using the `Result` enum.

If a move is done in a column greater than the board size nothing happens, but `Ok`
is returned for now. This should soon change to returning a corresponding `Err`. As things
are now, there is no way for the game logic know that the move wasn't actually done without
checking the board itself.

#### Game rules
The game module is the one responsible for driving the game. It receives the move
and passes it to the board. In case of a valid move, a check for end of the game is done.
If the game is finished, a `Result` is returned with the winner, if there is one.

For the end check, first a check is done in each row, column and diagonal, looking
for 4 pieces of a single player. If there are none, then is checked if the board is
full and if so, no further move is valid, resulting in a tie.

For now the game is just like the vanilla one, finishing the game when 4 pieces are connected.
And there are still plans to allow custom rules.
