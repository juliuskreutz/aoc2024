use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/05.txt")?;

    println!("Day05 Part1: {}", part1(&input)?);
    println!("Day05 Part2: {}", part2(&input)?);

    Ok(())
}

struct Parsed {
    rules: HashMap<usize, Vec<usize>>,
    updates: Vec<Vec<usize>>,
}

fn parse(input: &str) -> anyhow::Result<Parsed> {
    let mut split = input.split("\n\n");

    let rules_string = split.next().unwrap();
    let update_string = split.next().unwrap();

    let mut rules: HashMap<usize, Vec<_>> = HashMap::new();

    for rule_string in rules_string.lines() {
        let mut split = rule_string.split('|');

        let first: usize = split.next().unwrap().parse()?;
        let second: usize = split.next().unwrap().parse()?;

        rules.entry(second).or_default().push(first);
    }

    let mut updates = Vec::new();

    for update in update_string.lines() {
        let mut pages: Vec<usize> = Vec::new();

        for page in update.split(',') {
            pages.push(page.parse()?);
        }

        updates.push(pages);
    }

    Ok(Parsed { rules, updates })
}

fn check(pages: &[usize], rules: &HashMap<usize, Vec<usize>>) -> bool {
    let mut visited = HashSet::new();
    let mut wrong = HashSet::new();

    let rules = rules.clone();

    for &page in pages.iter() {
        if wrong.contains(&page) {
            return false;
        }

        visited.insert(page);

        if let Some(rules) = rules.get(&page) {
            for &rule in rules {
                if !visited.contains(&rule) {
                    wrong.insert(rule);
                }
            }
        }
    }

    true
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let Parsed { rules, updates } = parse(input)?;

    let mut sum = 0;

    for pages in updates {
        if check(&pages, &rules) {
            let middle = pages[pages.len() / 2];
            sum += middle;
        }
    }

    Ok(sum.to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let Parsed { rules, updates } = parse(input)?;

    let mut sum = 0;

    for pages in updates {
        if !check(&pages, &rules) {
            let mut dependencies: HashMap<_, Vec<_>> = HashMap::new();
            let mut degrees: HashMap<_, _> = pages.iter().map(|&p| (p, 0usize)).collect();

            for &page in &pages {
                if let Some(rules) = rules.get(&page) {
                    for &rule in rules {
                        if pages.contains(&rule) {
                            dependencies.entry(page).or_default().push(rule);
                            degrees.entry(rule).and_modify(|d| *d += 1);
                        }
                    }
                }
            }

            let mut q: VecDeque<_> = degrees
                .iter()
                .filter_map(|(&page, &degree)| if degree == 0 { Some(page) } else { None })
                .collect();

            let mut ordered = Vec::new();

            while let Some(page) = q.pop_back() {
                ordered.push(page);

                if let Some(dependencies) = dependencies.get(&page) {
                    for &dependency in dependencies {
                        if let Some(degree) = degrees.get_mut(&dependency) {
                            *degree -= 1;

                            if *degree == 0 {
                                q.push_back(dependency);
                            }
                        }
                    }
                }
            }

            let middle = ordered[ordered.len() / 2];
            sum += middle;
        }
    }

    Ok(sum.to_string())
}
