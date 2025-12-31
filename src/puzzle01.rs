mod dial;

use std::io;

pub fn run() {
    let mut counter = 0;
    let mut dial = dial::Dial::new(50);
    let lines = io::stdin().lines();
    for line in lines {
        let (kind, rotation) = parse_rotation(&line.unwrap().trim());
        dial.rotate(kind, rotation);
        if dial.position() == 0 {
            counter += 1;
        }
    }
    println!("The counter value is {counter}");
}

fn parse_rotation(line: &str) -> (dial::RotationKind, u16) {
    let mut chars = line.chars();
    let kind = match chars.next().unwrap() {
        'R' => dial::RotationKind::Right,
        'L' => dial::RotationKind::Left,
        _ => panic!("The first character must be either R or L!"),
    };
    let rotation = match line[1..].parse::<u16>() {
        Ok(rotation) => rotation,
        Err(e) => {
            panic!("Could not parse rotation: {}", e);
        }
    };
    (kind, rotation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rotation_left() {
        let line = "L89";
        let (kind, rotation) = parse_rotation(line);
        assert_eq!(kind, dial::RotationKind::Left);
        assert_eq!(rotation, 89);
    }

    #[test]
    fn parse_rotation_right() {
        let line = "R1000";
        let (kind, rotation) = parse_rotation(line);
        assert_eq!(kind, dial::RotationKind::Right);
        assert_eq!(rotation, 1000);
    }

    #[test]
    #[should_panic(expected = "first character must be")]
    fn parse_rotation_panic() {
        let line = "100";
        parse_rotation(line);
    }
}
