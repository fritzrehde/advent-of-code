use derive_new::new;
use parse_display::FromStr;
use std::str;

#[derive(Debug)]
struct Boxes([Box; 256]);

impl Boxes {
    fn new() -> Self {
        // TODO: seems hacky, is there no std lib function for this?
        let boxes = [(); 256].map(|_| Box::default());
        Self(boxes)
    }

    fn total_focusing_power(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            // Box numbers start from 1.
            .map(|(i, v)| (i + 1, v))
            .flat_map(|(box_number, lens_box)| {
                lens_box
                    .0
                    .iter()
                    .enumerate()
                    // Slot numbers start from 1.
                    .map(|(i, v)| (i + 1, v))
                    .map(move |(slot_number, lens)| (box_number, slot_number, lens))
            })
            .map(|(box_number, slot_number, lens)| lens.focusing_power(box_number, slot_number))
            .sum()
    }
}

impl Boxes {
    /// Remove a lens according to its label.
    fn remove(&mut self, lens_label: LensLabel) {
        if let Some(lens_box) = self.0.get_mut(lens_label.box_index()) {
            if let Some(remove_idx) = lens_box.0.iter().position(|lens| lens.label == lens_label) {
                lens_box.0.remove(remove_idx);
            }
        }
    }

    /// Insert a new lens.
    fn insert(&mut self, lens: Lens) {
        if let Some(lens_box) = self.0.get_mut(lens.label.box_index()) {
            // Find an existing lens with the same label as new lens.
            let existing_lens = lens_box
                .0
                .iter_mut()
                .find(|existing| lens.label == existing.label);

            match existing_lens {
                // If there is already a lens in the box with the same label,
                // replace the old lens with the new lens.
                Some(old_lens) => *old_lens = lens,
                // If there is not already a lens in the box with the same
                // label, add the lens to the box immediately behind any
                // lenses already in the box.
                None => lens_box.0.push(lens),
            };
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Box(Vec<Lens>);

#[derive(Debug, new, Clone)]
struct Lens {
    label: LensLabel,
    focal_length: FocalLength,
}

impl Lens {
    /// Calculate the focusing power of this lens according to its box number
    /// and slot number inside that box.
    fn focusing_power(&self, box_number: usize, slot_number: usize) -> usize {
        box_number * slot_number * self.focal_length.0
    }
}

#[derive(Debug, FromStr, PartialEq, Eq, Clone)]
struct LensLabel(String);

impl LensLabel {
    /// The index of the box to place this lens in.
    fn box_index(&self) -> usize {
        self.hash()
    }

    fn hash(&self) -> usize {
        let mut current_value = 0;
        for c in self.0.chars() {
            let ascii_code = c as usize;
            current_value += ascii_code;
            current_value *= 17;
            current_value %= 256;
        }
        current_value
    }
}

#[derive(Debug, FromStr, Clone)]
struct FocalLength(usize);

#[derive(Debug)]
struct LensOperations(Vec<LensOperation>);

// TODO: parse_display should be able to automate this in the future
impl str::FromStr for LensOperations {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lens_operations = s
            .split(',')
            .map(str::parse::<LensOperation>)
            .collect::<Result<_, _>>()?;
        Ok(Self(lens_operations))
    }
}

#[derive(Debug, FromStr)]
enum LensOperation {
    #[display("{0}={1}")]
    Insert(LensLabel, FocalLength),

    #[display("{0}-")]
    Remove(LensLabel),
}

/// Solve the problem and return the solution as a `String`.
pub fn solve(puzzle_input: &str) -> anyhow::Result<String> {
    let mut boxes = Boxes::new();
    let lens_operations: LensOperations = puzzle_input.parse()?;

    for lens_operation in lens_operations.0.into_iter() {
        match lens_operation {
            LensOperation::Insert(lens_label, focal_length) => {
                boxes.insert(Lens::new(lens_label, focal_length))
            }
            LensOperation::Remove(lens_label) => boxes.remove(lens_label),
        }
    }

    let total_focusing_power = boxes.total_focusing_power();

    Ok(total_focusing_power.to_string())
}

#[cfg(test)]
pub mod example {
    use indoc::indoc;

    /// Provide the example details as `(puzzle input, expected solution)`.
    pub fn example_details() -> (&'static str, String) {
        let puzzle_input = indoc! {"
            rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7
        "};
        let expected_solution = 145;
        (puzzle_input, expected_solution.to_string())
    }
}
