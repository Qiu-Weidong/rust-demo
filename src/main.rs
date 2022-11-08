pub mod stone;
mod stonemap;
pub mod step;

use stonemap::StoneMap;

fn main() {
    let mut stone_map = StoneMap::new();

    

    let mut input = String::new();

    loop {
        print!("{}", stone_map);
        
        input.clear();
        std::io::stdin().read_line(&mut input).expect("读取字符串失败");
        let input_str = input.trim();
        // 解析是否是相关操作命令
        match input_str as &str {
            "quit" => break,  
            _ => {}
        };

        match stone_map.parse_step(input_str) {
            Ok(step) => {
                stone_map.make_move(&step);
                stone_map.switch_turn();
            },
            Err(msg) => { print!("{}\n", msg);  }
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
