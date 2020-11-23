pub struct Dice {
    count: usize,
    faces: usize,
}

impl Dice {
    pub fn new<S>(s: S) -> Option<Self>
    where
        S: std::ops::Deref<Target=str>
    {
        let ops: Vec<&str> = s.split('d').collect();

        if ops.len() == 2 {
            let res = Self {
                count: ops[0].parse::<usize>().ok()?,
                faces: ops[1].parse::<usize>().ok()?,
            };

            if res.faces < 2 {
                None
            } else {
                Some(res)
            }
        } else {
            None
        }
    }

    pub fn roll(&self) -> Vec<usize> {
        let mut v = Vec::new();

        for _ in 0..self.count {
            v.push((rand::random::<usize>() % self.faces) + 1);
        }

        v
    }
}
