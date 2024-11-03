use std::cmp;
use std::collections::HashMap;
use std::fs::File;
//use std::io::Write;
use std::env;
use std::io::BufRead;
use std::io::BufReader;

fn initial_pos() -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::new();
    vec.push(vec![-1, 1, -1, 1, -1, 1]);
    vec.push(vec![1, -1, 1, -1, 1, -1]);
    vec.push(vec![-1, 1, -1, 0, -1, 1]);
    vec.push(vec![1, -1, 1, 0, 1, -1]);
    vec.push(vec![-1, 1, -1, 1, -1, 1]);
    vec.push(vec![1, -1, 1, -1, 1, -1]);
    return vec;
}

fn pack_values(eval: i32, depth: i32) -> i32 {
    return (eval + 140) + depth * 16384;
}

fn unpack_values(pack: i32) -> (i32, i32) {
    let depth: i32 = pack >> 14;
    let eval: i32 = ((pack << 18) >> 18) - 140;
    (eval, depth)
}

fn print_pos_with_move(pos: &Vec<Vec<i32>>, mv: &Vec<Vec<i32>>) {
    let mut res: String = "-------------\n".to_string();
    let mut i: i32 = 5;
    let mut j: i32;
    while i >= 0 {
        res = res + "|";
        j = 0;
        while j < 6 {
            if (i == mv[0][0] && j == mv[0][1]) || (i == mv[1][0] && j == mv[1][1]) {
                res = res + "\x1b[41m";
            }
            match &pos[i as usize][j as usize] {
                1 => {
                    res = res + "X";
                }
                0 => {
                    res = res + ".";
                }
                -1 => {
                    res = res + "O";
                }
                _ => (),
            }
            if (i == mv[0][0] && j == mv[0][1]) || (i == mv[1][0] && j == mv[1][1]) {
                res = res + "\x1b[0m";
            }
            res = res + "|";
            j += 1;
        }
        res = res + "\n";
        i -= 1;
    }
    res = res + "-------------";
    println!("{}", res);
}

fn print_move(mv: &Vec<Vec<i32>>) {
    let letters: HashMap<i32, &str> =
        HashMap::from([(0, "a"), (1, "b"), (2, "c"), (3, "d"), (4, "e"), (5, "f")]);
    let from_col = letters[&mv[0][1]];
    let to_col = letters[&mv[1][1]];
    println!("{}{} -> {}{}", from_col, mv[0][0] + 1, to_col, mv[1][0] + 1);
}

fn board_hash(pos: &Vec<Vec<i32>>, turn: i32) -> u64 {
    let mut i: u64 = 0;
    let mut j: u64;
    let mut res: u64 = 0;
    while i < 6 {
        j = 0;
        while j < 6 {
            res += (pos[i as usize][j as usize].abs() as u64) << (j + i * 6);
            j += 1;
        }
        i += 1;
    }
    res += ((turn + 1) as u64) << 35;
    return res;
}

fn find_moves(pos: &Vec<Vec<i32>>, turn: i32) -> Vec<Vec<Vec<i32>>> {
    let mut res: Vec<Vec<Vec<i32>>> = Vec::new();
    let mut i = 0;
    let j_init = ((turn + 1) as usize) / 2;
    let mut j;
    while i < 6 {
        j = (j_init + i) % 2;
        while j < 6 {
            if pos[i][j] == turn {
                if i <= 3 && pos[i + 1][j] != 0 && pos[i + 2][j] == 0 {
                    let mut curr_move: Vec<Vec<i32>> = Vec::new();
                    curr_move.push(vec![i as i32, j as i32]);
                    curr_move.push(vec![(i + 2) as i32, j as i32]);
                    res.push(curr_move);
                    if i <= 1 && pos[i + 3][j] != 0 && pos[i + 4][j] == 0 {
                        let mut one_more_move: Vec<Vec<i32>> = Vec::new();
                        one_more_move.push(vec![i as i32, j as i32]);
                        one_more_move.push(vec![(i + 4) as i32, j as i32]);
                        res.push(one_more_move);
                    }
                }
                if i >= 2 && pos[i - 1][j] != 0 && pos[i - 2][j] == 0 {
                    let mut curr_move: Vec<Vec<i32>> = Vec::new();
                    curr_move.push(vec![i as i32, j as i32]);
                    curr_move.push(vec![(i - 2) as i32, j as i32]);
                    res.push(curr_move);
                    if i >= 4 && pos[i - 3][j] != 0 && pos[i - 4][j] == 0 {
                        let mut one_more_move: Vec<Vec<i32>> = Vec::new();
                        one_more_move.push(vec![i as i32, j as i32]);
                        one_more_move.push(vec![(i - 4) as i32, j as i32]);
                        res.push(one_more_move);
                    }
                }
                if j <= 3 && pos[i][j + 1] != 0 && pos[i][j + 2] == 0 {
                    let mut curr_move: Vec<Vec<i32>> = Vec::new();
                    curr_move.push(vec![i as i32, j as i32]);
                    curr_move.push(vec![i as i32, (j + 2) as i32]);
                    res.push(curr_move);
                    if j <= 1 && pos[i][j + 3] != 0 && pos[i][j + 4] == 0 {
                        let mut one_more_move: Vec<Vec<i32>> = Vec::new();
                        one_more_move.push(vec![i as i32, j as i32]);
                        one_more_move.push(vec![i as i32, (j + 4) as i32]);
                        res.push(one_more_move);
                    }
                }
                if j >= 2 && pos[i][j - 1] != 0 && pos[i][j - 2] == 0 {
                    let mut curr_move: Vec<Vec<i32>> = Vec::new();
                    curr_move.push(vec![i as i32, j as i32]);
                    curr_move.push(vec![i as i32, (j - 2) as i32]);
                    res.push(curr_move);
                    if j >= 4 && pos[i][j - 3] != 0 && pos[i][j - 4] == 0 {
                        let mut one_more_move: Vec<Vec<i32>> = Vec::new();
                        one_more_move.push(vec![i as i32, j as i32]);
                        one_more_move.push(vec![i as i32, (j - 4) as i32]);
                        res.push(one_more_move);
                    }
                }
            }
            j += 2;
        }
        i += 1;
    }
    return res;
}

fn apply_move_in_place(pos: &mut Vec<Vec<i32>>, mv: &Vec<Vec<i32>>) {
    let side = pos[mv[0][0] as usize][mv[0][1] as usize];
    let mut direction = 0;
    if mv[0][1] == mv[1][1] {
        direction = 1;
    }
    if direction == 0 {
        let mut i = cmp::min(mv[0][1], mv[1][1]);
        let max_y = cmp::max(mv[0][1], mv[1][1]);
        while i <= max_y {
            pos[mv[0][0] as usize][i as usize] = 0;
            i += 1;
        }
    } else {
        let mut i = cmp::min(mv[0][0], mv[1][0]);
        let max_x = cmp::max(mv[0][0], mv[1][0]);
        while i <= max_x {
            pos[i as usize][mv[0][1] as usize] = 0;
            i += 1;
        }
    }
    pos[mv[1][0] as usize][mv[1][1] as usize] = side;
}

fn revert_move(pos: &mut Vec<Vec<i32>>, mv: &Vec<Vec<i32>>) {
    let side = pos[mv[1][0] as usize][mv[1][1] as usize];
    pos[mv[1][0] as usize][mv[1][1] as usize] = 0;
    pos[mv[0][0] as usize][mv[0][1] as usize] = side;
    if mv[0][0] == mv[1][0] {
        let from = mv[1][1];
        let to = mv[0][1];
        let mut direction = 1;
        if to < from {
            direction = -1;
        }
        let mut i = from + direction;
        while i * direction < to * direction {
            pos[mv[0][0] as usize][i as usize] = -1 * side;
            i += direction * 2;
        }
    } else {
        let from = mv[1][0];
        let to = mv[0][0];
        let mut direction = 1;
        if to < from {
            direction = -1;
        }
        let mut i = from + direction;
        while i * direction < to * direction {
            pos[i as usize][mv[0][1] as usize] = -1 * side;
            i += direction * 2;
        }
    }
}

fn eval_position(pos: &Vec<Vec<i32>>, turn: i32) -> i32 {
    let ours = find_moves(&pos, turn).len() as i32;
    if ours == 0 {
        return turn * 100 * -1;
    }
    let theirs = find_moves(&pos, -1 * turn).len() as i32;
    return (ours - theirs) * turn;
}

fn search_in_place(
    pos: &mut Vec<Vec<i32>>,
    turn: i32,
    depth: i32,
    cache: &mut HashMap<u64, i32>,
    leaves: &mut u64,
    cache_hits: &mut u64,
    alpha: i32,
    beta: i32,
    nodes: &mut u64,
) -> i32 {
    *nodes += 1;
    let pos_hash = board_hash(&pos, turn);
    if depth == 0 {
        *leaves += 1;
        match cache.get(&pos_hash) {
            Some(value) => {
                *cache_hits += 1;
                let (eval, _dpt) = unpack_values(*value);
                return eval;
            }
            None => {
                let eval = eval_position(&pos, turn);
                cache.insert(pos_hash, pack_values(eval, 0));
                return eval;
            }
        }
    }
    match cache.get(&pos_hash) {
        Some(value) => {
            let (eval, dpt) = unpack_values(*value);
            if eval.abs() >= 100 || dpt >= depth {
                *cache_hits += 1;
                return eval;
            }
        }
        None => (),
    }
    let next_moves = find_moves(&pos, turn);
    if next_moves.len() == 0 {
        cache.insert(pos_hash, pack_values((-100 - depth) * turn, depth));
        return (-100 - depth) * turn;
    }
    let mut local_alpha = alpha;
    let mut local_beta = beta;
    let mut max_eval = -10000;
    for mv in next_moves {
        apply_move_in_place(pos, &mv);
        let curr_eval = turn
            * search_in_place(
                pos,
                -1 * turn,
                depth - 1,
                cache,
                leaves,
                cache_hits,
                local_alpha,
                local_beta,
                nodes,
            );
        revert_move(pos, &mv);
        if curr_eval > max_eval {
            if turn == 1 && curr_eval > local_beta {
                return curr_eval * turn;
            } else if turn == -1 && (-1 * curr_eval) < local_alpha {
                return curr_eval * turn;
            }
            max_eval = curr_eval;
            if turn == 1 {
                local_alpha = max_eval;
            } else {
                local_beta = -1 * max_eval;
            }
        }
    }
    cache.insert(pos_hash, pack_values(max_eval * turn, depth));
    return max_eval * turn;
}

fn read_position(path: &str) -> Vec<Vec<i32>> {
    let mut vec: Vec<Vec<i32>> = Vec::new();
    let f = File::open(path);
    match f {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line_raw in reader.lines() {
                if let Ok(line) = line_raw {
                    let mut row: Vec<i32> = Vec::new();
                    for c in line.chars() {
                        match c {
                            'x' => {
                                row.push(1);
                            }
                            'o' => {
                                row.push(-1);
                            }
                            '.' => {
                                row.push(0);
                            }
                            _ => (),
                        }
                    }
                    vec.push(row);
                }
            }
        }
        Err(..) => {
            panic!("Could not open input file!");
        }
    }
    return vec.into_iter().rev().collect();
}

fn all_is_lost(mv: &Vec<Vec<i32>>) -> bool {
    if mv[0][0] == 0 && mv[1][0] == 0 && mv[0][1] == 0 && mv[1][1] == 0 {
        return true;
    }
    return false;
}

fn solve_in_place(
    pos: &mut Vec<Vec<i32>>,
    turn: i32,
    cache: &mut HashMap<u64, i32>,
    depth: i32,
) -> Vec<Vec<i32>> {
    let next_moves = find_moves(&pos, turn);
    let pos_hash = board_hash(&pos, turn);
    let mut leaves: u64 = 0;
    let mut cache_hits: u64 = 0;
    let mut nodes: u64 = 0;
    if next_moves.len() == 0 {
        cache.insert(pos_hash, pack_values((-100 - depth - 1) * turn, depth + 1));
        println!("Player {}: WE LOSE!", turn);
        let mut best_move: Vec<Vec<i32>> = Vec::new();
        best_move.push(vec![0, 0]);
        best_move.push(vec![0, 0]);
        return best_move;
    } else {
        let mut max_eval = -10000;
        let mut best_move: Vec<Vec<i32>> = Vec::new();
        let alpha = -2000;
        let beta = 2000;
        for mv in next_moves {
            apply_move_in_place(pos, &mv);
            let curr_eval = turn
                * search_in_place(
                    pos,
                    -1 * turn,
                    depth,
                    cache,
                    &mut leaves,
                    &mut cache_hits,
                    alpha,
                    beta,
                    &mut nodes,
                );
            revert_move(pos, &mv);
            if curr_eval > max_eval {
                max_eval = curr_eval;
                best_move = mv;
            }
        }
        //print_move(&best_move);
        println!(
            "Eval: {}, leaves visited: {}, nodes: {}, cache hits: {}, cache_length: {}",
            max_eval * turn,
            leaves,
            nodes,
            cache_hits,
            cache.len()
        );
        return best_move;
    }
}

fn main() {
    //let moves = prep_moves();
    let mut cache: HashMap<u64, i32> = HashMap::new();
    let args: Vec<String> = env::args().collect();
    let mut curr_pos = initial_pos();
    let mut depth: i32 = 8;
    let mut one = false;
    if args.len() > 1 {
        depth = args[1].parse().unwrap();
    }

    if args.len() > 2 {
        println!("Loading position from {}", args[2]);
        curr_pos = read_position(&args[2]);
    }

    if args.len() > 3 && args[3] == "1" {
        one = true;
    }
    let mut turn = 1;
    let mut next_move = solve_in_place(&mut curr_pos, turn, &mut cache, depth);
    if one {
        apply_move_in_place(&mut curr_pos, &next_move);
        print_pos_with_move(&curr_pos, &next_move);
        print_move(&next_move);
        return;
    }
    while all_is_lost(&next_move) == false {
        turn *= -1;
        apply_move_in_place(&mut curr_pos, &next_move);
        print_pos_with_move(&curr_pos, &next_move);
        print_move(&next_move);
        next_move = solve_in_place(&mut curr_pos, turn, &mut cache, depth);
    }
}
