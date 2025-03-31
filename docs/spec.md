# Lang Name TBD
## Brief Description:
A typed multi-paradigm scripting language, that is easy to incorporate with a smallish compiler, and is embeddable into multiple different applications.

## Syntax Documentation
### Variables
```rs
let a: Int = 1; // immutable variable, type optional
var b: Int = 2; // mutable variable, type optional
b += 1;
const C: Int = 7; // compile time constant, type required
```

### Control Flow
Most control flow is expression based
```rs
let x = 1;
let a = if x == 1
{
	0
}
else 
{
	1
}

let b = match a // pattern matching
{
	1 => 2,
	_ => 0,
}

for i in range(0, 5)
{
	println("At index: ${i}"); // string interpolation
	if i.is_even()
	{
		break; // multiple breaks allowed
	}

	if i.is_odd()
	{
		continue; // multiple continues allowed
	}
}

while true
{
	// ...
}

// Other forms of pattern matching
if let Some(x) = some_function()
{
	// ...
}

while let Some(x) = some_function()
{
	// ...
}
```

### Functions
```rs
fn add(a: Int, b: Int): Int
{
	a + b
}

fn square[T](a: T): T 
	where T : Mul
{
	a * a
}
```

### Structures
```rs
struct Player
{
	pub name: String, // readonly
	pub var health: Float = 100.0, // mutable with a default argument
}

impl Player
{
	pub fn new(name: String): Self
	{
		Self
		{
			name,
		}
	}
}

let player = Player.new("Bob");

struct Pair[T]
{
	left: T,
	right: T,
}

impl[T] Pair[T]
{
	fn new(a: T, b: T): Self
	{
		Self
		{
			left: a,
			right: b
		}
	}
}

impl[T] IClone for Pair[T] // generic impls
	where T : IClone // trait/interface bounds
{
	fn clone(self): Self
	{
		Self
		{
			left: self.left.clone(),
			right: self.right.clone(),
		}
	}
}
```

### Enums

### Interfaces/Traits

### Any

### Const evaluation

## Example Program
```rs

pub struct Task
{
    pub name: String,
    pub description: String,
    pub id: Int,
}

pub struct TaskList
{
	list: List[Task] = List.new(),
}

impl TaskList
{
	pub fn new() -> Self
	{
		Self {}
	}

	pub fn add_task(self, name: String, description: String) -> Int
	{
		let task = Task {
			name,
			description,
			id: self.list.length(),
		}

		self.list.add(task);
		task.id
	}

	pub fn remove_task(self, id: Int) -> Bool
	{
		let old = self.list.length();
		list.iter().retain(|task| task.id != id);
		old != self.list.length()
	}

	pub fn get_tasks(self) -> List<Task>
	{
		self.tasks
	}
}

fn main()
{
	let list = TaskList.new();
	let t = list.add_task("Take out trash", "We need to take out the trash");
	list.add_task("Make dinner", "Mac n Cheese + Peas?");
	list.remove_task(t);

	for i in list.get_tasks()
	{
		println("Task: ${i.name}");
	}
}
```