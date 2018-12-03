use std::fs;
use std::io::{self, BufRead};

struct Claim {
    id: u16,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
}

struct Fabric {
    raw_grid: Vec<u32>,
    dimension: usize,
}

impl Fabric {
    fn new(dimension: usize) -> Fabric {
        Fabric {
            raw_grid: vec![0; dimension * dimension],
            dimension,
        }
    }

    fn claim(&mut self, claim: &Claim) {
        for y in (claim.y)..(claim.y + claim.height) {
            for x in (claim.x)..(claim.x + claim.width) {
                self.raw_grid[y as usize * self.dimension + x as usize] += 1;
            }
        }
    }

    fn overlapping(&self) -> usize {
        self.raw_grid.iter().filter(|&x| *x > 1).count()
    }

    fn claimers(&self, claim: &Claim) -> u32 {
        let mut claims = 0;
        for y in (claim.y)..(claim.y + claim.height) {
            for x in (claim.x)..(claim.x + claim.width) {
                let cell = self.raw_grid[y as usize * self.dimension + x as usize];
                if cell > claims {
                    claims = cell;
                }
            }
        }
        claims
    }

    fn non_overlapping(&self, claims: &[Claim]) -> Option<u16> {
        for claim in claims {
            if self.claimers(claim) == 1 {
                return Some(claim.id);
            }
        }
        None
    }
}

fn parse_claim(claim: &str) -> Claim {
    // #<id> @ <x>,<y>: <width>x<height>
    let mut parts = claim[1..]
        .split(|c| c == '@' || c == ',' || c == ':' || c == 'x' || c == ' ')
        .filter_map(|e| {
            if e.is_empty() {
                None
            } else {
                Some(e.parse().expect("should be a number"))
            }
        });
    Claim {
        id: parts.next().expect("missing id"),
        x: parts.next().expect("missing x"),
        y: parts.next().expect("missing y"),
        width: parts.next().expect("missing width"),
        height: parts.next().expect("missing height"),
    }
}

fn read_input() -> Result<Vec<Claim>, io::Error> {
    let mut buffer = String::new();
    let mut reader = io::BufReader::new(fs::File::open("input")?);
    let mut claims = Vec::new();
    while reader.read_line(&mut buffer)? > 0 {
        claims.push(parse_claim(&buffer.trim_end()));
        buffer.clear();
    }
    Ok(claims)
}

fn main() {
    if let Ok(claims) = read_input() {
        let mut fabric = Fabric::new(1000);
        for claim in &claims {
            fabric.claim(claim);
        }
        println!("square inches: {}", fabric.overlapping());
        match fabric.non_overlapping(&claims) {
            Some(id) => println!("claim id: {:?}", id),
            _ => eprintln!("error: no claim found"),
        };
    } else {
        eprintln!("Cannot parse input");
    }
}

#[test]
fn claim_overlaps() {
    let claims = vec![
        parse_claim("#1 @ 1,3: 4x4"),
        parse_claim("#2 @ 3,1: 4x4"),
        parse_claim("#3 @ 5,5: 2x2"),
    ];
    let mut fabric = Fabric::new(12);
    for claim in &claims {
        fabric.claim(claim);
    }
    assert_eq!(4, fabric.overlapping());
    assert_eq!(Some(3), fabric.non_overlapping(&claims));
}
