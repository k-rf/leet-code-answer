struct Solution;

struct Coord {
    x: i32,
    y: i32,
}

impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        for i in 2..coordinates.len() {
            let c = coordinates[i - 2..i + 1]
                .into_iter()
                .map(|e| Coord { x: e[0], y: e[1] })
                .collect::<Vec<Coord>>();

            if (c[1].y - c[0].y) * (c[2].x - c[1].x) != (c[2].y - c[1].y) * (c[1].x - c[0].x) {
                return false;
            }
        }

        true
    }
}
