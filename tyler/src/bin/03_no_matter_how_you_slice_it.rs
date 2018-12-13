extern crate adventofcode;
extern crate regex;

use regex::Regex;

trait IsOn {
    fn is_on(&self, coord_x: u16, coord_y: u16) -> bool;
}

trait Overlaps {
    fn overlaps(&self, coord_x: u16, coord_y: u16, width: u16, height: u16) -> bool;
}

#[derive(Copy, Clone)]
struct FabricPlan {
    id: u16,
    indent_x: u16,
    indent_y: u16,
    height: u16,
    width: u16,
}

impl IsOn for FabricPlan {
    fn is_on(&self, coord_x: u16, coord_y: u16) -> bool {
        if coord_x >= self.indent_x && coord_x < self.indent_x + self.width
            && coord_y >= self.indent_y && coord_y < self.indent_y + self.height {
                return true;
            }

        false
    }
}

impl Overlaps for FabricPlan {
    fn overlaps(&self, coord_x: u16, coord_y: u16, width: u16, height: u16) -> bool {
        for x in coord_x..coord_x + width {
            for y in coord_y..coord_y + height {
                if self.is_on(x, y) {
                    return true;
                }
            }
        }

        false
    }
}

fn get_plans_from_input(input: String) -> Vec<FabricPlan> {
    // Create an empty vector for fabric plans.
    let mut fabric_plans: Vec<FabricPlan> = Vec::new();

    // Create a regex for the input.
    let re = Regex::new(r"#(\d{1,4}) @ (\d{1,3}),(\d{1,3}): (\d{1,2})x(\d{1,2})").unwrap();

    // Iterate over input that matches the regex.
    for cap in re.captures_iter(&input) {
        // Add the fabric plan to the collection of fabric plans.
        fabric_plans.push(FabricPlan {
            id: cap[1].parse::<u16>().unwrap(),
            indent_x: cap[2].parse::<u16>().unwrap(),
            indent_y: cap[3].parse::<u16>().unwrap(),
            height: cap[5].parse::<u16>().unwrap(),
            width: cap[4].parse::<u16>().unwrap()
        });
    }

    fabric_plans
}

fn find_overlapping_area(fabric_plans: Vec<FabricPlan>) -> String {
    // Area covered by two or more elf plans.
    let mut area_covered = 0;

    // Iterate over the width of the fabric.
    for x in 0..1000 {
        // Debug.
        println!("X: {}", x);

        // Iterate over the height of the fabric.
        for y in 0..1000 {
            // Number of plans that overlap on given x,y coordinate.
            let mut overlap_count = 0;

            // Iterate over the plans.
            for i in 0..fabric_plans.len() {
                // Check if the plan covers the current coordinate.
                if fabric_plans[i].is_on(x, y) {
                    // Increment the number of plans currently covering the coordinate.
                    overlap_count += 1;

                    // Check if more than one plan covers the current coordinate.
                    if overlap_count > 1 {
                        // Increment the area covered.
                        area_covered += 1;

                        // Break out of current for loop.
                        break;
                    }
                }
            }
        }
    }

    // Return square inches of overlapping plans.
    area_covered.to_string() 
}

fn find_unique_plan(fabric_plans: Vec<FabricPlan>) -> String {

    for i in 0..fabric_plans.len() {
        let mut overlap: bool = false;
        for oi in 0..fabric_plans.len() {
            if oi == i { continue; }
            if fabric_plans[oi].overlaps(
                fabric_plans[i].indent_x,
                fabric_plans[i].indent_y,
                fabric_plans[i].width,
                fabric_plans[i].height,
            ) {
                overlap = true;
            }
        }
        if !overlap {
            return fabric_plans[i].id.to_string();
        }
    }

    "Nothing".to_string()

    // if non_overlapping_plans.len() > 1 {
    //     println!("Number of plans not overlapping: {}", non_overlapping_plans.len());
    // } else {
    //     return non_overlapping_plans[0].id.to_string();
    // }

    // "Something went wrong, we have more than one unique plan!".to_string()
}

fn main() {
    let input = adventofcode::read_input_file(3);
    let fabric_plans = get_plans_from_input(input);
    // println!("{}", find_overlapping_area(fabric_plans));
    println!("{}", find_unique_plan(fabric_plans));
}
