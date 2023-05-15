use std::time::Instant;
fn main() {
    
    //let static_arr = [3,1,2,5,4,8,62,-52,55,66,142,-65,-97,-77,-78,20,-600];
    let static_arr = [3,1,2,5,4,-6,-77,50];
    println!("Static array: {:?} ", static_arr);
    let mut arr = static_arr;
    let timer = Instant::now();
    do_quick_sort(&mut arr);
    println!("do_quick_sort took: {:?} nanoseconds.", timer.elapsed().as_nanos());
    println!("Sorted array: {:?}", arr);
    
}

fn do_quick_sort(arr: &mut [i32]) {
    if arr.len() == 1 {
        return;
    }
    println!("do_quick_sort called with arr: {:?} ", arr);
    let pivot_value = arr[arr.len()-1];
    let mut current_index = 0;
    let mut swap_index: isize = -1;
    let mut first_pass: bool = true;
    while current_index < arr.len() {

        if !first_pass {
            current_index+=1;
            if current_index >= arr.len() {
                break;
            }
        }
        
        let current_index_value = arr[current_index];
        println!("comparing ({:?})(idx: {:?}) to pivot_value ({:?})", current_index_value, current_index, pivot_value);
        if current_index_value.lt(&pivot_value) || current_index_value.eq(&pivot_value) {
            println!("current_index_value ({:?})(idx: {:?}) is less than or equal to pivot_value ({:?})", current_index_value, current_index, pivot_value);
            swap_index+=1;
            if current_index > swap_index as usize {
                let temp_current_index_val = arr[current_index];
                let temp_swap_index_val = arr[swap_index as usize];

                arr[current_index] = temp_swap_index_val;
                arr[swap_index as usize] = temp_current_index_val;
                    
                println!("Swapping current index {:?} with swap index {:?} ", current_index, swap_index);
                if current_index == arr.len()-1 {
                    let len = arr.len();
                    println!("arr after swap: {:?}", arr);
                    println!("VALUE IS: {:?}", arr[swap_index as usize]);
                    let mut less_array = &mut arr[0..swap_index as usize];
                    println!("less_array IS: {:?}", less_array);
                    if less_array.len() > 0 {
                        do_quick_sort(&mut less_array);
                    }
                    
                    let mut greater_array = &mut arr[swap_index as usize +1..len];
                    println!("greater_array IS: {:?}", greater_array);
                    if greater_array.len() > 0 {
                        do_quick_sort(&mut greater_array);
                    }
                }
            }
        }
        if first_pass {
            first_pass = false;
        }
    }
}
