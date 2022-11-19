use colored::Colorize;
mod step;
mod step_parser;
mod stone;
mod stonemap;
mod computer_player;

use step::Step;
use step_parser::parse_step;
use stonemap::StoneMap;
use computer_player::ComputerPlayer;
use crate::{stonemap::StoneIndex, stone::Camp};

fn main() {
    let mut stone_map = StoneMap::new();
    let mut input = String::new();

    let mut game_over = true;
    let mut steps: Vec<Step> = Vec::new();

    println!("游戲還未開始, 输入 `start` 或 `开始` 或 `開始` 开始游戏.");
    println!("如果需要幫助信息, 請輸入 `help` 或 `幫助` 或 `帮助` .");

    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("读取字符串失败");
        let input_str = input.trim();

        // 解析是否是相关操作命令
        match input_str {
            "quit" | "離開" | "退出" | "离开" => break,
            "start" | "开始" | "開始" => {
                stone_map.start();
                steps.clear();
                game_over = false;
                print!("{}", stone_map);
                continue;
            }
            "help" | "幫助" | "帮助" => {
                println!("~中國象棋對戰~");
                println!("輸入 `start` 或 `开始` 或 `開始` 开始游戏");
                println!("輸入 `quit` 或 `離開` 或 `退出` 或 `离开` 退出程序");
                println!("游戲開始後, 輸入正確的走步即可下棋. 正確走步如 `炮二平五`、`馬八進七`、`前車進一`等.");
                continue;
            }
            _ => {}
        };


        if !game_over {
            if input_str == "regret" {
                if steps.len() <= 0 {
                    println!("还没有走动呢.");
                } else {
                    let step = steps.pop().unwrap();
                    stone_map.revoke_move(&step);
                    stone_map.switch_turn();
                }
            } else if stone_map.turn == Camp::Down {
                match parse_step(&mut stone_map, input_str) {
                    Ok(step) => {
                        stone_map.make_move(&step);
                        stone_map.switch_turn();
                        steps.push(step);
                    }
                    Err(msg) => {
                        print!("{}\n", msg);
                    }
                }
            }
            else {
                println!("AI思考中.");
                let mut player = ComputerPlayer::new(stone_map.clone(), 4);
                let step = player.play();
                stone_map.make_move(&step);
                stone_map.switch_turn();
            }
            
            
            print!("{}", stone_map);
            if let StoneIndex::Dead = stone_map.up_stones[4] {
                game_over = true;
                println!("红方获胜!");
            } else if let StoneIndex::Dead = stone_map.down_stones[4] {
                game_over = true;
                println!("黑方获胜!");
            } else {
                println!(
                    "轮到{}",
                    match stone_map.turn {
                        Camp::Up => "黑方".bright_white(),
                        Camp::Down => "红方".red(),
                    }
                );
            }
        }
        else {
            println!("游戏已结束, 输入 `start` 或 `开始` 或 `開始` 开始游戏.");
        }
    
    }
}
/*
unicode符号
１２３４５６７８９０ ＡＢＣＤＥＦ
｜＋＿－￣

方案一　　　　　　　　　　　　　方案二
　　１２３４５６７８９　　　　　　　　１２３４５６７８９
　　＿＿＿＿＿＿＿＿＿　　　　　　　　＿＿＿＿＿＿＿＿＿
一｜車馬象士將士象馬車｜零　　　　　｜車馬象士將士象馬車｜
二｜　　　　　　　　　｜一　　　　　｜＋＋＋＋＋＋＋＋＋｜
三｜　砲　　　　　砲　｜二　　　　　｜＋砲＋＋＋＋＋砲＋｜
四｜卒　卒　卒　卒　卒｜三　　　　　｜卒＋卒＋卒＋卒＋卒｜
五｜－－－－－－－－－｜四　　　　　｜－－－－－－－－－｜
六｜－－－－－－－－－｜五　　　　　｜－－－－－－－－－｜
七｜兵　兵　兵　兵　兵｜六　　　　　｜兵＋兵＋兵＋兵＋兵｜
八｜　炮　　　　　炮　｜七　　　　　｜＋炮＋＋＋＋＋炮＋｜
九｜　　　　　　　　　｜八　　　　　｜＋＋＋＋＋＋＋＋＋｜
十｜俥傌相仕帥仕相傌俥｜九　　　　　｜俥傌相仕帥仕相傌俥｜
　　￣￣￣￣￣￣￣￣￣　　　　　　　　￣￣￣￣￣￣￣￣￣
　　９８７６５４３２１　　　　　　　　９８７６５４３２１



*/
