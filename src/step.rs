use crate::stone::Stone;
use crate::StoneMap;

#[derive(Clone, Copy)]
pub struct Step {
    pub from: (usize, usize),
    pub to: (usize, usize),
    pub killed: Option<Stone>,
}

impl Step {
    pub fn new(from: (usize, usize), to: (usize, usize), killed: Option<Stone>) -> Self {
        Step { from, to, killed }
    }

    pub fn parse(stone_map: &mut StoneMap, input: &str) -> Result<Self, String> {
        let chars: Vec<char> = input.trim().chars().collect();
        // 获取棋子位置
        let (x, y) = Step::get_loaction(stone_map, &chars)?;

        // 解析棋子移动 进、退、平
        todo!()
    }

    // 私有辅助函数
    fn get_loaction(stone_map: &mut StoneMap, input: &[char]) -> Result<(usize, usize), String> {
        if input.len() < 4 {
            return Err(String::from("输入字符串过短"));
        }

        match input[0] {
            '将' | '帅' | '將' | '帥' | '王' => Step::get_king_location(stone_map, input),
            '车' | '伡' | '車' | '俥' => Step::get_rook_location(stone_map, input),
            '炮' | '砲' => Step::get_cannon_location(stone_map, input),
            '兵' | '卒' => Step::get_pawn_location(stone_map, input),
            '士' | '仕' => Step::get_mandarin_location(stone_map, input),
            '象' | '相' => Step::get_bishop_location(stone_map, input),
            '馬' | '傌' | '马' | '㐷' => Step::get_knight_location(stone_map, input),
            '前' | '后' | '後' | '中' | '二' | '三' | '四' => match input[1] {
                '车' | '伡' | '車' | '俥' => Step::get_rook_location(stone_map, input),
                '炮' | '砲' => Step::get_cannon_location(stone_map, input),
                '兵' | '卒' => Step::get_pawn_location(stone_map, input),
                '士' | '仕' => Step::get_mandarin_location(stone_map, input),
                '象' | '相' => Step::get_bishop_location(stone_map, input),
                '馬' | '傌' | '马' | '㐷' => Step::get_knight_location(stone_map, input),
                '一' | '壹' | '1' | '１' | '二' | '贰' | '2' | '２' | '三' | '叁' | '3' | '３'
                | '四' | '肆' | '4' | '４' | '五' | '伍' | '5' | '５' | '六' | '陆' | '6'
                | '６' | '七' | '柒' | '7' | '７' | '八' | '捌' | '8' | '８' | '九' | '玖'
                | '9' | '９' => Step::get_pawn_location(stone_map, input),
                _ => Err(format!("未知棋子 `{}`", input[1])),
            },
            _ => Err(format!("未知棋子 `{}`", input[0])),
        }
    }
    fn get_king_location(
        stone_map: &mut StoneMap,
        input: &[char],
    ) -> Result<(usize, usize), String> {
        // 将只有一个，不用管前后
        let col = Step::char_to_number(input[1])?;
        let stones = match stone_map.turn {
            crate::stone::Camp::Up => &stone_map.up_stones,
            crate::stone::Camp::Down => &stone_map.down_stones,
        };

        // 看看将是否还活着
        
        todo!()
    }
    fn get_mandarin_location(
        stone_map: &mut StoneMap,
        input: &[char],
    ) -> Result<(usize, usize), String> {
        todo!()
    }
    fn get_bishop_location(
        stone_map: &mut StoneMap,
        input: &[char],
    ) -> Result<(usize, usize), String> {
        todo!()
    }
    fn get_knight_location(
        stone_map: &mut StoneMap,
        input: &[char],
    ) -> Result<(usize, usize), String> {
        todo!()
    }
    fn get_rook_location(
        stone_map: &mut StoneMap,
        input: &[char],
    ) -> Result<(usize, usize), String> {
        todo!()
    }
    fn get_cannon_location(
        stone_map: &mut StoneMap,
        input: &[char],
    ) -> Result<(usize, usize), String> {
        todo!()
    }
    fn get_pawn_location(
        stone_map: &mut StoneMap,
        input: &[char],
    ) -> Result<(usize, usize), String> {
        todo!()
    }

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
            _ => Err(format!("无法将字符 `{}` 解析为数字.", c)),
        }
    }
}
