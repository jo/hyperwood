use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fmt::Write;
use std::io::{BufRead, BufReader, Read};

static HEADER_LINES: usize = 6;

// A variant defines a slat in its 3 dimensions
#[derive(Debug, Serialize, Deserialize)]
pub struct Variant {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Variant {
    fn default() -> Variant {
        Variant {
            x: 1.0,
            y: 1.0,
            z: 1.0,
        }
    }
}

impl Variant {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

// A slat is the building block of a rendering
#[derive(Debug)]
pub struct Slat {
    pub name: String,
    pub layer: isize,
    pub origin: Point,
    pub vector: Vector,
}

impl Slat {
    pub fn from_hef_line(line: String, name: String) -> Self {
        let mut origin_parts: Vec<&str> = line.rsplitn(5, " ").collect();
        let origin_part = origin_parts.pop().unwrap();
        let layer: isize = origin_parts[0].parse().unwrap();
        let origin = Point::from_hef_part(origin_part.to_string());
        let mut vector_parts: Vec<&str> = line.splitn(4, " ").collect();
        let vector_part = vector_parts.pop().unwrap();
        let vector = Vector::from_hef_part(vector_part.to_string());
        Self {
            name,
            layer,
            origin,
            vector,
        }
    }

    pub fn to_hef_line(&self) -> String {
        format!(
            "{:} {:} {:}",
            self.origin.to_hef_part(),
            self.vector.to_hef_part(),
            self.layer
        )
    }

    pub fn to_bom_line(&self, variant: &Variant) -> String {
        format!("{:} {:} {:}", self.length(&variant), self.layer, self.name)
    }

    pub fn length(&self, variant: &Variant) -> f32 {
        self.vector.length(variant) + self.vector.unit().length(variant)
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point {
    pub fn from_hef_part(part: String) -> Self {
        let parts: Vec<f32> = part.split(' ').map(|no| no.parse().unwrap()).collect();

        Self {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }

    pub fn to_hef_part(&self) -> String {
        format!("{:} {:} {:}", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    pub fn from_hef_part(part: String) -> Self {
        let parts: Vec<f32> = part.split(' ').map(|no| no.parse().unwrap()).collect();

        Self {
            x: parts[0],
            y: parts[1],
            z: parts[2],
        }
    }

    pub fn to_hef_part(&self) -> String {
        format!("{:} {:} {:}", self.x, self.y, self.z)
    }

    pub fn length(&self, variant: &Variant) -> f32 {
        ((self.x * variant.x).powf(2.0)
            + (self.y * variant.y).powf(2.0)
            + (self.z * variant.z).powf(2.0))
        .sqrt()
    }

    pub fn unit(&self) -> Vector {
        let length = self.length(&Variant::default());

        Vector {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

pub struct Model<T, F> {
    pub parameters: T,
    pub properties: F,
    pub name: String,
    pub variant: Variant,
    pub slats: Vec<Slat>,
}

impl<
    T: Default + Serialize + for<'a> Deserialize<'a>,
    F: Default + Serialize + for<'a> Deserialize<'a>,
> Model<T, F>
{
    pub fn from_hef<R: Read>(reader: R) -> Self
    where
        Self: Sized,
    {
        let hef = BufReader::new(reader);

        let mut name = format!("unknown");
        let mut parameters = T::default();
        let mut properties = F::default();
        let mut variant = Variant::default();
        let mut slats = vec![];

        let mut number_of_parts: Option<usize> = None;
        let mut parts = HashMap::new();
        for (i, line) in hef.lines().enumerate() {
            if i < 3 {
                continue;
            } else if i == 3 {
                match line {
                    Ok(ref line) => name = line.clone(),
                    _ => {}
                }
            } else if i == 4 {
                match line {
                    Ok(ref line) => {
                        parameters =
                            serde_json::from_str(line).expect("could not parse parameters line")
                    }
                    _ => {}
                }
            } else if i == 5 {
                match line {
                    Ok(ref line) => {
                        variant = serde_json::from_str(line).expect("could not parse variant line")
                    }
                    _ => {}
                }
            } else if i == 6 {
                match line {
                    Ok(ref line) => {
                        properties =
                            serde_json::from_str(line).expect("could not parse properties line")
                    }
                    _ => {}
                }
            } else if i == HEADER_LINES + 1 {
                match line {
                    Ok(ref line) => number_of_parts = Some(line.parse().unwrap()),
                    _ => {}
                }
            } else if i > HEADER_LINES + 1 && i <= HEADER_LINES + 1 + number_of_parts.unwrap() {
                match line {
                    Ok(ref line) => {
                        parts.insert(i - HEADER_LINES - 2, line.clone());
                    }
                    _ => {}
                }
            } else if i > HEADER_LINES + 1 + number_of_parts.unwrap() {
                match line {
                    Ok(ref line) => {
                        let (slat_line, part_index) = line.rsplit_once(" ").unwrap();
                        let part = parts.get(&part_index.parse().unwrap()).unwrap();
                        let slat = Slat::from_hef_line(slat_line.to_string(), part.to_string());
                        slats.push(slat);
                    }
                    _ => {}
                }
            }
        }

        Self {
            parameters,
            properties,
            name,
            variant,
            slats,
        }
    }

    pub fn to_hef(&self) -> String {
        let mut hef = String::new();

        writeln!(hef, "Hyperwood Exchange Format").unwrap();
        writeln!(hef, "Version 1").unwrap();
        writeln!(hef, "hyperwood.org").unwrap();

        writeln!(hef, "{:}", self.name).unwrap();
        writeln!(
            hef,
            "{:}",
            serde_json::to_string(&self.parameters).expect("could not serialize parameters")
        )
        .unwrap();
        writeln!(
            hef,
            "{:}",
            serde_json::to_string(&self.variant).expect("could not serialize variant")
        )
        .unwrap();
        writeln!(
            hef,
            "{:}",
            serde_json::to_string(&self.properties).expect("could not serialize properties")
        )
        .unwrap();

        let mut parts = HashSet::new();
        for slat in &self.slats {
            parts.insert(slat.name.to_owned());
        }

        writeln!(hef, "{:}", parts.len()).unwrap();

        let mut parts_indexes = HashMap::new();
        for (i, part) in parts.iter().enumerate() {
            parts_indexes.insert(part, i);
            writeln!(hef, "{:}", part).unwrap();
        }

        for slat in &self.slats {
            writeln!(
                hef,
                "{:} {:}",
                slat.to_hef_line(),
                parts_indexes.get(&slat.name).unwrap()
            )
            .unwrap();
        }

        hef
    }

    pub fn bom_lines(&self) -> Vec<String> {
        self.slats
            .iter()
            .map(|slat| slat.to_bom_line(&self.variant))
            .collect()
    }

    pub fn to_bom(&self) -> String {
        let mut bom = String::new();

        for slat in &self.slats {
            writeln!(bom, "{:}", slat.to_bom_line(&self.variant)).unwrap();
        }

        bom
    }

    pub fn length_total(&self) -> f32 {
        self.slats
            .iter()
            .map(|slat| slat.length(&self.variant))
            .sum()
    }
}
