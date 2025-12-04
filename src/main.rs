fn main() {
    let input = include_str!("input.txt");

    let mut count = 0;
    let mut dial = 50;
    for l in input.lines(){
        println!("in {l} with {dial}");

        let (dir,rest) = l.split_at(1);
        let amount : u32 = rest.parse().unwrap();
        
        let full_spins = amount/100;
        if full_spins>0{
            println!("adding {} because of full spins",amount/100);
        }

        count += full_spins;
        let amount = amount%100;

        match dir {
            "L"=>{
                if amount>=dial && dial!=0{
                    println!("looped neg");
                    count+=1;
                }
                dial=(dial+100-amount)%100;

            },
            "R"=>{
                if amount>=100-dial && dial!=0{
                    println!("looped pos");
                    count+=1;
                }
                dial=(dial+amount)%100;
            },
            r=>panic!("unexpected dir {r}"),
        };


    }

    println!("count is {count}", );
}
