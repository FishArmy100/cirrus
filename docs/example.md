# Code Examples
### Tick Tack Toe
```rs
pub enum Option[T]
{
    Some(T),
    None,
}

pub enum TileState : u32
{
    X,
    O,
    Empty,
}

pub enum Player : u32
{
    X,
    O
}

impl Player
{
    pub fn next(self) -> Player
    {
        match self
        {
            Player.X => Player.O,
            Player.O => Player.X
        }
    }
}

pub struct Board
{
    values: [][]TileState = [
        [TileState.Empty, TileState.Empty, TileState.Empty],
        [TileState.Empty, TileState.Empty, TileState.Empty],
        [TileState.Empty, TileState.Empty, TileState.Empty],
    ];
}

impl Board
{
    pub fn new() -> Self
    {
        Self {}
        
    }

    pub fn set(self, x: u32, y: u32, state: TileState)
    {
        values[x][y] = state;
    }

    pub fn get(self, x: u32, y: u32) -> TileState
    {
        values[x][y]
    }

    pub fn check_win(self) -> Option[Player]
    {
        if self.check_all(TileState.X)
        {
            Option.Some(Player.X)
        }
        else if self.check_all(TileState.O)
        {
            Option.Some(Player.O)
        }
        else 
        {
            Option.None
        }
    }

    fn check_all(self, state: TileState) -> bool
    {
        self.check_row(0, state) || 
        self.check_row(1, state) || 
        self.check_row(2, state) || 

        self.check_column(0, state) || 
        self.check_column(1, state) || 
        self.check_column(2, state) || 

        self.check_diagonals(state)
    }

    fn check_row(self, x: u32, state: TileState) -> bool
    {
        self.values[x][0] == state &&
        self.values[x][1] == state &&
        self.values[x][2] == state
    }

    fn check_column(self, y: u32, state: TileState) -> bool
    {
        self.values[0][y] == state &&
        self.values[1][y] == state &&
        self.values[2][y] == state
    }

    fn check_diagonals(self, state: TileState) -> bool
    {
        self.values[0][0] == state &&
        self.values[1][1] == state &&
        self.values[2][2] == state
        ||
        self.values[2][0] == state &&
        self.values[1][1] == state &&
        self.values[0][2] == state
    }
}

fn get_input(board: Board, player: Player)
{
    let state = match player {
        Player.X => TileState.X,
        Player.O => TileState.O
    };

    while true
    {
        match Console.ReadKey()
        {
            '1' => board.set(0, 0, state),
            '2' => board.set(1, 0, state),
            '3' => board.set(2, 0, state),
            '4' => board.set(0, 1, state),
            '5' => board.set(1, 1, state),
            '6' => board.set(2, 1, state),
            '7' => board.set(0, 2, state),
            '8' => board.set(1, 2, state),
            '9' => board.set(2, 2, state),
            _ => continue;
        }

        break;
    }
}

fn main()
{
    let board = Board.new();
    let player = Player.X;

    let game_state = board.check_win();
    while game_state.is_none()
    {
        get_input(board, player);
        player.next();
    }
}
```

### Fibonacci Example:
```rs
use Option.*;
pub enum Option<T>
{
    Some(T),
    None
}

pub type Iter<T> = fn() -> Option<T>;

pub interface Iterable<T>
{
    pub fn iter(self) -> Iter<T>;
}

impl<T> Iterable<T> for []T
{
    fn iter(self) -> Iter<T>
    {
        let mut i = 0;
        || => { 
            if i < self.length()
            {
                i += 1;
                (self[i - 1])
            }
            else 
            {
                None
            }
        }
    }
}

let items: [Int] = [4, 5, 6, 7];

fn main()
{
    let iter = items.iter();
    while let Some(item) = iter()
    {
        println(item.to_string());
    }
}
```