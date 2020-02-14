#[derive(Debug)]
struct Intcode {
    store: Vec<i64>,
    index: usize,
    offset: usize,
    modes: [usize; 3],
}

impl Intcode {
    fn add(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 4));
        let poc = self.store[self.index + 3] as usize;
        let poa = self.store[self.index + 1] as usize;
        let pob = self.store[self.index + 2] as usize;
        self.store[match self.modes[2] {
            0 => poc,
            1 => panic!("OH NO"),
            2 => poc + self.offset,
            _ => panic!("oh no"),
        }] = self.store[match self.modes[0] {
            0 => poa,
            1 => self.index + 1,
            2 => poa + self.offset,
            _ => panic!("OH LORD"),
        }] + self.store[match self.modes[1] {
            0 => pob,
            1 => self.index + 2,
            2 => pob + self.offset,
            _ => panic!("OH LORD"),
        }];
        self.index += 4;
    }
    fn mul(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 4));
        let poc = self.store[self.index + 3] as usize;
        let poa = self.store[self.index + 1] as usize;
        let pob = self.store[self.index + 2] as usize;
        self.store[match self.modes[2] {
            0 => poc,
            2 => poc + self.offset,
            _ => panic!("oh no"),
        }] = self.store[match self.modes[0] {
            0 => poa,
            1 => self.index + 1,
            2 => poa + self.offset,
            _ => panic!("OH LORD"),
        }] * self.store[match self.modes[1] {
            0 => pob,
            1 => self.index + 2,
            2 => pob + self.offset,
            _ => panic!("OH LORD"),
        }];
        self.index += 4;
    }
    fn out(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 2));
        println!(
            "VALUE: {}\n",
            self.store[match self.modes[0] {
                0 => self.store[self.index + 1] as usize,
                1 => self.index + 1,
                2 => self.store[self.index + 1] as usize + self.offset,
                _ => panic!("OH LORD"),
            }]
        );
        self.index += 2;
    }
    fn jumpa(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 3));
        let poa = self.store[self.index + 1] as usize;
        let pob = self.store[self.index + 2] as usize;
        let temp = self.store[match self.modes[1] {
            0 => pob,
            1 => self.index + 2,
            2 => pob + self.offset,
            _ => panic!("NO NO NO"),
        }] as usize;
        println!("{}, {}, {}", poa, self.index, self.store[self.index + 1]);
        if self.store[match self.modes[0] {
            0 => poa,
            1 => self.index + 1,
            2 => poa + self.offset,
            _ => panic!("NO Nnnn NO"),
        }] != 0
        {
            self.index = temp;
        } else {
            self.index += 3;
        }
    }
    fn jumpb(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 3));
        let poa = self.store[self.index + 1] as usize;
        let pob = self.store[self.index + 2] as usize;
        let temp = self.store[match self.modes[1] {
            0 => pob,
            1 => self.index + 2,
            2 => pob + self.offset,
            _ => panic!("NO NO NO"),
        }] as usize;
        println!("{}, {}, {}", poa, self.index, self.store[self.index + 1]);
        if self.store[match self.modes[0] {
            0 => poa,
            1 => self.index + 1,
            2 => poa + self.offset,
            _ => panic!("NO Nnnn NO"),
        }] == 0
        {
            self.index = temp;
        } else {
            self.index += 3;
        }
    }
    fn less(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 4));
        let poa = self.store[self.index + 1] as usize;
        let pob = self.store[self.index + 2] as usize;
        let poc = self.store[self.index + 3] as usize;
        let index = match self.modes[2] {
            0 => poc,
            2 => poc + self.offset,
            _ => panic!("NO)))"),
        } as usize;
        self.store[index] = if self.store[match self.modes[0] {
            0 => poa,
            1 => self.index + 1,
            2 => poa + self.offset,
            _ => panic!("NO Nnnn NO"),
        }] < self.store[match self.modes[1] {
            0 => pob,
            1 => self.index + 2,
            2 => pob + self.offset,
            _ => panic!("NO Nnnn NO"),
        }] {
            1
        } else {
            0
        };
        if index != self.index {
            self.index += 4;
        }
    }
    fn eq(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 4));
        let poa = self.store[self.index + 1] as usize; // 4
        let pob = self.store[self.index + 2] as usize; // 4
        let poc = self.store[self.index + 3] as usize; // 1
        let index = match self.modes[2] {
            0 => poc, // index = 4
            2 => poc + self.offset,
            _ => panic!("NO)((()))))"),
        } as usize;
        self.store[index] = if self.store[match self.modes[0] {
            0 => poa,
            1 => self.index + 1,
            2 => poa + self.offset,
            _ => panic!("NNnnn NO"),
        }] == self.store[match self.modes[1] {
            0 => pob,
            1 => self.index + 2,
            2 => pob + self.offset,
            _ => panic!("NO Nn NO"),
        }] {
            1
        } else {
            0
        };
        if index != self.index {
            self.index += 4;
        }
    }
    fn adj(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 2));
        let poa = self.store[self.index + 1];
        self.offset = (self.offset as i64
            + self.store[match self.modes[0] {
                0 => poa as usize,
                1 => self.index + 1,
                2 => ((poa + self.offset as i64) as usize),
                _ => panic!("At the disco"),
            }]) as usize;
        self.index += 2;
    }
    fn input(&mut self) {
        println!("{:?}", self.store.get(self.index..self.index + 2));
        let poa = self.store[self.index + 1] as usize;
        self.store[match self.modes[0] {
            0 => poa,
            1 => self.index + 1,
            2 => poa + self.offset,
            _ => panic!("Not at the disco"),
        }] = 8;
        self.index += 2;
    }
}

impl Iterator for Intcode {
    type Item = Option<i64>;
    fn next(&mut self) -> Option<Option<i64>> {
        self.modes[2] = self.store[self.index] as usize / 10000;
        self.modes[1] = self.store[self.index] as usize / 1000 - 10 * self.modes[2];
        self.modes[0] = self.store[self.index] as usize / 100 - 10 * self.modes[1];
        match self.store[self.index] % 100 {
            1 => self.add(),
            2 => self.mul(),
            3 => self.input(),
            4 => self.out(),
            5 => self.jumpa(),
            6 => self.jumpb(),
            7 => self.less(),
            8 => self.eq(),
            9 => self.adj(),
            99 => {
                println!("{:?}", self.store.get(self.index..self.index + 4));
                return None;
            }
            _ => {
                panic!("god damnit: {}\n{:?}", self.store[self.index], self);
            }
        }
        Some(Some(5))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut a = Intcode {
        store: vec![
            1, 0, 0, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 2, 6, 19, 23, 1, 23, 5,
            27, 1, 27, 13, 31, 2, 6, 31, 35, 1, 5, 35, 39, 1, 39, 10, 43, 2, 6, 43, 47, 1, 47, 5,
            51, 1, 51, 9, 55, 2, 55, 6, 59, 1, 59, 10, 63, 2, 63, 9, 67, 1, 67, 5, 71, 1, 71, 5,
            75, 2, 75, 6, 79, 1, 5, 79, 83, 1, 10, 83, 87, 2, 13, 87, 91, 1, 10, 91, 95, 2, 13, 95,
            99, 1, 99, 9, 103, 1, 5, 103, 107, 1, 107, 10, 111, 1, 111, 5, 115, 1, 115, 6, 119, 1,
            119, 10, 123, 1, 123, 10, 127, 2, 127, 13, 131, 1, 13, 131, 135, 1, 135, 10, 139, 2,
            139, 6, 143, 1, 143, 9, 147, 2, 147, 6, 151, 1, 5, 151, 155, 1, 9, 155, 159, 2, 159, 6,
            163, 1, 163, 2, 167, 1, 10, 167, 0, 99, 2, 14, 0, 0,
        ],
        index: 0,
        offset: 0,
        modes: [0, 0, 0],
    };

    println!("{:?}\n", a);
    while let Some(_) = a.next() {
        println!("{:?}\n", a);
    }
    Ok(())
}
