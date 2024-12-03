
// 昇順ソートにするため，逆のヒープ条件を考える（親が子より大きい．）
fn heapify<T:Ord>(vector: &mut Vec<T>, target_idx: usize, heap_size: usize) {
    let mut child_idx = 2 * target_idx + 1; // 左の子
    
    if child_idx >= heap_size {
        return;
    } // 子が無い（範囲外）なら終わり

    if child_idx + 1 < heap_size && vector[child_idx] < vector[child_idx + 1] {
        child_idx = child_idx + 1;
    } // 右の子の方があって，かつ右の子のほうが大きいならそちらを選ぶ．

    // 親より子のほうが大きいなら交換して局所的にヒープ条件を満たす．
    if vector[target_idx] < vector[child_idx] {
        vector.swap(target_idx, child_idx);
        // 交換先でもヒープ条件を満たすように再帰呼出し．
        heapify(vector, child_idx, heap_size);
    }
}

fn heapify_whole<T:Ord>(vector: &mut Vec<T>) {
    // 下から順にヒープ化していく．
    for i in (0..vector.len()).rev() {
        heapify(vector, i, vector.len());
    }
}

fn extract_heap<T:Ord>(vector: &mut Vec<T>) {
    for heap_size in (1..vector.len()).rev() {
        // ヒープからpop(そのままヒープの範囲外へ) indexがサイズと同じならそれは範囲外なことに注意．
        vector.swap(0, heap_size);
        heapify(vector, 0, heap_size);
    }
}

pub fn heapsort<T:Ord>(vector: &mut Vec<T>){
    heapify_whole(vector);
    extract_heap(vector);
}