fn main() {
    let input = include_str!("input.txt");

    let mut count = 0;
    let mut dial = 50;
    for l in input.lines(){
        let (dir,rest) = l.split_at(1);
        let amount : u32 = rest.parse().unwrap();
        let amount = amount%100;

        dial=match dir {
            "L"=>(dial+100-amount)%100,
            "R"=>(dial+amount)%100,
            r=>panic!("unexpected dir {r}"),
        };

        if dial==0{
            count+=1;
        }
    }

    println!("count is {count}", );
}
