use std::array::from_fn;

use crate::{mask::Mask, piece::Direction, square::Square};

use magic_gen::create_move_list;

const ROOK_INDEX_BITS: u8 = 16;
const ROOK_MAGICS: &[u64; 64] = &[
    0x002d00c004884001,
    0x0104010051000400,
    0x0041020004112400,
    0x08057004a0009180,
    0x0800c06000480002,
    0x1140400044841081,
    0x5820200004a5040a,
    0x0650010092002004,
    0x008010c005412001,
    0x00044000c30a0004,
    0x5186c03030814080,
    0x8020202480040008,
    0x2028480200098800,
    0xa210828141180200,
    0x2011010410200044,
    0x0000800070028a05,
    0x0048140041421820,
    0x0c010a0200100008,
    0x0120012400820223,
    0x010b01800a808200,
    0xa000200286000086,
    0x0001082408002102,
    0x0020426112641200,
    0x1020000400090610,
    0x0810021080204280,
    0x0800040820360108,
    0xc030010030060102,
    0x4000808045050410,
    0x80985005a0050044,
    0x8800102010028b20,
    0x4142000058004809,
    0x0080820649600108,
    0x0c00404111020104,
    0x0c08c02000021800,
    0x4008001884028441,
    0x0004d09000900a20,
    0x0004010040082200,
    0x0000204c00310710,
    0x0240024100118402,
    0x00103408101202ca,
    0x1200200448d20002,
    0x1001000c00121040,
    0x2006028100020e00,
    0x0803520200030028,
    0x08c0240030020001,
    0x8420201010802900,
    0x0000428800296207,
    0xa010008000080608,
    0x2d02208150000440,
    0x1c402120040020c0,
    0x1000004042200420,
    0x0324290004100746,
    0x008200040000102a,
    0x0000400095300082,
    0x2020c02140802803,
    0x0886801880041004,
    0xa000880c01000841,
    0x0102048901404222,
    0x020c004010020081,
    0x10140128200a0082,
    0x0082308590204402,
    0x1340082940900802,
    0x0300901000301413,
    0x02000082c2100402,
];

const BISHOP_INDEX_BITS: u8 = 14;
const BISHOP_MAGICS: &[u64; 64] = &[
    0x4480002040000844,
    0x0120670091028084,
    0x080280010a08040a,
    0x0004022014418101,
    0x1986210100008c01,
    0x404005003a104840,
    0x0601224800180240,
    0x0420300044000888,
    0x0500061188013409,
    0x0008010880820200,
    0x0120008c524841a1,
    0x14040d0101104404,
    0x0102000060490208,
    0x0002220128800320,
    0x040d031100108420,
    0x80448a0200521080,
    0x24122a42c4112000,
    0x0208016400c001c0,
    0x80000300c0800800,
    0x101000080004a003,
    0x0922008008040800,
    0x000a004200010180,
    0x00005052200b8004,
    0x0401301009202620,
    0x0201008111004000,
    0x000005040200a000,
    0x1820080200040120,
    0x0048426008000490,
    0x0248020040400880,
    0x00060000240009c0,
    0x4c40200402040810,
    0x1000084100a00120,
    0x000a200080804080,
    0x442a002022100128,
    0x483a20194a400c03,
    0x0002001802010820,
    0x420008082030a10c,
    0x4008120020880400,
    0x0000080141409012,
    0x0480100044690140,
    0x0002541102092800,
    0x01020b002003040a,
    0x2020408004201800,
    0x48c101102110a194,
    0x7000100110824800,
    0x4090c004c8814200,
    0x7843402808081818,
    0x201000802200420c,
    0x2520080430082420,
    0x8002000240420208,
    0x40082801040a00b0,
    0x0000020026000811,
    0x0000089004000210,
    0x0c00104000262030,
    0x0006002001004820,
    0x01180a04000800b2,
    0x0000100866008842,
    0x4001800c20400191,
    0x0200010004200041,
    0x0000518206110542,
    0x0208830046001082,
    0x000a00004099810a,
    0x0203018081642094,
    0x8201006504018212,
];

#[derive(Debug)]
pub struct MoveGen {
    orthogonal_magics: Vec<MagicEntry>,
    diagonal_magics: Vec<MagicEntry>,
    orthogonal_moves: Vec<Vec<Mask>>,
    diagonal_moves: Vec<Vec<Mask>>,
}

impl MoveGen {
    pub fn init() -> Self {
        let orthogonal_magics = Vec::from_iter::<[MagicEntry; 64]>(from_fn(|i| MagicEntry {
            mask: Square::from_usize(i).unwrap().mask(),
            magic: ROOK_MAGICS[i],
            index_bits: ROOK_INDEX_BITS,
        }));

        let diagonal_magics = Vec::from_iter::<[MagicEntry; 64]>(from_fn(|i| MagicEntry {
            mask: Square::from_usize(i).unwrap().mask(),
            magic: BISHOP_MAGICS[i],
            index_bits: BISHOP_INDEX_BITS,
        }));

        let orthogonal_moves = create_move_list(Direction::Orthogonal, &orthogonal_magics);

        let diagonal_moves = create_move_list(Direction::Diagonal, &diagonal_magics);

        Self {
            orthogonal_magics,
            diagonal_magics,
            orthogonal_moves,
            diagonal_moves,
        }
    }

    pub fn get_rook_moves(&self, square: Square, blockers: Mask) -> Mask {
        let magic = &self.orthogonal_magics[square as usize];
        let moves = &self.orthogonal_moves[square as usize];

        moves[magic.index(blockers)]
    }

    pub fn get_bishop_moves(&self, square: Square, blockers: Mask) -> Mask {
        let magic = &self.diagonal_magics[square as usize];
        let moves = &self.diagonal_moves[square as usize];

        moves[magic.index(blockers)]
    }
}

#[derive(Debug, Clone)]
pub struct MagicEntry {
    pub mask: Mask,
    pub magic: u64,
    pub index_bits: u8,
}

impl MagicEntry {
    pub fn index(&self, blockers: Mask) -> usize {
        let blockers = blockers & self.mask;
        let mul = blockers.0.wrapping_mul(self.magic);

        (mul >> (64 - self.index_bits)) as usize
    }
}

#[allow(unused)]
mod magic_gen {
    use rand::{thread_rng, Rng};

    use crate::{mask::Mask, piece::Direction, square::Square};

    use super::MagicEntry;

    fn random_u64() -> u64 {
        thread_rng().gen()
    }

    fn generate_magic(
        direction: Direction,
        square: Square,
        index_bits: u8,
    ) -> (MagicEntry, Vec<Mask>) {
        let mask = direction.all_blocker_subsets()[square as usize][0];

        loop {
            let magic = random_u64() & random_u64() & random_u64();
            let new_entry = MagicEntry {
                mask,
                magic,
                index_bits,
            };

            if let Ok(table) = try_fill_magic_table(direction, &new_entry, square) {
                return (new_entry, table);
            }
        }
    }

    #[derive(Debug)]
    struct TableFillError;

    fn try_fill_magic_table(
        direction: Direction,
        entry: &MagicEntry,
        square: Square,
    ) -> Result<Vec<Mask>, TableFillError> {
        let mut table = vec![Mask(0); 1 << entry.index_bits];

        for blockers in entry.mask.subsets() {
            let moves = direction.moves_for(square, blockers);
            let new_entry = &mut table[entry.index(blockers)];

            if new_entry.0 == 0 {
                *new_entry = moves;
            } else if *new_entry != moves {
                return Err(TableFillError);
            }
        }

        Ok(table)
    }

    fn create_magics(direction: Direction, index_bits: u8) -> (Vec<MagicEntry>, Vec<Vec<Mask>>) {
        let mut magics: Vec<MagicEntry> = Vec::with_capacity(64);
        let mut masks: Vec<Vec<Mask>> = Vec::with_capacity(64);

        for i in 0..64 {
            let square = Square::from_usize(i).unwrap();

            let (new_magics, new_masks) = generate_magic(direction, square, index_bits);
            magics.push(new_magics);
            masks.push(new_masks);
        }

        (magics, masks)
    }

    pub fn create_move_list(direction: Direction, magics: &[MagicEntry]) -> Vec<Vec<Mask>> {
        let mut moves = Vec::with_capacity(64);

        for (i, magic) in magics.into_iter().enumerate() {
            let move_table =
                try_fill_magic_table(direction, magic, Square::from_usize(i).unwrap()).unwrap();
            moves.push(move_table);
        }

        moves
    }

    #[cfg(test)]
    pub mod magic_gen_tests {
        use std::fs;

        use crate::{board_helper::BoardHelper, piece::Direction};

        use super::*;

        #[test]
        fn debug_magic_generation() {
            let direction = Direction::Orthogonal;
            let square = Square::A1;
            let index_bits = 16;
            let (magic, _) = generate_magic(direction, square, index_bits);

            dbg!(magic);
        }

        #[test]
        fn debug_magic_index_usage() {
            let direction = Direction::Diagonal;
            let square = Square::A1;
            let index_bits = 16;

            let (magic, moves) = generate_magic(direction, square, index_bits);

            let blockers = Mask(random_u64() & random_u64());

            println!("Blockers:");
            BoardHelper::print_mask(&blockers);
            println!();

            println!("Relevant blockers:");
            BoardHelper::print_mask(&(blockers & direction.all_blockers()[square as usize]));
            println!();

            println!("Moves from A1:");
            BoardHelper::print_mask(&moves[magic.index(blockers)]);
        }

        #[test]
        fn create_rook_magics() -> std::io::Result<()> {
            let rook_index_bits = 16;
            let (rook_magics, _) = create_magics(Direction::Orthogonal, rook_index_bits);

            let mut buf = String::new();

            for (i, magic) in rook_magics.into_iter().enumerate() {
                buf.push_str(&format!("{:#018x}", magic.magic));

                if i != 63 {
                    buf.push('\n');
                }
            }

            fs::create_dir_all("build")?;
            fs::write(format!("build/rook_magics_{}.txt", rook_index_bits), buf)?;

            Ok(())
        }

        #[test]
        fn create_bishop_magics() -> std::io::Result<()> {
            let bishop_index_bits = 14;
            let (bishop_magics, _) = create_magics(Direction::Orthogonal, bishop_index_bits);

            let mut buf = String::new();

            for (i, magic) in bishop_magics.into_iter().enumerate() {
                buf.push_str(&format!("{:#018x}", magic.magic));

                if i != 63 {
                    buf.push('\n');
                }
            }

            fs::create_dir_all("build")?;
            fs::write(
                format!("build/bishop_magics_{}.txt", bishop_index_bits),
                buf,
            )?;

            Ok(())
        }
    }
}
