
use std::collections::HashMap;
use std::cmp;

fn main()-> Result<(), Box<dyn std::error::Error>>{
    let mut amount = String::new();
    println!("入力お願いします :");
    let _= std::io::stdin().read_line(&mut amount);
    // trim()呼ぶこと必要はわかるため10分かかりましたが、学んだことはtrimしないと入力にスペース入らなくてもparseエラーになる
    // let int_amount : i32 = amount.trim().parse::<i32>(); -> 以下のmatch必要ないですが、intバリでション向け追加しました
    match amount.trim().parse::<i32>(){
        Ok(value) => {
            find_stock_values(value);
        }
        Err(_) => {
            println!("入力NGです。評価対象となる金額を入力してください（0～2,147,483,647の整数値）");
        }
    }   
    Ok(())
}


fn find_stock_values(mut amount :i32) -> () {
    println!(" {} 円 分離中...", amount);

    if amount <= 0 {
        print!("{} NGです!!", amount);
        return();
    }
    

    let denominations = [10000, 5000, 1000, 500, 100, 50, 10, 5, 1];

    // HashSetも使えることができると思います: https://doc.rust-lang.org/rust-by-example/std/hash/hashset.html
    let stock = HashMap::from([
       (10000, 1),
       (5000,2),
       (1000, 5),
       (500, 3),
       (100, 10),
       (50, 10),
       (10, 10),
       (5, 10),
       (1, 10),
    ]);

    let mut result:HashMap<i32, i32> = HashMap::new();
    

    for k in denominations{
        let q = amount/k;
        let value = cmp::min(q, stock[&k]);
        amount = amount - (k*value);
        result.insert(k, value);
    }

    if amount == 0 {
        let json_string = serde_json::to_string(&result).unwrap();
        println!("{}", json_string);
        return();
    }

    println!("すみません、このamountはOut of Stockです");

}
