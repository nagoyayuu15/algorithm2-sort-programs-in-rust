//3値の中央値を求める
fn mid3<T:Ord+Copy>(a: &T,b: &T,c: &T) -> T {
    if (a < b && b < c) || (c < b && b < a) {
        b.clone()
    } else if (b < a && a < c) || (c < a && a < b) {
        a.clone()
    } else {
        c.clone()
    }
}

fn inner_quicksort<T:Ord+Copy>(vector: &mut Vec<T>, start: usize, size: usize) {
    if size <= 1 {
        return;
    }

    // 中央値を選ぶ．
    let pivot = mid3(&vector[start], &vector[start + size/2], &vector[start + size - 1]);

    let mut left = 0;// startからleft個はpivot未満
    let mut pivot_hit = 0;// pivotの個数
    let mut right = 0;// leftからright個はpivotより大きい

    while left + right + pivot_hit < size {
        if vector[start + left] < pivot {
            // startからleft個はpivot未満
            left += 1;
        } else if vector[start + size - 1 - right] > pivot {
            // leftからright個はpivotより大きい
            right += 1;
        } else if vector[start + left] == pivot && vector[start + size - 1 - right] == pivot{
            // 耐pivot重複処理
            // pivot_hitを増やしながら, 左側からpivot以外の値が見つかるまで探索
            // なお，範囲外を対策しないようにleft + right + pivot_hit < sizeとする．
            while left + right + pivot_hit < size && vector[start + (left + pivot_hit)] == pivot {
                pivot_hit += 1;
            }
            // left + pivot_hitがrightの位置を超えていたら終了
            if left + right + pivot_hit >= size {
                break;
            }
            // pivot以外の値がpivotより大きいか小さいかによって左右にくくりだす
            if vector[start + (left + pivot_hit)] < pivot {
                vector.swap(start + left, start + (left + pivot_hit));
                left += 1;
            } else {
                vector.swap(start + size - 1 - right, start + (left + pivot_hit));
                right += 1;
            }
        } else {
            // leftかrightの少なくともどちらか一方をインクリメントできるようにする．
            vector.swap(start + left, start + size - 1 - right);
        }
    }

    inner_quicksort(vector, start, left);    
    inner_quicksort(vector, start+size-right, right);

}

pub fn quicksort<T:Ord+Copy>(vector: &mut Vec<T>) {
    inner_quicksort(vector, 0, vector.len());
}