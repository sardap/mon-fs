use serde_derive::Deserialize;

fn main() {
    let mut pc = PC::new();

    println!(
        "PC size: {}:Bytes {}:KB",
        pc.bit_count() / 8,
        pc.bit_count() / 8 / 1024
    );

    pc.add_mon(BoxMon::default());

    return;

    let raw_json = fs::read_to_string("wild_encounters.json").expect("Could not read file");

    let wild_encounters: WildEncounters =
        serde_json::from_str(&raw_json).expect("Could not parse JSON");

    let mut scores = vec![];

    for group in wild_encounters.wild_encounter_groups {
        for encounter in group.encounters {
            if encounter.map.contains("MAP_SAFARI") || encounter.map.contains("MAP_ROUTE119") {
                continue;
            }

            let probs = encounter.get_probabilities();
            let score = probs.get_score();
            if score > 0. {
                scores.push((score, probs));
            }
        }
    }

    scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    for (score, probs) in scores {
        println!("Score: {:.2}", score);
        probs.print();
        println!();
    }

    println!("Hello, world!");
}

#[derive(Debug, Deserialize)]
struct Fields {
    #[serde(rename = "type")]
    field_type: String,
    encounter_rates: Vec<i32>,
}

#[derive(Debug, Deserialize)]
struct EncounterMon {
    min_level: i32,
    max_level: i32,
    species: String,
}

#[derive(Debug, Deserialize)]
struct EncounterMons {
    encounter_rate: i32,
    mons: Vec<EncounterMon>,
}

#[derive(Debug, Deserialize)]
struct Encounter {
    map: String,
    base_label: String,
    #[serde(default)]
    land_mons: Option<EncounterMons>,
}

impl Encounter {
    fn get_probabilities(&self) -> EncounterProbabilities {
        let mut result = vec![];
        if let Some(land_mons) = &self.land_mons {
            let mut mons = HashMap::new();
            for (i, mon) in land_mons.mons.iter().enumerate() {
                if !mons.contains_key(&mon.species) {
                    mons.insert(mon.species.clone(), 0);
                }

                let index_prob = match i {
                    0 => 20,
                    1 => 20,
                    2 => 10,
                    3 => 10,
                    4 => 10,
                    5 => 10,
                    6 => 5,
                    7 => 5,
                    8 => 4,
                    9 => 4,
                    10 => 1,
                    11 => 1,
                    _ => panic!("Too many mons in encounter"),
                };

                let current_prob = mons.get_mut(&mon.species).unwrap();
                *current_prob += index_prob;
            }

            for (species, prob) in mons {
                result.push(MonProbabilities {
                    species,
                    probability: prob as f32 / 100.,
                    absolute_probability: land_mons.encounter_rate as f32 / 100. * prob as f32
                        / 100.,
                });
            }
        }

        EncounterProbabilities {
            map: self.map.clone(),
            probabilities: result,
        }
    }
}

#[derive(Debug)]
struct MonProbabilities {
    species: String,
    probability: f32,
    absolute_probability: f32,
}

#[derive(Debug)]
struct EncounterProbabilities {
    map: String,
    probabilities: Vec<MonProbabilities>,
}

impl EncounterProbabilities {
    fn get_score(&self) -> f32 {
        let mut result = 0.;
        let mut over_threshold = 0;

        for mon in &self.probabilities {
            if mon.probability < 0.2 {
                continue;
            }
            over_threshold += 1;
            result += mon.absolute_probability;
        }

        if over_threshold < 3 {
            return 0.;
        }

        result * over_threshold as f32
    }

    fn print(&self) {
        println!("Map: {}", self.map);
        for mon in &self.probabilities {
            println!(
                "{}: {:.2} ({:.2})",
                mon.species, mon.probability, mon.absolute_probability
            );
        }
    }
}

#[derive(Debug, Deserialize)]
struct WildEncounterGroup {
    label: String,
    for_maps: bool,
    #[serde(default)]
    fields: Vec<Fields>,
    #[serde(default)]
    encounters: Vec<Encounter>,
}

#[derive(Debug, Deserialize)]
struct WildEncounters {
    wild_encounter_groups: Vec<WildEncounterGroup>,
}
