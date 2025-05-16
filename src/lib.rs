// [問題]
// 2行5列の床に赤青黄のタイルを配置する。
// 1. 赤4枚、青4枚、黄2枚を使う。
// 2. 同じ色のタイルが隣接しないように配置する。
// 3. 全ての配置パターンを表示する。
// 4. 適合するパターンの数を表示する。

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub fn run() {
    let mut count = 0; // 適合したパターンの数
    'brute_force: for i in 0..59049 { // 全パターン 総当たり
        // 床パターンを生成
        let mut floor = Floor::new();
        for r in 0..2 {
            for c in 0..5 {
                floor.tiles[r][c] = match nth!(i, r*5+c+1, 3) {
                    0 => Tile::Red,
                    1 => Tile::Blue,
                    2 => Tile::Yellow,
                    _ => unreachable!(),
                };
            }
        }
        // タイルの枚数を確認
        if floor.count(Tile::Red   ) != 4 { continue 'brute_force; }
        if floor.count(Tile::Blue  ) != 4 { continue 'brute_force; }
        if floor.count(Tile::Yellow) != 2 { continue 'brute_force; }
        // タイルの配置を確認
        for r in 0..2 {
            for c in 0..5 {
                // 同じ色の隣接を却下
                if floor.check_adjacent(r, c) == false { continue 'brute_force; }
            }
        }
        // ここまで来たら適合するパターン
        count += 1;
        // 適合するパターンを表示
        floor.show();
    }
    // 総当たり終了
    println!("適合するパターンの数: {}", count);
}

// 床とタイルを定義

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    // 赤青黄の3色
    Red,
    Blue,
    Yellow,
}

#[derive(Debug, Clone, Copy)]
pub struct Floor {
    // 2行5列のタイル配置
    pub tiles: [[Tile; 5]; 2],
}


impl Floor {

    // 初期化

    fn new() -> Self {
        Self {
            tiles: [[Tile::Red; 5]; 2], // 赤で初期化
        }
    }

    // タイルの条件

    fn count(&self, tile: Tile) -> usize {
        let mut count = 0;
        for row in 0..2 {
            for col in 0..5 {
                if self.tiles[row][col] == tile {
                    count += 1;
                }
            }
        }
        count
    }

    fn check_adjacent(&self, row: usize, col: usize) -> bool {
        let tile = self.tiles[row][col];
        // 上下左右の隣接を確認
        if row > 0 && self.tiles[row - 1][col] == tile { return false; } // 上
        if row < 1 && self.tiles[row + 1][col] == tile { return false; } // 下
        if col > 0 && self.tiles[row][col - 1] == tile { return false; } // 左
        if col < 4 && self.tiles[row][col + 1] == tile { return false; } // 右
        true
    }

    // 表示用

    fn tile_char(tile: Tile) -> &'static str {
        match tile {
            Tile::Red    => "\x1b[31m赤\x1b[0m",
            Tile::Blue   => "\x1b[34m青\x1b[0m",
            Tile::Yellow => "\x1b[33m黄\x1b[0m",
        }
    }

    pub fn show(&self) {
        for row in 0..2 {
            let mut line = String::with_capacity(2);
            for col in 0..5 {
                let t = self.tiles[row][col];
                line.push_str(Self::tile_char(t));
            }
            println!("{}", line);
        }
        println!("");
    }
}


// 整数xのbase進数でのk桁目を取得するマクロ

#[macro_export]
macro_rules! nth {
    ($x:expr, $k:expr, $base:expr) => {{
        ($x / ($base as i32).pow((($k - 1) as u32))) % $base
    }};
}