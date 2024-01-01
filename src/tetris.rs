use crossterm::event::KeyCode;
use rand::Rng;

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
            arr: [[None; 10]; 20],
            piece,
            map: [[None; 10]; 20],
        }
    }

    pub fn get_arr(&self, i: usize, j: usize) -> Option<i8> {
        return self.arr[i][j];
    }

    pub fn map_to_string(&self) -> String {
        let mut s = String::new();
        for i in 0..20 {
            for j in 0..10 {
                match self.map[i][j] {
                    Some(x) => s.push_str("██"),
                    None => s.push_str("  "),
                }
                /*if self.map[i][j] == 0 {
                    s.push_str("  ");
                } else {
                    s.push_str("[]");
                }*/
            }
            s.push_str("\n");
        }
        s
    }

    pub fn tick(&mut self) {
        /*
        let mut rng = rand::thread_rng();
        let mut i = rng.gen_range(0..20);
        let mut j = rng.gen_range(0..10);
        self.arr[i][j] = Some(1);*/

        if self.counter % 30 == 0 {
            self.action();
        }
        self.render_piece();
        self.counter += 1;
        return;
    }
    pub fn input(&mut self, key: i32) {
        let Some(piece) = &mut self.piece else {
            return;
        };
        if key == 1 {
            piece.movement(KeyCode::Left, &self.arr);
        }
        if key == 2 {
            piece.movement(KeyCode::Right, &self.arr);
        }
        if key == 3 {
            piece.down(&self.arr);
        }
        if key == 4 {
            piece.rotate(false, &self.arr);
        }
        if key == 5 {
            piece.rotate(true, &self.arr);
        }
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
                    if piece.arr[i][j] == Some(1) {
                        self.map[(piece.offset[0] + i as i32) as usize][piece.offset_h(j)] =
                            Some(1);
                    }
                }
            }
        }
    }
    pub fn get_debug_value(&self) -> String {
        let mut s = String::new();
        s.push_str("y:");
        s.push_str(&self.piece.as_ref().unwrap().offset[0].to_string());
        s.push_str(" x:");
        s.push_str(&self.piece.as_ref().unwrap().offset[1].to_string());
        s.push_str(" ğ:");

        s
    }
    pub fn action(&mut self) {
        let mut canMove = true;
        if let Some(piece) = &mut self.piece {
            for i in 0..4 {
                for j in 0..4 {
                    if piece.arr[i][j] != Some(1) {
                        continue;
                    }
                    if piece.offset_v(i) as i32 + 1 >= 20 {
                        canMove = false;
                        break;
                    }
                    if self.arr[piece.offset_v(i) + 1][piece.offset_h(j)] != None {
                        canMove = false;
                    }
                }
            }
            if canMove {
                piece.offset[0] += 1;
            } else {
                for i in 0..4 {
                    for j in 0..4 {
                        if piece.arr[i][j] == Some(1) {
                            self.arr[piece.offset_v(i)][piece.offset_h(j)] = Some(1);
                        }
                    }
                }
                self.line_check(self.piece.as_ref().unwrap().offset[0] as usize);
                self.piece = Option::Some(Piece::new());
            }
        }
    }

    fn line_check(&mut self, p: usize) {
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
                //self.arr[i][j] = self.arr[i - count][j];
            }
        }
    }
}

pub struct Piece {
    p_type: i32,
    rotate_count: u16,
    centern_offset: [[i32; 2]; 4],
    centerp_offset: [[i32; 2]; 4],
    pub arr: [[Option<i8>; 4]; 4],
    pub offset: [i32; 2],
}

impl Piece {
    pub fn new() -> Piece {
        let pT = rand::thread_rng().gen_range(0..7);
        let mut ar: [[Option<i8>; 4]; 4] = [[None; 4]; 4];
        let mut centern_offset: [[i32; 2]; 4] = [[0; 2]; 4];
        let mut centerp_offset: [[i32; 2]; 4] = [[0; 2]; 4];
        self::Piece::init(&mut ar, &mut centern_offset, &mut centerp_offset, pT);

        Piece {
            p_type: pT,
            rotate_count: (u16::MAX - 7) / 2,
            arr: ar,
            centern_offset: centern_offset,
            centerp_offset: centerp_offset,
            offset: [0, 5],
        }
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
        for i in 0..4 {
            for j in 0..4 {
                if self.arr[i][j] == Some(1) {
                    if self.offset_v(i) as i32 + 1 >= 20 {
                        return;
                    }
                    let i = (self.offset_v(i) as i32 + 1) as usize;
                    if tmp_arr[i][self.offset_h(j)] != None {
                        return;
                    }
                }
            }
        }
        self.offset[0] += 1;
    }

    pub fn movement(&mut self, dir: KeyCode, tmp_arr: &[[Option<i8>; 10]; 20]) {
        let mut canMove = true;
        let direction;
        if dir == KeyCode::Left {
            direction = -1;
        } else {
            direction = 1;
        }
        for i in 0..4 {
            for j in 0..4 {
                if self.arr[i][j] == Some(1) {
                    if self.offset_h(j) as i32 + direction < 0
                        || self.offset_h(j) as i32 + direction >= 10
                    {
                        canMove = false;
                        break;
                    }
                    if ({
                        let j = (self.offset_h(j) as i32 + direction) as usize;
                        tmp_arr[self.offset_v(i)][j]
                    }) != None
                    {
                        canMove = false;
                    }
                }
            }
        }
        if canMove {
            if dir == KeyCode::Left {
                self.offset[1] -= 1;
            }
            if dir == KeyCode::Right {
                self.offset[1] += 1;
            }
        }
    }

    pub fn rotate(&mut self, clock_wise: bool, tmp_arr: &[[Option<i8>; 10]; 20]) {
        let mut ar: [[Option<i8>; 4]; 4] = [[None; 4]; 4];
        let mut canRotate = true;

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

    fn init(a: &mut [[Option<i8>; 4]; 4], cn: &mut [[i32; 2]; 4], cp: &mut [[i32; 2]; 4], pT: i32) {
        if pT == 0 {
            // I
            a[0][2] = Some(1);
            a[1][2] = Some(1);
            a[2][2] = Some(1);
            a[3][2] = Some(1);
            //c.copy_from_slice(&[[0, 0], [0, 0], [-1, 0], [0, 1]]);
            cp[0][1] = 1;
            cp[2][1] = 1;
            cn[1][0] = 1;
            cn[3][0] = 1;
        }
        if pT == 1 {
            // O
            a[1][1] = Some(1);
            a[1][2] = Some(1);
            a[2][1] = Some(1);
            a[2][2] = Some(1);
        }
        if pT == 2 {
            // T
            a[1][1] = Some(1);
            a[2][0] = Some(1);
            a[2][1] = Some(1);
            a[2][2] = Some(1);
            cp[0][0] = -1;
            cp[1][0] = -1;
            cp[2][0] = -1;
            cp[3][0] = -1;
            cn[0][1] = 1;
            cn[1][1] = 1;
            cn[2][1] = 1;
            cn[3][1] = 1;
        }
        if pT == 3 {
            // S
            a[1][1] = Some(1);
            a[1][2] = Some(1);
            a[2][0] = Some(1);
            a[2][1] = Some(1);
            cp[0][1] = -1;
            cp[2][1] = -1;
            cn[1][0] = -1;
            cn[3][0] = -1;
        }
        if pT == 4 {
            // Z
            a[1][0] = Some(1);
            a[1][1] = Some(1);
            a[2][1] = Some(1);
            a[2][2] = Some(1);
            cp[0][1] = -1;
            cp[2][1] = -1;
            cn[1][0] = -1;
            cn[3][0] = -1;
        }
        if pT == 5 {
            // L
            a[0][1] = Some(1);
            a[1][1] = Some(1);
            a[2][1] = Some(1);
            a[2][2] = Some(1);
            cp[0][1] = -1;
            cp[1][1] = -1;
            cp[2][1] = -1;
            cp[3][1] = -1;
            cn[0][0] = -1;
            cn[1][0] = -1;
            cn[2][0] = -1;
            cn[3][0] = -1;
        }
        if pT == 6 {
            // J
            a[0][1] = Some(1);
            a[1][1] = Some(1);
            a[2][1] = Some(1);
            a[2][0] = Some(1);
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
