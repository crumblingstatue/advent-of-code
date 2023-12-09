#[cfg(test)]
const TEST_INPUT: &str = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(Debug)]
struct MapRange {
    dest_start: u64,
    source_start: u64,
    len: u64,
}

impl MapRange {
    fn from_str(input: &str) -> Self {
        let mut split = input.split_whitespace();
        let [dest_start, source_start, len] =
            std::array::from_fn(|_| split.next().unwrap().parse().unwrap());
        Self {
            dest_start,
            source_start,
            len,
        }
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<MapRange>,
    soil_to_fert: Vec<MapRange>,
    fert_to_watr: Vec<MapRange>,
    watr_to_ligt: Vec<MapRange>,
    ligt_to_temp: Vec<MapRange>,
    temp_to_humi: Vec<MapRange>,
    humi_to_loca: Vec<MapRange>,
}

#[derive(Default, Clone, Copy)]
enum ParseStatus {
    #[default]
    Init,
    SeedToSoil,
    SoilToFert,
    FertToWatr,
    WatrToLigt,
    LigtToTemp,
    TempToHumi,
    HumiToLoca,
}

impl Almanac {
    fn from_str(input: &str) -> Self {
        let mut status = ParseStatus::default();
        let mut seeds = vec![];
        let mut seed_to_soil = vec![];
        let mut soil_to_fert = vec![];
        let mut fert_to_watr = vec![];
        let mut watr_to_ligt = vec![];
        let mut ligt_to_temp = vec![];
        let mut temp_to_humi = vec![];
        let mut humi_to_loca = vec![];
        for line in input.lines() {
            if line.is_empty() {
                status = ParseStatus::Init;
                continue;
            }
            match status {
                ParseStatus::Init => {
                    let (name, contents) = line.split_once(':').unwrap();
                    if name == "seeds" {
                        seeds.extend(
                            contents
                                .split_whitespace()
                                .map(|s| s.parse::<u64>().unwrap()),
                        );
                    } else {
                        let map_name = name.strip_suffix(" map").unwrap();
                        status = match map_name {
                            "seed-to-soil" => ParseStatus::SeedToSoil,
                            "soil-to-fertilizer" => ParseStatus::SoilToFert,
                            "fertilizer-to-water" => ParseStatus::FertToWatr,
                            "water-to-light" => ParseStatus::WatrToLigt,
                            "light-to-temperature" => ParseStatus::LigtToTemp,
                            "temperature-to-humidity" => ParseStatus::TempToHumi,
                            "humidity-to-location" => ParseStatus::HumiToLoca,
                            _ => panic!("Unknown map: {map_name}"),
                        };
                    }
                }
                rest => {
                    let v = match rest {
                        ParseStatus::SeedToSoil => &mut seed_to_soil,
                        ParseStatus::SoilToFert => &mut soil_to_fert,
                        ParseStatus::FertToWatr => &mut fert_to_watr,
                        ParseStatus::WatrToLigt => &mut watr_to_ligt,
                        ParseStatus::LigtToTemp => &mut ligt_to_temp,
                        ParseStatus::TempToHumi => &mut temp_to_humi,
                        ParseStatus::HumiToLoca => &mut humi_to_loca,
                        ParseStatus::Init => unreachable!(),
                    };
                    v.push(MapRange::from_str(line));
                }
            }
        }
        Self {
            seeds,
            seed_to_soil,
            soil_to_fert,
            fert_to_watr,
            watr_to_ligt,
            ligt_to_temp,
            temp_to_humi,
            humi_to_loca,
        }
    }
    fn lowest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|s| self.resolve_seed_map(*s))
            .min()
            .unwrap()
    }
    fn resolve_seed_map(&self, seed: u64) -> u64 {
        let soil = map(seed, &self.seed_to_soil);
        let fertilizer = map(soil, &self.soil_to_fert);
        let water = map(fertilizer, &self.fert_to_watr);
        let light = map(water, &self.watr_to_ligt);
        let temp = map(light, &self.ligt_to_temp);
        let humi = map(temp, &self.temp_to_humi);
        map(humi, &self.humi_to_loca)
    }
}

fn map(value: u64, ranges: &[MapRange]) -> u64 {
    ranges
        .iter()
        .find_map(|range| value_range_map(range, value))
        .unwrap_or(value)
}

fn value_range_map(range: &MapRange, value: u64) -> Option<u64> {
    (value >= range.source_start && value < range.source_start + range.len).then(|| {
        let relative_val = value - range.source_start;
        range.dest_start + relative_val
    })
}

fn part1(input: &str) -> u64 {
    Almanac::from_str(input).lowest_location()
}

aoc::tests! {
fn part1:
    TEST_INPUT => 35;
    in => 175622908;
}

aoc::main!(part1);
