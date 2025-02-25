const SIZE: usize = 10;

#[derive(Debug)]
struct List<T>
where
    T: Default + std::fmt::Debug + Copy,
{
    head: usize,
    z: usize,
    x: usize,

    key: [T; SIZE],
    next: [usize; SIZE],
}

impl<T: Default + std::fmt::Debug + Copy> List<T> {
    pub fn new() -> Self {
        let head = 0;
        let z = 1;
        let x = 2;
        // The Copy Trait is being used here because the default value is being
        // copied for each place holder
        let key = [T::default(); SIZE];
        let mut next = [0; SIZE];

        next[head] = z as usize;
        next[z] = z as usize;

        Self {
            head,
            z,
            x,
            key,
            next,
        }
    }

    pub fn delete_next(&mut self, t: usize) {
        self.next[t] = self.next[self.next[t]]
    }

    pub fn insert_after(&mut self, v: T, t: usize) -> usize {
        self.key[self.x] = v;
        self.next[self.x] = self.next[t];
        self.next[t] = self.x;
        self.x += 1;
        self.x - 1
    }

    pub fn print(&self) {
        let mut n = self.next[0];

        while n != self.z {
            println!("{:?}", self.key[n]);
            n = self.next[n];
        }
    }
}

fn main() {
    let mut list: List<i32> = List::new();
    let mut next = list.insert_after(5, 0);
    for x in [10, 15, 20] {
        next = list.insert_after(x, next);
        println!("{:?} -- next: {next}", list);
    }
    list.insert_after(100, next);
    println!("{:?}", list);

    list.print();

    list.delete_next(4);
    println!("{:?}", list);
    list.print();
}
