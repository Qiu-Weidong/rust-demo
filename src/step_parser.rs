use core::num;

use crate::step::Step;
use crate::stone::Camp::Down;
use crate::stone::Camp::Up;
use crate::stone::StoneType::{Bishop, Cannon, King, Knight, Mandarin, Pawn, Rook};
use crate::stonemap::{StoneIndex, StoneMap};
use StoneIndex::Alive;

pub fn parse_step(map: &mut StoneMap, input: &str) -> Result<Step, String> {
    // 首先转换为 字符数组
    let chars: Vec<char> = input.trim().chars().collect();
    if chars.len() < 4
        || chars.len() >= 5 && chars[1] != '兵' && chars[1] != '卒'
        || chars.len() > 5
    {
        return Err(String::from("非法输入."));
    }
    let chars = if chars.len() >= 5 {
        [chars[0], chars[2], chars[3], chars[4]]
    } else {
        [chars[0], chars[1], chars[2], chars[3]]
    };

    // 获取棋子的起始位置
    let (x, y) = get_location(map, &chars)?;
    // 获取棋子的目标位置
    let (dest_x, dest_y) = get_dest(map, &chars, (x, y))?;
    // 构造step
    let step = map.make_step((x, y), (dest_x, dest_y));
    // 判断可达性
    if map.can_move(&step) {
        Ok(step)
    } else {
        Err(String::from("非法走步."))
    }
}
#[allow(dead_code)]
pub fn describe_step(map: &mut StoneMap, step: &Step) -> Result<String, String> {
    if !map.can_move(step) {
        Err(String::from("非法走步."))
    } else {
        let mover = map.stone_map[step.from.0][step.from.1].unwrap(); // 一定有mover
        match mover.stone_type {
            King => describe_king_step(map, step),
            Mandarin => describe_mandarin_step(map, step),
            Bishop => describe_bishop_step(map, step),
            Knight => describe_knight_step(map, step),
            Rook => describe_rook_step(map, step),
            Cannon => describe_cannon_step(map, step),
            Pawn => describe_pawn_step(map, step),
        }
    }
}

fn describe_king_step(map: &StoneMap, step: &Step) -> Result<String, String> {
    let mover = map.stone_map[step.from.0][step.from.1].unwrap();
    let stones = match mover.camp {
        Up => &map.up_stones,
        Down => &map.down_stones,
    };
    let col = step.from.1;
    let col = match mover.camp {
        Up => col + 1,
        Down => 9 - col,
    };

    let op: char = if step.from.0 == step.to.0 {
        '平'
    } else if step.from.1 < step.to.1 && mover.camp == Up
        || step.from.1 > step.to.1 && mover.camp == Down
    {
        '進'
    } else {
        '退'
    };
    let chars = [mover.get_char(), number_to_char(col)?, op];
    let ret: String = chars.iter().collect();
    Ok(ret)
}
fn describe_mandarin_step(map: &StoneMap, step: &Step) -> Result<String, String> {
    todo!()
}
fn describe_bishop_step(map: &StoneMap, step: &Step) -> Result<String, String> {
    todo!()
}
fn describe_knight_step(map: &StoneMap, step: &Step) -> Result<String, String> {
    todo!()
}
fn describe_rook_step(map: &StoneMap, step: &Step) -> Result<String, String> {
    todo!()
}
fn describe_cannon_step(map: &StoneMap, step: &Step) -> Result<String, String> {
    todo!()
}
fn describe_pawn_step(map: &StoneMap, step: &Step) -> Result<String, String> {
    todo!()
}

// 解析走步的函数
fn get_location(map: &StoneMap, input: &[char]) -> Result<(usize, usize), String> {
    match input[0] {
        '将' | '帅' | '將' | '帥' | '王' => get_king_location(map, input),
        '车' | '伡' | '車' | '俥' => get_rook_location(map, input),
        '炮' | '砲' => get_cannon_location(map, input),
        '兵' | '卒' => get_pawn_location(map, input),
        '士' | '仕' => get_mandarin_location(map, input),
        '象' | '相' => get_bishop_location(map, input),
        '馬' | '傌' | '马' | '㐷' => get_knight_location(map, input),
        '前' | '后' | '後' | '中' | '二' | '贰' | '2' | '２' | '三' | '叁' | '3' | '３' | '四'
        | '肆' | '4' | '４' => match input[1] {
            '车' | '伡' | '車' | '俥' => get_rook_location(map, input),
            '炮' | '砲' => get_cannon_location(map, input),
            '兵' | '卒' => get_pawn_location(map, input),
            '士' | '仕' => get_mandarin_location(map, input),
            '象' | '相' => get_bishop_location(map, input),
            '馬' | '傌' | '马' | '㐷' => get_knight_location(map, input),
            '一' | '壹' | '1' | '１' | '二' | '贰' | '2' | '２' | '三' | '叁' | '3' | '３'
            | '四' | '肆' | '4' | '４' | '五' | '伍' | '5' | '５' | '六' | '陆' | '6' | '６'
            | '七' | '柒' | '7' | '７' | '八' | '捌' | '8' | '８' | '九' | '玖' | '9' | '９' => {
                get_pawn_location(map, input)
            }
            _ => Err(format!("未知棋子 `{}`", input[1])),
        },
        _ => Err(format!("未知棋子 `{}`", input[0])),
    }
}
fn get_king_location(map: &StoneMap, input: &[char]) -> Result<(usize, usize), String> {
    let col = char_to_number(input[1])?;
    let col = match map.turn {
        Up => col - 1,
        Down => 9 - col,
    };
    let stones = map.get_current_stones();

    // 检查是否活着
    if let Alive(x, y) = stones[4] {
        if y != col {
            Err(format!("将或帅不位于列{}", input[1]))
        } else {
            Ok((x, y))
        }
    } else {
        Err(String::from("将或帅已经死亡."))
    }
}
fn get_mandarin_location(map: &StoneMap, input: &[char]) -> Result<(usize, usize), String> {
    get_two_location(map, input, 3, 5)
}
fn get_bishop_location(map: &StoneMap, input: &[char]) -> Result<(usize, usize), String> {
    get_two_location(map, input, 2, 6)
}
fn get_knight_location(map: &StoneMap, input: &[char]) -> Result<(usize, usize), String> {
    get_two_location(map, input, 1, 7)
}
fn get_rook_location(map: &StoneMap, input: &[char]) -> Result<(usize, usize), String> {
    get_two_location(map, input, 0, 8)
}
fn get_cannon_location(map: &StoneMap, input: &[char]) -> Result<(usize, usize), String> {
    get_two_location(map, input, 10, 14)
}
fn get_pawn_location(map: &StoneMap, input: &[char]) -> Result<(usize, usize), String> {
    let stones = map.get_current_stones();
    // 不管怎么样，首先要找到所有活着的兵
    let mut pawns: Vec<(usize, usize)> = Vec::new();
    if let Alive(x, y) = stones[9] {
        pawns.push((x, y));
    }
    if let Alive(x, y) = stones[11] {
        pawns.push((x, y));
    }
    if let Alive(x, y) = stones[12] {
        pawns.push((x, y));
    }
    if let Alive(x, y) = stones[13] {
        pawns.push((x, y));
    }
    if let Alive(x, y) = stones[15] {
        pawns.push((x, y));
    }
    if pawns.len() <= 0 {
        return Err(String::from("兵全部死了."));
    }

    match input[0] {
        '前' | '后' | '後' | '二' | '三' | '四' => match input[1] {
            '兵' | '卒' => {
                // [前后後二三四]兵进一 这种情况下必须所有兵在同一条直线上
                let col = pawns[0].1;
                for (_x, y) in pawns.iter() {
                    if *y != col {
                        return Err(String::from("兵不在同一直綫上,請指明兵所在列."));
                    }
                }
                match map.turn {
                    Up => pawns.sort_by(|(x1, _y1), (x2, _y2)| {
                        if *x1 > *x2 {
                            std::cmp::Ordering::Less
                        } else if *x1 == *x2 {
                            std::cmp::Ordering::Equal
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    }),
                    Down => pawns.sort_by(|(x1, _y1), (x2, _y2)| {
                        if *x1 < *x2 {
                            std::cmp::Ordering::Less
                        } else if *x1 == *x2 {
                            std::cmp::Ordering::Equal
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    }),
                };
                if input[0] == '后' || input[0] == '後' {
                    Ok(pawns[pawns.len() - 1])
                } else if input[0] == '前' {
                    Ok(pawns[0])
                } else {
                    let index = char_to_number(input[0])? - 1;
                    if index >= pawns.len() {
                        Err(format!("todo."))
                    } else {
                        Ok(pawns[index])
                    }
                }
            }
            '一' | '壹' | '1' | '１' | '二' | '贰' | '2' | '２' | '三' | '叁' | '3' | '３'
            | '四' | '肆' | '4' | '４' | '五' | '伍' | '5' | '５' | '六' | '陆' | '6' | '６'
            | '七' | '柒' | '7' | '７' | '八' | '捌' | '8' | '８' | '九' | '玖' | '9' | '９' =>
            {
                // [前后後二三四]三进一
                let col = char_to_number(input[1])?;
                // col要作處理
                let col = match map.turn {
                    Up => col - 1,
                    Down => 9 - col,
                };
                let mut tmp: Vec<(usize, usize)> = Vec::new();
                for (_x, y) in pawns.iter() {
                    if *y == col {
                        tmp.push((*_x, *y));
                    }
                }
                let mut pawns = tmp;
                match map.turn {
                    Up => pawns.sort_by(|(x1, _y1), (x2, _y2)| {
                        if *x1 > *x2 {
                            std::cmp::Ordering::Less
                        } else if *x1 == *x2 {
                            std::cmp::Ordering::Equal
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    }),
                    Down => pawns.sort_by(|(x1, _y1), (x2, _y2)| {
                        if *x1 < *x2 {
                            std::cmp::Ordering::Less
                        } else if *x1 == *x2 {
                            std::cmp::Ordering::Equal
                        } else {
                            std::cmp::Ordering::Greater
                        }
                    }),
                };

                if input[0] == '后' || input[0] == '後' {
                    Ok(pawns[pawns.len() - 1])
                } else if input[0] == '前' {
                    Ok(pawns[0])
                } else {
                    let index = char_to_number(input[0])? - 1;
                    if index >= pawns.len() {
                        Err(format!("todo."))
                    } else {
                        Ok(pawns[index])
                    }
                }
            }
            _ => Err(format!("未知字符 `{}`", input[1])),
        },
        '兵' | '卒' => {
            // 兵五平一
            let col = char_to_number(input[1])?;
            // col要作處理
            let col = match map.turn {
                Up => col - 1,
                Down => 9 - col,
            };

            let mut ret: (usize, usize) = (0, 0);
            let mut cnt = 0;
            for (x, y) in pawns.iter() {
                if *y == col {
                    cnt += 1;
                    ret = (*x, *y);
                }
            }
            if cnt != 1 {
                Err(format!(""))
            } else {
                Ok(ret)
            }
        }
        _ => Err(format!("未知字符 `{}`", input[0])),
    }
}

fn get_dest(
    map: &StoneMap,
    input: &[char],
    from: (usize, usize),
) -> Result<(usize, usize), String> {
    let (dest_x, dest_y) = match input[0] {
        '兵' | '卒' | '车' | '伡' | '車' | '俥' | '炮' | '砲' | '将' | '帅' | '將' | '帥'
        | '王' => parse_straight_dest(map, input, from)?,

        '士' | '仕' => parse_mandarin_dest(map, input, from)?,
        '象' | '相' => parse_bishop_dest(map, input, from)?,
        '馬' | '傌' | '马' | '㐷' => parse_knight_dest(map, input, from)?,

        _ => match input[1] {
            '士' | '仕' => parse_mandarin_dest(map, input, from)?,
            '象' | '相' => parse_bishop_dest(map, input, from)?,
            '馬' | '傌' | '马' | '㐷' => parse_knight_dest(map, input, from)?,
            _ => parse_straight_dest(map, input, from)?,
        },
    };

    if dest_x < 0 || dest_y < 0 || dest_x > 9 || dest_y > 8 {
        return Err(format!("todo"));
    }
    Ok((dest_x as usize, dest_y as usize))
}

fn parse_straight_dest(
    map: &StoneMap,
    input: &[char],
    from: (usize, usize),
) -> Result<(i32, i32), String> {
    let dest = char_to_number(input[3])?;
    let line = match map.turn {
        Up => dest - 1,
        Down => 9 - dest,
    };

    match input[2] {
        '進' | '进' => match map.turn {
            Up => Ok((from.0 as i32 + dest as i32, from.1 as i32)),
            Down => Ok((from.0 as i32 - dest as i32, from.1 as i32)),
        },
        '退' => match map.turn {
            Up => Ok((from.0 as i32 - dest as i32, from.1 as i32)),
            Down => Ok((from.0 as i32 + dest as i32, from.1 as i32)),
        },
        '平' => Ok((from.0 as i32, line as i32)),
        _ => return Err(format!("未知操作`{}`", input[2])),
    }
}
fn parse_knight_dest(
    map: &StoneMap,
    input: &[char],
    from: (usize, usize),
) -> Result<(i32, i32), String> {
    let line = char_to_number(input[3])?;
    let line = match map.turn {
        Up => line - 1,
        Down => 9 - line,
    };

    let dest = (from.1 as i32 - line as i32) * (from.1 as i32 - line as i32);
    let gap = if dest == 1 {
        2
    } else if dest == 4 {
        1
    } else {
        return Err(format!("todo"));
    };

    match input[2] {
        '進' | '进' => match map.turn {
            Up => Ok((from.0 as i32 + gap, line as i32)),
            Down => Ok((from.0 as i32 - gap, line as i32)),
        },
        '退' => match map.turn {
            Up => Ok((from.0 as i32 - gap, line as i32)),
            Down => Ok((from.0 as i32 + gap, line as i32)),
        },
        _ => return Err(format!("未知操作`{}`", input[2])),
    }
}
fn parse_bishop_dest(
    map: &StoneMap,
    input: &[char],
    from: (usize, usize),
) -> Result<(i32, i32), String> {
    let line = char_to_number(input[3])?;
    let line = match map.turn {
        Up => line - 1,
        Down => 9 - line,
    };

    let dest = (from.1 as i32 - line as i32) * (from.1 as i32 - line as i32);
    if dest != 4 {
        return Err(format!(""));
    }

    match input[2] {
        '進' | '进' => match map.turn {
            Up => Ok((from.0 as i32 + 2, line as i32)),
            Down => Ok((from.0 as i32 - 2, line as i32)),
        },
        '退' => match map.turn {
            Up => Ok((from.0 as i32 - 2, line as i32)),
            Down => Ok((from.0 as i32 + 2, line as i32)),
        },
        _ => return Err(format!("未知操作`{}`", input[2])),
    }
}
fn parse_mandarin_dest(
    map: &StoneMap,
    input: &[char],
    from: (usize, usize),
) -> Result<(i32, i32), String> {
    let line = char_to_number(input[3])?;
    let line = match map.turn {
        Up => line - 1,
        Down => 9 - line,
    };

    let dest = (from.1 as i32 - line as i32) * (from.1 as i32 - line as i32);
    if dest != 1 {
        return Err(format!(""));
    }

    match input[2] {
        '進' | '进' => match map.turn {
            Up => Ok((from.0 as i32 + 1, line as i32)),
            Down => Ok((from.0 as i32 - 1, line as i32)),
        },
        '退' => match map.turn {
            Up => Ok((from.0 as i32 - 1, line as i32)),
            Down => Ok((from.0 as i32 + 1, line as i32)),
        },
        _ => return Err(format!("未知操作`{}`", input[2])),
    }
}

// 一个辅助函数
fn char_to_number(c: char) -> Result<usize, String> {
    match c {
        '一' | '壹' | '1' | '１' => Ok(1),
        '二' | '贰' | '2' | '２' => Ok(2),
        '三' | '叁' | '3' | '３' => Ok(3),
        '四' | '肆' | '4' | '４' => Ok(4),
        '五' | '伍' | '5' | '５' => Ok(5),
        '六' | '陆' | '6' | '６' => Ok(6),
        '七' | '柒' | '7' | '７' => Ok(7),
        '八' | '捌' | '8' | '８' => Ok(8),
        '九' | '玖' | '9' | '９' => Ok(9),
        _ => Err(format!("无法将字符 `{}` 解析为数字 1..9 .", c)),
    }
}
fn number_to_char(x: usize) -> Result<char, String> {
    match x {
        1 => Ok('一'),
        2 => Ok('二'),
        3 => Ok('三'),
        4 => Ok('四'),
        5 => Ok('五'),
        6 => Ok('六'),
        7 => Ok('七'),
        8 => Ok('八'),
        9 => Ok('九'),
        _ => Err(format!("无法将数字 `{}` 转换为字符", x)),
    }
}
// 獲取 士、相、馬、車、炮 的位置
fn get_two_location(
    map: &StoneMap,
    input: &[char],
    first: usize,
    second: usize,
) -> Result<(usize, usize), String> {
    let stones = map.get_current_stones();

    let stone_1 = stones[first];
    let stone_2 = stones[second];

    let result: Result<(usize, usize), String> = match input[0] {
        '前' => {
            if let (Alive(x1, y1), Alive(x2, y2)) = (stone_1, stone_2) {
                if y1 != y2 {
                    Err(format!("兩個`{}`不在同一列.", input[1]))
                } else if map.turn == Up && x1 < x2 || map.turn == Down && x1 > x2 {
                    Ok((x2, y2))
                } else {
                    Ok((x1, y1))
                }
            } else {
                Err(format!("兩個`{}`不在同一列.", input[1]))
            }
        }
        '后' | '後' => {
            if let (Alive(x1, y1), Alive(x2, y2)) = (stone_1, stone_2) {
                if y1 != y2 {
                    Err(format!("兩個`{}`不在同一列.", input[1]))
                } else if map.turn == Up && x1 < x2 || map.turn == Down && x1 > x2 {
                    Ok((x1, y1))
                } else {
                    Ok((x2, y2))
                }
            } else {
                Err(format!("兩個`{}`不在同一列.", input[1]))
            }
        }
        _ => {
            let col = char_to_number(input[1])?;
            // col要作處理
            let col = match map.turn {
                Up => col - 1,
                Down => 9 - col,
            };

            if let (Alive(x1, y1), Alive(x2, y2)) = (stone_1, stone_2) {
                // 都活著，保證不共綫
                if y1 == y2 {
                    Err(format!("兩個`{}`共綫, 請使用`前`、`后/後`指明.", input[0]))
                } else if y1 == col {
                    Ok((x1, y1))
                } else if y2 == col {
                    Ok((x2, y2))
                } else {
                    Err(format!("兩個`{}`都不在列{}上.", input[0], input[1]))
                }
            } else if let Alive(x, y) = stone_1 {
                if y != col {
                    Err(format!("沒有`{}`位於列{}", input[0], input[1]))
                } else {
                    Ok((x, y))
                }
            } else if let Alive(x, y) = stone_2 {
                if y != col {
                    Err(format!("沒有`{}`位於列{}", input[0], input[1]))
                } else {
                    Ok((x, y))
                }
            } else {
                Err(format!("兩個`{}`都已經陣亡了.", input[0]))
            }
        }
    };
    result
}
