use project_euler_net::{factors_of_n, TriangleNumberIter};

fn main() -> Result<(), String> {
    let tni = TriangleNumberIter::default();

    for t in tni {
        let factor_set = factors_of_n(t);
        if factor_set.len() > 500 {
            dbg!(t);
            return Ok(());
        }
    }

    return Err("You somehow escaped my infinite loop of triangle numbers".into());
}