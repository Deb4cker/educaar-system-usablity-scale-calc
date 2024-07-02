fn main() {
    let points_ru_01_1: [i32; 10] = [5, 1, 5, 2, 5, 1, 5, 1, 5, 1];
    let points_ru_01_2: [i32; 10] = [5, 1, 4, 3, 4, 1, 4, 1, 5, 2];

    let points_ru_02_1: [i32; 10] = [5, 1, 5, 2, 5, 1, 5, 1, 5, 1];
    let points_ru_02_2: [i32; 10] = [5, 1, 5, 2, 5, 1, 5, 1, 5, 3];

    let points_ru_03_1: [i32; 10] = [5, 1, 5, 1, 5, 1, 5, 1, 5, 1];
    let points_ru_03_2: [i32; 10] = [5, 1, 5, 2, 5, 1, 5, 1, 5, 2];

    let points_ru_04_1: [i32; 10] = [5, 1, 5, 2, 5, 1, 5, 1, 5, 2];
    let points_ru_04_2: [i32; 10] = [5, 1, 5, 1, 5, 1, 5, 1, 5, 1];

    let sus_ru01_1 = calc_sus(points_ru_01_1);
    let sus_ru01_2 = calc_sus(points_ru_01_2);

    let sus_ru02_1 = calc_sus(points_ru_02_1);
    let sus_ru02_2 = calc_sus(points_ru_02_2);

    let sus_ru03_1 = calc_sus(points_ru_03_1);
    let sus_ru03_2 = calc_sus(points_ru_03_2);

    let sus_ru04_1 = calc_sus(points_ru_04_1);
    let sus_ru04_2 = calc_sus(points_ru_04_2);

    let ru_01_final_result = (sus_ru01_1 + sus_ru01_2) / 2.0;
    let ru_02_final_result = (sus_ru02_1 + sus_ru02_2) / 2.0;
    let ru_03_final_result = (sus_ru03_1 + sus_ru03_2) / 2.0;
    let ru_04_final_result = (sus_ru04_1 + sus_ru04_2) / 2.0;

    println!("RU01: SUS usuario 1 = {}, SUS usuario 2 = {}; media final: {}", sus_ru01_1 , sus_ru01_2, ru_01_final_result);
    println!("RU02: SUS usuario 1 = {}, SUS usuario 2 = {}; media final: {}", sus_ru02_1, sus_ru02_2, ru_02_final_result);
    println!("RU03: SUS usuario 1 = {}, SUS usuario 2 = {}; media final: {}", sus_ru03_1, sus_ru03_2, ru_03_final_result);
    println!("RU04: SUS usuario 1 = {}, SUS usuario 2 = {}; media final: {}", sus_ru04_1, sus_ru04_2, ru_04_final_result);
}

fn calc_sus(points: [i32; 10]) -> f64{
    let mut result = 0;
    let mut sus_values = Vec::new();

    for i in 0..points.len(){
        if (i+1) % 2 == 0 {
            sus_values.push(5 - points[i]); 
        } else {
            
            sus_values.push(points[i] - 1); 
        }
    }

    for num in sus_values{
        result += num;
    }

    result as f64 * 2.5
}
