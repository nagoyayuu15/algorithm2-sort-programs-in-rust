pub fn shellsort<T:Ord>(vector: &mut Vec<T>) {
    // 増分列 2^k - 1とする
    if vector.len() <= 1 {
        return;
    }

    // 初めの増分
    let mut k = vector.len().ilog2();
    let mut gap = 2usize.pow(k) - 1;

    while gap > 0 {
        // 増分を小さくしながら
        for group in 0..gap { // 増分の剰余でグルーピング
            // グループごとに挿入ソート
            for i in (group..vector.len()).step_by(gap) {
                let mut j = i;
                while j >= gap && vector[j - gap] > vector[j] {
                    vector.swap(j - gap, j);
                    j -= gap;
                }
            }
        }
        k -= 1;
        gap = 2usize.pow(k) - 1;
    }
}