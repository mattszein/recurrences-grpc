use rrule::{RRuleResult, RRuleSet};

fn parse_rrule(rrule: &str) -> RRuleResult {
    let rule: RRuleSet = rrule.parse().unwrap();
    rule.all(100)
}

fn main() {
    let result = parse_rrule("DTSTART:20120201T093000Z\nRRULE:FREQ=DAILY;COUNT=3");
    println!("Result is {:?}", &result)
}
