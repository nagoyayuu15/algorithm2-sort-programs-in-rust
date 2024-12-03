pub fn combsort<T:Ord>(vector: &mut Vec<T>) {
    let mut gap = vector.len();
    let shrink = 1.3;
    let mut sorted = false;
    while !sorted {
        gap = (gap as f64 / shrink) as usize;
        // 11ルール
        if gap == 9 || gap == 10 {
            gap = 11;
        }
        // 1未満の場合は1に
        if gap <= 1 {
            gap = 1;
            // sorted=trueがif分の中にあることについて：
            // gapが1でないときに入れ替えが起こらなかったとしても，ソートは完了していないので注意が必要
            sorted = true;
        }
        // ストローク
        for i in 0..(vector.len() - gap) {
            if vector[i] > vector[i + gap] {
                vector.swap(i, i + gap);
                sorted = false;
            }
        }
    }
}