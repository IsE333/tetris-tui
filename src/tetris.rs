use rand::Rng;

pub enum Input {
    Left,
    Right,
    Down,
    RotateClockwise,
    RotateCounterClockwise,
}

pub struct GameLoop {
    counter: i32,
    arr: [[Option<i8>; 10]; 20],
    pub piece: Option<Piece>,
    pub map: [[Option<i8>; 10]; 20],
}

impl GameLoop {
    pub fn new() -> GameLoop {
        let piece = Some(Piece::new());

        GameLoop {
            counter: 0,
            arr: [[None; 10]; 20], //without the piece array
            piece,
            map: [[None; 10]; 20],
        }
    }

    pub fn tick(&mut self) {
        if self.counter % 30 == 0 {
            self.action();
            self.render_piece();
        }
        self.piece.as_mut().expect("").tick();
        self.counter += 1;
        return;
    }

    pub fn input(&mut self, key: Input) {
        let Some(piece) = &mut self.piece else {
            return;
        };
        match key {
            Input::Left => piece.movement(Input::Left, &self.arr),
            Input::Right => piece.movement(Input::Right, &self.arr),
            Input::Down => piece.down(&self.arr),
            Input::RotateClockwise => piece.rotate(true, &self.arr),
            Input::RotateCounterClockwise => piece.rotate(false, &self.arr),
        }
        self.render_piece();
    }
    fn render_piece(&mut self) {
        for i in 0..20 {
            for j in 0..10 {
                self.map[i][j] = self.arr[i][j];
            }
        }
        if let Some(piece) = &self.piece {
            for i in 0..4 {
                for j in 0..4 {
                    if piece.arr[i][j] != None {
                        self.map[(piece.offset[0] + i as i32) as usize][piece.offset_h(j)] =
                            piece.arr[i][j];
                    }
                }
            }
        }
    }

    pub fn action(&mut self) {
        let can_move = self.piece.as_mut().expect("No piece").move_down(&self.arr);
        if !can_move {
            self.piece.as_mut().expect("No piece").place(&mut self.arr);
            self.line_check();
            self.piece = Option::Some(Piece::new());
        }
    }

    fn line_check(&mut self) {
        let mut bool_arr: [bool; 20] = [false; 20];
        for i in 0..20 {
            let mut is_full = true;
            for j in 0..10 {
                if self.arr[i][j] == None {
                    is_full = false;
                    break;
                }
            }
            if is_full {
                bool_arr[i] = true;
            }
        }
        let mut count = 0;
        for i in (0..20).rev() {
            if bool_arr[i] {
                count += 1;
                continue;
            }
            if count == 0 {
                continue;
            }
            for j in 0..10 {
                if i < count {
                    self.arr[i][j] = None;
                    continue;
                }
                self.arr[i + count][j] = self.arr[i][j];
            }
        }
    }
}

pub struct Piece {
    rotate_count: u16,
    down_counter: i16,
    move_counter: i16,
    move_started: bool,
    move_continue: bool,
    direction: i32,
    centern_offset: [[i32; 2]; 4],
    centerp_offset: [[i32; 2]; 4],
    pub arr: [[Option<i8>; 4]; 4],
    pub offset: [i32; 2],
}

impl Piece {
    pub fn new() -> Piece {
        let p_t: i8 = rand::thread_rng().gen_range(0..7);
        let mut ar: [[Option<i8>; 4]; 4] = [[None; 4]; 4];
        let mut centern_offset: [[i32; 2]; 4] = [[0; 2]; 4];
        let mut centerp_offset: [[i32; 2]; 4] = [[0; 2]; 4];
        self::Piece::init(&mut ar, &mut centern_offset, &mut centerp_offset, p_t);

        Piece {
            //p_type: p_t,
            rotate_count: (u16::MAX - 7) / 2,
            arr: ar,
            centern_offset: centern_offset,
            centerp_offset: centerp_offset,
            offset: [0, 3],
            move_counter: -14,
            down_counter: -7,
            move_started: false,
            move_continue: false,
            direction: 0,
        }
    }
    pub fn tick(&mut self) {
        if !self.move_continue {
            self.move_started = false;
            return;
        }
        self.move_continue = false;
    }

    pub fn offset_h(&self, j: usize) -> usize {
        let h = (self.offset[1] + j as i32) as usize;
        return h;
    }

    pub fn offset_v(&self, i: usize) -> usize {
        let v = (self.offset[0] + i as i32) as usize;
        return v;
    }

    fn is_out_of_bounds(&self, i: usize, j: usize) -> bool {
        if (self.offset_h(j) as i32) < 0
            || self.offset_h(j) as i32 >= 10
            || self.offset_v(i) as i32 >= 20
        {
            return true;
        }

        return false;
    }

    pub fn down(&mut self, tmp_arr: &[[Option<i8>; 10]; 20]) {
        if self.down_counter < 7 {
            self.down_counter += 1;
            return;
        }
        self.down_counter = 0;
        for i in 0..4 {
            for j in 0..4 {
                if self.arr[i][j] == None {
                    continue;
                }
                if self.offset_v(i) as i32 + 1 >= 20 {
                    return;
                }
                let i = (self.offset_v(i) as i32 + 1) as usize;
                if tmp_arr[i][self.offset_h(j)] != None {
                    return;
                }
            }
        }
        self.offset[0] += 1;
    }

    pub fn move_down(&mut self, tmp_arr: &[[Option<i8>; 10]; 20]) -> bool {
        //check if the piece can move down
        for i in 0..4 {
            for j in 0..4 {
                if self.arr[i][j] == None {
                    continue;
                }
                if self.offset_v(i) as i32 + 1 >= 20 {
                    return false;
                }
                if tmp_arr[self.offset_v(i + 1)][self.offset_h(j)] != None {
                    return false;
                }
            }
        }
        self.offset[0] += 1; //move 1 down
        return true;
    }
    pub fn place(&mut self, tmp_arr: &mut [[Option<i8>; 10]; 20]) {
        // place piece
        for i in 0..4 {
            for j in 0..4 {
                if self.arr[i][j] != None {
                    tmp_arr[self.offset_v(i)][self.offset_h(j)] = self.arr[i][j];
                }
            }
        }
    }

    pub fn movement(&mut self, dir: Input, tmp_arr: &[[Option<i8>; 10]; 20]) {
        let direction;
        match dir {
            Input::Left => direction = -1,
            Input::Right => direction = 1,
            _ => return,
        }
        if direction != self.direction {
            self.move_started = false;
            self.direction = direction;
        }

        self.move_continue = true;
        if !self.move_started {
            self.move_counter = -14;
            self.move_started = true;
        }

        if self.move_counter == -14 {
            self.move_counter += 1;
        } else if self.move_counter < 7 {
            self.move_counter += 1;
            return;
        } else {
            self.move_counter = 0;
        }

        for i in 0..4 {
            for j in 0..4 {
                if self.arr[i][j] == None {
                    continue;
                }
                if self.offset_h(j) as i32 + direction < 0
                    || self.offset_h(j) as i32 + direction >= 10
                {
                    return;
                }
                if ({
                    let j = (self.offset_h(j) as i32 + direction) as usize;
                    tmp_arr[self.offset_v(i)][j]
                }) != None
                {
                    return;
                }
            }
        }

        self.offset[1] += direction;
    }

    pub fn rotate(&mut self, clock_wise: bool, tmp_arr: &[[Option<i8>; 10]; 20]) {
        let mut ar: [[Option<i8>; 4]; 4] = [[None; 4]; 4];

        if clock_wise {
            self.rotate_count += 1;
            for i in 0..4 {
                for j in 0..4 {
                    let x = (i as i32) + self.centerp_offset[(self.rotate_count % 4) as usize][0];

                    let y =
                        3 - (j as i32) + self.centerp_offset[(self.rotate_count % 4) as usize][1];

                    if x < 0 || x >= 4 || y < 0 || y >= 4 {
                        continue;
                    }
                    ar[i][j] = self.arr[y as usize][x as usize];
                    if ar[i][j] == None {
                        continue;
                    }
                    if self.is_out_of_bounds(i, j)
                        || tmp_arr[self.offset_v(i)][self.offset_h(j)] != None
                    {
                        self.rotate_count -= 1;
                        return;
                    }
                }
            }
        } else {
            self.rotate_count -= 1;
            for i in 0..4 {
                for j in 0..4 {
                    let x =
                        3 - (i as i32) + self.centern_offset[(self.rotate_count % 4) as usize][0];
                    let y = (j as i32) + self.centern_offset[(self.rotate_count % 4) as usize][1];
                    if x < 0 || x >= 4 || y < 0 || y >= 4 {
                        continue;
                    }
                    ar[i][j] = self.arr[y as usize][x as usize];
                    if ar[i][j] == None {
                        continue;
                    }
                    if self.is_out_of_bounds(i, j)
                        || tmp_arr[self.offset_v(i)][self.offset_h(j)] != None
                    {
                        self.rotate_count -= 1;
                        return;
                    }
                }
            }
        }
        self.arr = ar;
    }

    fn init(a: &mut [[Option<i8>; 4]; 4], cn: &mut [[i32; 2]; 4], cp: &mut [[i32; 2]; 4], p_t: i8) {
        if p_t == 0 {
            // I
            a[0][2] = Some(p_t);
            a[1][2] = Some(p_t);
            a[2][2] = Some(p_t);
            a[3][2] = Some(p_t);
            cp[0][1] = 1;
            cp[2][1] = 1;
            cn[1][0] = 1;
            cn[3][0] = 1;
        } else if p_t == 1 {
            // O
            a[1][1] = Some(p_t);
            a[1][2] = Some(p_t);
            a[2][1] = Some(p_t);
            a[2][2] = Some(p_t);
        } else if p_t == 2 {
            // T
            a[1][1] = Some(p_t);
            a[2][0] = Some(p_t);
            a[2][1] = Some(p_t);
            a[2][2] = Some(p_t);
            cp[0][0] = -1;
            cp[1][0] = -1;
            cp[2][0] = -1;
            cp[3][0] = -1;
            cn[0][1] = 1;
            cn[1][1] = 1;
            cn[2][1] = 1;
            cn[3][1] = 1;
        } else if p_t == 3 {
            // S
            a[1][1] = Some(p_t);
            a[1][2] = Some(p_t);
            a[2][0] = Some(p_t);
            a[2][1] = Some(p_t);
            cp[0][1] = -1;
            cp[2][1] = -1;
            cn[1][0] = -1;
            cn[3][0] = -1;
        } else if p_t == 4 {
            // Z
            a[1][0] = Some(p_t);
            a[1][1] = Some(p_t);
            a[2][1] = Some(p_t);
            a[2][2] = Some(p_t);
            cp[0][1] = -1;
            cp[2][1] = -1;
            cn[1][0] = -1;
            cn[3][0] = -1;
        } else if p_t == 5 {
            // L
            a[0][1] = Some(p_t);
            a[1][1] = Some(p_t);
            a[2][1] = Some(p_t);
            a[2][2] = Some(p_t);
            cp[0][1] = -1;
            cp[1][1] = -1;
            cp[2][1] = -1;
            cp[3][1] = -1;
            cn[0][0] = -1;
            cn[1][0] = -1;
            cn[2][0] = -1;
            cn[3][0] = -1;
        } else if p_t == 6 {
            // J
            a[0][1] = Some(p_t);
            a[1][1] = Some(p_t);
            a[2][1] = Some(p_t);
            a[2][0] = Some(p_t);
            cp[0][1] = -1;
            cp[1][1] = -1;
            cp[2][1] = -1;
            cp[3][1] = -1;
            cn[0][0] = -1;
            cn[1][0] = -1;
            cn[2][0] = -1;
            cn[3][0] = -1;
        }
    }
}
