use regex;
use std::fs;

static YES: i32 = 1;
static NO: i32 = 0;
static QUESTIONS: &str = "abcdefghijklmnopqrstuvwxyz";
static QUESTION_COUNT: usize = 26;

type Group = Vec<String>;

fn main() {
  println!("EXAMPLE\n----------");
  let input = load_input("06-example.txt");
  part1(&input);
  part2(&input);

  println!("\nINPUT\n----------");
  let input = load_input("06.txt");
  part1(&input);
  part2(&input);
}

fn part1(groups: &Vec<Group>) {
  let all_groups_yeses: i32 = groups //
    .iter()
    .map(|group| count_yeses(group))
    .sum();
  println!("PT.1: {}", all_groups_yeses);
}

fn part2(groups: &Vec<Group>) {
  let all_groups_yeses: i32 = groups //
    .iter()
    .map(|group| count_only_yeses(group))
    .sum();
  println!("PT.2: {}", all_groups_yeses);
}

fn count_yeses(group: &Group) -> i32 {
  let mut group_answers = vec![NO; QUESTION_COUNT];
  group.iter().for_each(|form| {
    form.chars().for_each(|yesed_question| {
      let yesed_index = (yesed_question as usize) - ('a' as usize);
      group_answers[yesed_index] = YES;
    });
  });
  return group_answers.iter().sum();
}

fn count_only_yeses(group: &Group) -> i32 {
  let mut group_answers = vec![YES; QUESTION_COUNT];
  group.iter().for_each(|form| {
    QUESTIONS.chars().enumerate().for_each(|(index, question)| {
      if !form.contains(question) {
        group_answers[index] = NO;
      }
    });
  });
  return group_answers.iter().sum::<i32>();
}

fn load_input(file_name: &str) -> Vec<Group> {
  let group_separator = regex::Regex::new("\r\n\r\n").unwrap();

  let input_str = fs::read_to_string(file_name).unwrap();
  let groups = group_separator
    .split(&input_str)
    .map(|group_str| parse_group(group_str))
    .collect::<Vec<Group>>();
  return groups;
}

fn parse_group(group_str: &str) -> Group {
  return group_str
    .to_string()
    .lines()
    .map(|line| line.to_string())
    .collect::<Group>();
}
