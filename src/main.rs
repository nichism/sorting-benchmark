

fn main() {
    let static_arr = [3,1,2,5,4];
    let mut arr = static_arr;

    //let first = &arr[0..2];
    //let second = &arr[3..arr.len()];
    //println!("first: {:?}, second: {:?}", first, second);
    do_quick_sort(&mut arr);
    println!("Static array: {:?} ", static_arr);
    println!("Sorted array: {:?}", arr);
}

fn do_quick_sort(arr: &mut [i32]) {
    if arr.len() == 1 {
        return;
    }
    let pivot_index = arr.len()-1;
    let pivot_value = arr[arr.len()-1];

    println!("Pivot value: ");
    let mut current_index = 0;
    let mut swap_index = 0;
    while current_index < arr.len() {
        let val = arr[current_index];
        if val < pivot_value {
            swap_index+=1;
            if current_index > swap_index {
                let temp_current_index_val = arr[current_index];
                let temp_swap_index_val = arr[swap_index];

                arr[current_index] = temp_swap_index_val;
                arr[swap_index] = temp_current_index_val;
                if val == pivot_value && current_index == pivot_index {
                    let len = arr.len();
                    let mut less_array = &mut arr[0..current_index];
                    do_quick_sort(&mut less_array);
                    let mut greater_array = &mut arr[current_index+1..len];
                    do_quick_sort(&mut greater_array);
                }
            }
        }
        
        current_index+=1;
    }
}
