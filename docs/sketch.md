# Sketch

```rs
interface Add[R]
{
    fn add(self, rhs: Self) -> R;
}

struct Pair[A, B]
{
    pub a: A,
    pub b: B,
}

// adding the members of two pairs together
impl[A, B, R1, R2] Add[Pair[R1, R2]] for Pair[A, B]
    where A : Add[R1], 
          B : Add[R2], 
{
    fn add(self, rhs: Self) -> Pair[R1, R2]
    {
        Pair
        {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

let p = Pair { a: 1, b: 0.5 } + Pair { a: 2, b: 1.5 };
```