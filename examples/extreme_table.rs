use obvious::bruteforce::BruteforceTruthTableBuilder;

// takes 44 seconds on my machine
fn main() {
    let table = BruteforceTruthTableBuilder::build(
        &[
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t",
        ],
        |vars| {
            let a = vars[0].clone();
            let b = vars[1].clone();
            let c = vars[2].clone();
            let d = vars[3].clone();
            let e = vars[4].clone();
            let f = vars[5].clone();
            let g = vars[6].clone();
            let h = vars[7].clone();
            let i = vars[8].clone();
            let j = vars[9].clone();
            let k = vars[10].clone();
            let l = vars[11].clone();
            let m = vars[12].clone();
            let n = vars[13].clone();
            let o = vars[14].clone();
            let p = vars[15].clone();
            let q = vars[16].clone();
            let r = vars[17].clone();
            let s = vars[18].clone();
            let t = vars[19].clone();

            a.implies(&b)
                .implies(&c)
                .implies(&d)
                .implies(&e)
                .implies(&f)
                .implies(&g)
                .implies(&h)
                .implies(&i)
                .implies(&j)
                .implies(&k)
                .implies(&l)
                .implies(&m)
                .implies(&n)
                .implies(&o)
                .implies(&p)
                .implies(&q)
                .implies(&r)
                .implies(&s)
                .implies(&t)
        },
    )
    .unwrap();
    println!("{}", table);
}
