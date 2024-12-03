fn inner_mergesort<T:Ord+Copy>(vector: &mut Vec<T>, start: usize, size: usize) {
    if size <= 1 {
        return;
    }

    // 分割する．
    let left_size = size/2;
    let right_size = size - size/2;
    let pivot = start + left_size; 
    // e.g. 0<start> 1 2<pivot> 3 4
    inner_mergesort(vector, start, left_size);
    inner_mergesort(vector, pivot,right_size);

    // マージする．
    // 参照のままだと上書きされてしまうのでvecとして別の領域に確保
    let buf = vector[start..pivot].to_vec();
    let mut buf_cursor = 0;
    let mut right_cursor = pivot; 

    // place_cursorはright_cursorを追い越すことはない．
    // なぜならば，place_cursorが最もright_cursorに近づくのはbuf_cursorが連続でleft_size回進んだときであるが，
    // この時 place_cursor = pivot = right_cursorであるから．
    for place_cursor in start..(start+size) {
        if buf_cursor == left_size {
            // どちらかのカーソルが進みきっていたらもう片方から選ぶ．
            vector[place_cursor] = vector[right_cursor];
            right_cursor += 1;
        } else if right_cursor == pivot + right_size {
            vector[place_cursor] = buf[buf_cursor];
            buf_cursor += 1;
        } else if buf[buf_cursor] < vector[right_cursor] {
            // さもなくば小さい方を選ぶ．
            vector[place_cursor] = buf[buf_cursor];
            buf_cursor += 1;
        } else {
            vector[place_cursor] = vector[right_cursor];
            right_cursor += 1;
        }
    }
}

pub fn mergesort<T:Ord+Copy>(vector: &mut Vec<T>) {
    inner_mergesort(vector, 0, vector.len());
}