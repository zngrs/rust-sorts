#![allow(unused)]
extern crate rand;
use rand::Rng;
use std::{
    array,
    collections::HashMap,
    io, str, thread,
    time::{self, Instant},
};

fn main() {
    let mut random_vec = Vec::new();

    let l = 120;

    let max = 100;
    let do_graph = true;
    for _ in 0..l {
        random_vec.push(rand::thread_rng().gen_range(0..max + 1));
    }

    let mut sort_method = String::new();

    io::stdin()
        .read_line(&mut sort_method)
        .expect("failed to read");
    random_vec.push(9);
    println!("{:?}", random_vec);
    println!("{}", get_digit(9, 0));

    let real_sort_test_vec = random_vec.clone();

    if do_graph {
        match sort_method.trim() {
            "bubble" => println!("{:?}", bubble_sort(random_vec, true)),
            "radix" => println!("{:?}", radix_sort(random_vec, 0, true)),
            "comb" => println!("{:?}", comb_sort(random_vec, true)),
            "merge" => print!("{:?}", merge_sort(random_vec, true)),
            _ => {}
        }
    }

    let now = Instant::now();
    match sort_method.trim() {
        "bubble" => println!("{:?}", bubble_sort(real_sort_test_vec, false)),
        "radix" => println!("{:?}", radix_sort(real_sort_test_vec, 0, false)),
        "comb" => println!("{:?}", comb_sort(real_sort_test_vec, false)),
        "merge" => print!("{:?}", merge_sort(real_sort_test_vec, true)),
        _ => {}
    }

    let elapsed = now.elapsed();

    println!(
        "Time taken to complete {} sort: {:.2} ms, or {:.2} seconds",
        sort_method.trim(),
        elapsed.as_secs_f64() * 1000.0 + f64::from(elapsed.subsec_nanos()) / 1_000_000.0, // as milliseconds
        elapsed.as_secs_f64() + f64::from(elapsed.subsec_nanos()) / 1_000_000_000.0 // as seconds
    );
}

fn comb_sort(mut a: Vec<i32>, do_graph: bool) -> Vec<i32> {
    let mut gap: f32 = a.len() as f32;
    let shrink: f32 = 1.3;
    let mut sorted: bool = false;

    while !sorted || gap > 1.0 {
        gap = (gap / shrink).floor();
        if gap < 1.0 {
            gap = 1.0;
        }

        let mut i = 0;
        sorted = true;

        while ((i as f32 + gap) as usize) < a.len() {
            if do_graph {
                graph(a.clone());
                thread::sleep(time::Duration::from_millis(10));
            }

            if a[i as usize] > a[(i as f32 + gap) as usize] {
                a.swap(i as usize, (i as f32 + gap) as usize);
                sorted = false;
            }
            i += 1;
        }
    }

    a
}

fn bubble_sort(a: Vec<i32>, do_graph: bool) -> Vec<i32> {
    let mut returned = a;
    let l = returned.len();
    for i in 0..l - 1 {
        for j in 0..l - i - 1 {
            if returned[j] > returned[j + 1] {
                returned.swap(j, j + 1);
                if do_graph {
                    graph(returned.clone());

                    thread::sleep(time::Duration::from_millis(10));
                }
            }
        }
    }
    return returned;
}

fn radix_sort(mut a: Vec<i32>, d: i32, do_graph: bool) -> Vec<i32> {
    println!("after sort {}:", d + 1);
    println!("{:?}", a);
    if do_graph {
        graph(a.clone());
        thread::sleep(time::Duration::from_millis(10));
    }

    let mut nums = a.clone();
    let mut bins: HashMap<i32, Vec<i32>> = HashMap::new();
    for digit in 0..=9 {
        bins.insert(digit, Vec::new());
    }

    let mut max_digits: i32 = 0;

    for &num in &nums {
        if get_digits(num) > max_digits {
            max_digits = get_digits(num);
        }
    }

    for &num in &nums {
        let digit = get_digit(num, d);
        bins.get_mut(&digit).unwrap().push(num);
    }

    let mut returned: Vec<i32> = Vec::new();

    for digit in 0..=9 {
        returned.extend(bins.get(&digit).unwrap().iter().cloned());
    }

    if max_digits - 1 < d {
        println!("Final result:");
        return nums;
    }

    return radix_sort(returned, d + 1, do_graph);
}

fn merge_sort(arr: Vec<i32>, do_graph: bool) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let middle = arr.len() / 2;
    let left = merge_sort(arr[..middle].to_vec(), do_graph);
    let right = merge_sort(arr[middle..].to_vec(), do_graph);
    let sorted = merge(left, right);

    if do_graph {
        let mut max_length: i32 = 0;
        let scale_factor: i32 = 10;
        let vec = sorted.clone();
        for el in &sorted {
            if *el > max_length {
                max_length = *el;
            }
        }
        let width = vec.len();
        print!("{}[2J", 27 as char); // Clear terminal

        for i in (0..=max_length / scale_factor).rev() {
            for num in &vec {
                if *num >= i * scale_factor {
                    print!("# ");
                } else {
                    print!("  ");
                }
            }
            println!();
        }

        thread::sleep(time::Duration::from_millis(10));
    }

    sorted
}

fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        result.push(left[i]);
        i += 1;
    }
    while j < right.len() {
        result.push(right[j]);
        j += 1;
    }

    result
}

fn get_max(a: Vec<i32>) -> i32 {
    let mut max: i32 = a[0];
    for i in a {
        if i > max {
            max = i;
        }
    }
    return max;
}

fn get_digits(n: i32) -> i32 {
    let num = n.abs(); // Take the absolute value to handle negative numbers

    if num / 10 == 0 {
        return 1;
    }
    return 1 + get_digits(num / 10);
}

fn get_digit(n: i32, d: i32) -> i32 {
    let divisor = if d == 0 { 1 } else { 10_i32.pow(d as u32) };
    (n / divisor) % 10
}

fn graph(v: Vec<i32>) {
    let mut max_length: i32 = 0;
    let scale_factor: i32 = 10;
    let vec = v.clone();
    for el in &v {
        if *el > max_length {
            max_length = *el;
        }
    }
    let width = vec.len();
    print!("{}[2J", 27 as char); // Clear terminal

    for i in (0..=max_length / scale_factor).rev() {
        for num in &vec {
            if *num >= i * scale_factor {
                print!("# ");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}